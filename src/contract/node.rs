use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

use amplify::{AsAny, Wrapper};
use bitcoin::hashes::{sha256, sha256t, Hash};
use bitcoin::{OutPoint, Txid};
use bp::seals::txout::TxoSeal;
use commit_verify::lnpbp4::ProtocolId;
use commit_verify::{
    commit_encode, CommitEncode, CommitVerify, ConsensusCommit, PrehashedProtocol, TaggedHash,
    ToMerkleSource,
};
use lnpbp::bech32::{FromBech32Str, ToBech32String};
use lnpbp::chain::Chain;
use once_cell::sync::Lazy;

use super::{
    ConcealSeals, ConcealState, OwnedRights, OwnedRightsInner, ParentOwnedRights,
    ParentPublicRights, PublicRights, PublicRightsInner, TypedAssignments,
};
use crate::reveal::{self, MergeReveal};
use crate::schema::{
    ExtensionType, FieldType, NodeSubtype, NodeType, OwnedRightType, TransitionType,
};
use crate::{schema, seal, ConfidentialDataError, Metadata, PublicRightType, SchemaId};

static EMPTY_OWNED_RIGHTS: Lazy<ParentOwnedRights> = Lazy::new(ParentOwnedRights::default);
static EMPTY_PUBLIC_RIGHTS: Lazy<ParentPublicRights> = Lazy::new(ParentPublicRights::default);

/// Midstate for a tagged hash engine. Equals to a single SHA256 hash of
/// the value of two concatenated SHA256 hashes for `rgb:node` prefix string.
static MIDSTATE_NODE_ID: [u8; 32] = [
    0x90, 0xd0, 0xc4, 0x9b, 0xa6, 0xb8, 0xa, 0x5b, 0xbc, 0xba, 0x19, 0x9, 0xdc, 0xbd, 0x5a, 0x58,
    0x55, 0x6a, 0xe2, 0x16, 0xa5, 0xee, 0xb7, 0x3c, 0x1, 0xe0, 0x86, 0x91, 0x22, 0x43, 0x12, 0x9f,
];

pub const RGB_CONTRACT_ID_HRP: &str = "rgb";

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictEncode, StrictDecode)]
#[display("{node_id}/{ty}/{no}")]
/// RGB contract node output pointer, defined by the node ID and output
/// number.
pub struct NodeOutpoint {
    pub node_id: NodeId,
    pub ty: OwnedRightType,
    pub no: u16,
}

#[derive(Clone, Eq, PartialEq, Debug, Display, Error, From)]
#[display(inner)]
pub enum OutpointParseError {
    #[from]
    InvalidNodeId(bitcoin::hashes::hex::Error),

    InvalidType(ParseIntError),

    InvalidOutputNo(ParseIntError),

    /// invalid node outpoint format ('{0}')
    #[display(doc_comments)]
    WrongFormat(String),
}

impl FromStr for NodeOutpoint {
    type Err = OutpointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');
        match (split.next(), split.next(), split.next(), split.next()) {
            (Some(node_id), Some(ty), Some(no), None) => Ok(NodeOutpoint {
                node_id: node_id.parse()?,
                ty: ty.parse().map_err(OutpointParseError::InvalidType)?,
                no: no.parse().map_err(OutpointParseError::InvalidOutputNo)?,
            }),
            _ => Err(OutpointParseError::WrongFormat(s.to_owned())),
        }
    }
}

/// Tag used for [`NodeId`] and [`ContractId`] hash types
pub struct NodeIdTag;

impl sha256t::Tag for NodeIdTag {
    #[inline]
    fn engine() -> sha256::HashEngine {
        let midstate = sha256::Midstate::from_inner(MIDSTATE_NODE_ID);
        sha256::HashEngine::from_midstate(midstate, 64)
    }
}

impl lnpbp::bech32::Strategy for NodeIdTag {
    const HRP: &'static str = RGB_CONTRACT_ID_HRP;
    type Strategy = lnpbp::bech32::strategies::UsingStrictEncoding;
}

impl NodeOutpoint {
    pub fn new(node_id: NodeId, ty: u16, no: u16) -> NodeOutpoint {
        NodeOutpoint { node_id, ty, no }
    }
}

/// Unique node (genesis, extensions & state transition) identifier equivalent
/// to the commitment hash
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
#[derive(Wrapper, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
#[wrapper(Debug, Display, BorrowSlice)]
pub struct NodeId(sha256t::Hash<NodeIdTag>);

impl<Msg> CommitVerify<Msg, PrehashedProtocol> for NodeId
where Msg: AsRef<[u8]>
{
    #[inline]
    fn commit(msg: &Msg) -> NodeId { NodeId::hash(msg) }
}

impl commit_encode::Strategy for NodeId {
    type Strategy = commit_encode::strategies::UsingStrict;
}

impl FromStr for NodeId {
    type Err = bitcoin::hashes::hex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(NodeId::from_inner(s.parse()?)) }
}

/// Unique contract identifier equivalent to the contract genesis commitment
#[derive(Wrapper, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Display, From)]
#[wrapper(Debug, BorrowSlice)]
#[display(ContractId::to_bech32_string)]
pub struct ContractId(sha256t::Hash<NodeIdTag>);

impl From<ContractId> for ProtocolId {
    fn from(contract_id: ContractId) -> Self {
        ProtocolId::from_inner(contract_id.into_inner().into_inner())
    }
}

impl From<ProtocolId> for ContractId {
    fn from(protocol_id: ProtocolId) -> Self {
        ContractId::from_inner(<ContractId as Wrapper>::Inner::from_inner(
            protocol_id.into_inner(),
        ))
    }
}

impl From<ContractId> for lnpbp::chain::AssetId {
    fn from(id: ContractId) -> Self { Self::from_inner(id.into_inner().into_inner()) }
}

impl commit_encode::Strategy for ContractId {
    type Strategy = commit_encode::strategies::UsingStrict;
}

impl lnpbp::bech32::Strategy for ContractId {
    const HRP: &'static str = RGB_CONTRACT_ID_HRP;
    type Strategy = lnpbp::bech32::strategies::UsingStrictEncoding;
}

// Make this part of `lnpbp::bech32`
#[cfg(feature = "serde")]
impl serde::Serialize for ContractId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_bech32_string())
        } else {
            serializer.serialize_bytes(&self[..])
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ContractId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = ContractId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "Bech32 string with `{}` HRP",
                    RGB_CONTRACT_ID_HRP
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where E: serde::de::Error {
                ContractId::from_str(v).map_err(serde::de::Error::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where E: serde::de::Error {
                self.visit_str(&v)
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where E: serde::de::Error {
                ContractId::from_bytes(&v)
                    .map_err(|_| serde::de::Error::invalid_length(v.len(), &"32 bytes"))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(Visitor)
        } else {
            deserializer.deserialize_byte_buf(Visitor)
        }
    }
}

impl FromStr for ContractId {
    type Err = lnpbp::bech32::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { ContractId::from_bech32_str(s) }
}

/// RGB contract node API, defined as trait
///
/// Implemented by all contract node types (see [`NodeType`]):
/// - Genesis ([`Genesis`])
/// - State transitions ([`Transitions`])
/// - Public state extensions ([`Extensions`])
pub trait Node: AsAny {
    /// Returns type of the node (see [`NodeType`]). Unfortunately, this can't
    /// be just a const, since it will break our ability to convert concrete
    /// `Node` types into `&dyn Node` (entities implementing traits with const
    /// definitions can't be made into objects)
    fn node_type(&self) -> NodeType;

    /// Returns full contract node type information
    fn subtype(&self) -> NodeSubtype;

    /// Returns [`NodeId`], which is a hash of this node commitment
    /// serialization
    fn node_id(&self) -> NodeId;

    /// Returns [`Option::Some`]`(`[`ContractId`]`)`, which is a hash of
    /// genesis.
    /// - For genesis node, this hash is byte-equal to [`NodeId`] (however
    ///   displayed in a reverse manner, to introduce semantical distinction)
    /// - For extension node function returns id of the genesis, to which this
    ///   node commits to
    /// - For state transition function returns [`Option::None`], since they do
    ///   not keep this information; it must be deduced through state transition
    ///   graph
    fn contract_id(&self) -> Option<ContractId>;

    /// Returns [`Option::Some`]`(`[`TransitionType`]`)` for transitions or
    /// [`Option::None`] for genesis and extension node types
    fn transition_type(&self) -> Option<TransitionType>;

    /// Returns [`Option::Some`]`(`[`ExtensionType`]`)` for extension nodes or
    /// [`Option::None`] for genesis and trate transitions
    fn extension_type(&self) -> Option<ExtensionType>;

    /// Returns reference to a full set of metadata (in form of [`Metadata`]
    /// wrapper structure) for the contract node.
    fn metadata(&self) -> &Metadata;

    /// Returns reference to information about the owned rights in form of
    /// [`ParentOwnedRights`] wrapper structure which this node updates with
    /// state transition ("parent owned rights").
    ///
    /// This is always an empty `Vec` for [`Genesis`] and [`Extension`] node
    /// types.
    fn parent_owned_rights(&self) -> &ParentOwnedRights;

    /// Returns reference to information about the public rights (in form of
    /// [`ParentPublicRights`] wrapper structure), defined with "parent" state
    /// extensions (i.e. those finalized with the current state transition) or
    /// referenced by another state extension, which this node updates
    /// ("parent public rights").
    ///
    /// This is always an empty `Vec` for [`Genesis`].
    fn parent_public_rights(&self) -> &ParentPublicRights;
    fn owned_rights(&self) -> &OwnedRights;
    fn owned_rights_mut(&mut self) -> &mut OwnedRights;
    fn public_rights(&self) -> &PublicRights;
    fn public_rights_mut(&mut self) -> &mut PublicRights;

    #[inline]
    fn field_types(&self) -> Vec<FieldType> { self.metadata().keys().copied().collect() }

    #[inline]
    fn parent_public_right_types(&self) -> Vec<PublicRightType> {
        self.parent_public_rights()
            .values()
            .flat_map(BTreeSet::iter)
            .copied()
            .collect()
    }

    #[inline]
    fn parent_by_public_right_type(&self, t: PublicRightType) -> Vec<NodeId> {
        self.parent_public_rights()
            .iter()
            .filter(|(_, t2)| t2.contains(&t))
            .map(|(node_id, _)| *node_id)
            .collect()
    }

    /// For genesis and public state extensions always returns an empty list.
    /// While public state extension do have parent nodes, they do not contain
    /// indexed rights.
    #[inline]
    fn parent_outputs(&self) -> Vec<NodeOutpoint> {
        self.parent_owned_rights()
            .iter()
            .flat_map(|(node_id, map)| {
                let node_id = *node_id;
                map.iter()
                    .flat_map(|(ty, vec)| vec.iter().map(|no| (*ty, *no)))
                    .map(move |(ty, no)| NodeOutpoint { node_id, ty, no })
            })
            .collect()
    }

    #[inline]
    fn parent_outputs_by_type(&self, t: OwnedRightType) -> Vec<NodeOutpoint> {
        self.parent_outputs_by_types(&[t])
    }

    fn parent_outputs_by_types(&self, types: &[OwnedRightType]) -> Vec<NodeOutpoint> {
        self.parent_owned_rights()
            .iter()
            .flat_map(|(node_id, map)| {
                let node_id = *node_id;
                map.iter()
                    .filter(|(t, _)| types.contains(*t))
                    .flat_map(|(ty, vec)| vec.iter().map(|no| (*ty, *no)))
                    .map(move |(ty, no)| NodeOutpoint { node_id, ty, no })
            })
            .collect()
    }

    #[inline]
    fn parent_owned_right_types(&self) -> Vec<OwnedRightType> {
        self.parent_owned_rights()
            .values()
            .flat_map(BTreeMap::keys)
            .copied()
            .collect()
    }

    #[inline]
    fn owned_right_types(&self) -> BTreeSet<OwnedRightType> {
        self.owned_rights().keys().cloned().collect()
    }

    #[inline]
    fn owned_rights_by_type(&self, t: OwnedRightType) -> Option<&TypedAssignments> {
        self.owned_rights()
            .iter()
            .find_map(|(t2, a)| if *t2 == t { Some(a) } else { None })
    }

    #[inline]
    fn to_confiential_seals(&self) -> Vec<seal::Confidential> {
        self.owned_rights()
            .iter()
            .flat_map(|(_, assignment)| assignment.to_confidential_seals())
            .collect()
    }

    #[inline]
    fn revealed_seals(&self) -> Result<Vec<seal::Revealed>, ConfidentialDataError> {
        let unfiltered = self
            .owned_rights()
            .iter()
            .map(|(_, assignment)| assignment.revealed_seals())
            .collect::<Vec<_>>();
        if unfiltered.contains(&Err(ConfidentialDataError)) {
            return Err(ConfidentialDataError);
        }
        Ok(unfiltered
            .into_iter()
            .filter_map(Result::ok)
            .flat_map(Vec::into_iter)
            .collect())
    }

    #[inline]
    fn revealed_seals_by_type(
        &self,
        assignment_type: OwnedRightType,
    ) -> Result<Vec<seal::Revealed>, ConfidentialDataError> {
        Ok(self
            .owned_rights_by_type(assignment_type)
            .map(TypedAssignments::revealed_seals)
            .transpose()?
            .unwrap_or_default())
    }

    #[inline]
    fn filter_revealed_seals(&self) -> Vec<seal::Revealed> {
        self.owned_rights()
            .iter()
            .flat_map(|(_, assignment)| assignment.filter_revealed_seals())
            .collect()
    }

    #[inline]
    fn filter_revealed_seals_by_type(
        &self,
        assignment_type: OwnedRightType,
    ) -> Vec<seal::Revealed> {
        self.owned_rights_by_type(assignment_type)
            .map(TypedAssignments::filter_revealed_seals)
            .unwrap_or_else(Vec::new)
    }

    fn node_outputs(&self, witness_txid: Txid) -> BTreeMap<NodeOutpoint, OutPoint> {
        let node_id = self.node_id();
        let mut res: BTreeMap<NodeOutpoint, OutPoint> = bmap! {};
        for (ty, assignments) in self.owned_rights() {
            for (seal, node_output) in assignments.revealed_seal_outputs() {
                let outpoint = seal.outpoint_or(witness_txid);
                let node_outpoint = NodeOutpoint::new(node_id, *ty, node_output);
                res.insert(node_outpoint, outpoint);
            }
        }
        res
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, AsAny)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Genesis {
    schema_id: SchemaId,
    chain: Chain,
    metadata: Metadata,
    owned_rights: OwnedRights,
    public_rights: PublicRights,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, AsAny)]
#[derive(StrictEncode, StrictDecode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Extension {
    extension_type: ExtensionType,
    contract_id: ContractId,
    metadata: Metadata,
    owned_rights: OwnedRights,
    parent_public_rights: ParentPublicRights,
    public_rights: PublicRights,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, AsAny)]
#[derive(StrictEncode, StrictDecode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct Transition {
    transition_type: TransitionType,
    metadata: Metadata,
    parent_owned_rights: ParentOwnedRights,
    owned_rights: OwnedRights,
    parent_public_rights: ParentPublicRights,
    public_rights: PublicRights,
}

impl ConsensusCommit for Genesis {
    type Commitment = NodeId;
}

impl ConsensusCommit for Extension {
    type Commitment = NodeId;
}

impl ConsensusCommit for Transition {
    type Commitment = NodeId;
}

impl CommitEncode for Extension {
    fn commit_encode<E: io::Write>(&self, mut e: E) -> usize {
        let mut len = self.extension_type.commit_encode(&mut e);
        len += self.contract_id.commit_encode(&mut e);
        len += self
            .metadata
            .to_merkle_source()
            .consensus_commit()
            .commit_encode(&mut e);
        len += self
            .owned_rights
            .to_merkle_source()
            .consensus_commit()
            .commit_encode(&mut e);
        len += self
            .parent_public_rights
            .to_merkle_source()
            .consensus_commit()
            .commit_encode(&mut e);
        len += self
            .public_rights
            .to_merkle_source()
            .consensus_commit()
            .commit_encode(&mut e);
        len
    }
}

impl CommitEncode for Transition {
    fn commit_encode<E: io::Write>(&self, mut e: E) -> usize {
        let mut len = self.transition_type.commit_encode(&mut e);
        len += self
            .metadata
            .to_merkle_source()
            .consensus_commit()
            .commit_encode(&mut e);
        len += self
            .parent_owned_rights
            .to_merkle_source()
            .consensus_commit()
            .commit_encode(&mut e);
        len += self
            .owned_rights
            .to_merkle_source()
            .consensus_commit()
            .commit_encode(&mut e);
        len += self
            .parent_public_rights
            .to_merkle_source()
            .consensus_commit()
            .commit_encode(&mut e);
        len += self
            .public_rights
            .to_merkle_source()
            .consensus_commit()
            .commit_encode(&mut e);
        len
    }
}

impl ConcealState for Genesis {
    fn conceal_state_except(&mut self, seals: &[seal::Confidential]) -> usize {
        let mut count = 0;
        for (_, assignment) in self.owned_rights_mut().iter_mut() {
            count += assignment.conceal_state_except(seals);
        }
        count
    }
}

impl ConcealState for Extension {
    fn conceal_state_except(&mut self, seals: &[seal::Confidential]) -> usize {
        let mut count = 0;
        for (_, assignment) in self.owned_rights_mut().iter_mut() {
            count += assignment.conceal_state_except(seals);
        }
        count
    }
}

impl ConcealState for Transition {
    fn conceal_state_except(&mut self, seals: &[seal::Confidential]) -> usize {
        let mut count = 0;
        for (_, assignment) in self.owned_rights_mut().iter_mut() {
            count += assignment.conceal_state_except(seals);
        }
        count
    }
}

impl ConcealSeals for Transition {
    fn conceal_seals(&mut self, seals: &[seal::Confidential]) -> usize {
        let mut count = 0;
        for (_, assignment) in self.owned_rights_mut().iter_mut() {
            count += assignment.conceal_seals(seals);
        }
        count
    }
}

impl MergeReveal for Genesis {
    fn merge_reveal(mut self, other: Self) -> Result<Self, reveal::Error> {
        if self.consensus_commit() != other.consensus_commit() {
            return Err(reveal::Error::NodeMismatch(NodeType::Genesis));
        }
        self.owned_rights = self.owned_rights.merge_reveal(other.owned_rights)?;
        Ok(self)
    }
}

impl MergeReveal for Transition {
    fn merge_reveal(mut self, other: Self) -> Result<Self, reveal::Error> {
        if self.consensus_commit() != other.consensus_commit() {
            return Err(reveal::Error::NodeMismatch(NodeType::StateTransition));
        }
        self.owned_rights = self.owned_rights.merge_reveal(other.owned_rights)?;
        Ok(self)
    }
}

impl MergeReveal for Extension {
    fn merge_reveal(mut self, other: Self) -> Result<Self, reveal::Error> {
        if self.consensus_commit() != other.consensus_commit() {
            return Err(reveal::Error::NodeMismatch(NodeType::StateExtension));
        }
        self.owned_rights = self.owned_rights.merge_reveal(other.owned_rights)?;
        Ok(self)
    }
}

impl Node for Genesis {
    #[inline]
    fn node_type(&self) -> NodeType { NodeType::Genesis }

    #[inline]
    fn subtype(&self) -> NodeSubtype { NodeSubtype::Genesis }

    #[inline]
    fn node_id(&self) -> NodeId { self.clone().consensus_commit() }

    #[inline]
    fn contract_id(&self) -> Option<ContractId> {
        Some(ContractId::from_inner(self.node_id().into_inner()))
    }

    #[inline]
    fn transition_type(&self) -> Option<TransitionType> { None }

    #[inline]
    fn extension_type(&self) -> Option<ExtensionType> { None }

    #[inline]
    fn parent_owned_rights(&self) -> &ParentOwnedRights { &EMPTY_OWNED_RIGHTS }

    #[inline]
    fn parent_public_rights(&self) -> &ParentPublicRights { &EMPTY_PUBLIC_RIGHTS }

    #[inline]
    fn metadata(&self) -> &Metadata { &self.metadata }

    #[inline]
    fn owned_rights(&self) -> &OwnedRights { &self.owned_rights }

    #[inline]
    fn owned_rights_mut(&mut self) -> &mut OwnedRights { &mut self.owned_rights }

    #[inline]
    fn public_rights(&self) -> &PublicRights { &self.public_rights }

    #[inline]
    fn public_rights_mut(&mut self) -> &mut PublicRights { &mut self.public_rights }
}

impl Node for Extension {
    #[inline]
    fn node_type(&self) -> NodeType { NodeType::StateExtension }

    #[inline]
    fn subtype(&self) -> NodeSubtype { NodeSubtype::StateExtension(self.extension_type) }

    #[inline]
    fn node_id(&self) -> NodeId { self.clone().consensus_commit() }

    #[inline]
    fn contract_id(&self) -> Option<ContractId> { Some(self.contract_id) }

    #[inline]
    fn transition_type(&self) -> Option<TransitionType> { None }

    #[inline]
    fn extension_type(&self) -> Option<ExtensionType> { Some(self.extension_type) }

    #[inline]
    fn parent_owned_rights(&self) -> &ParentOwnedRights { &EMPTY_OWNED_RIGHTS }

    #[inline]
    fn parent_public_rights(&self) -> &ParentPublicRights { &self.parent_public_rights }

    #[inline]
    fn metadata(&self) -> &Metadata { &self.metadata }

    #[inline]
    fn owned_rights(&self) -> &OwnedRights { &self.owned_rights }

    #[inline]
    fn owned_rights_mut(&mut self) -> &mut OwnedRights { &mut self.owned_rights }

    #[inline]
    fn public_rights(&self) -> &PublicRights { &self.public_rights }

    #[inline]
    fn public_rights_mut(&mut self) -> &mut PublicRights { &mut self.public_rights }
}

impl Node for Transition {
    #[inline]
    fn node_type(&self) -> NodeType { NodeType::StateTransition }

    #[inline]
    fn subtype(&self) -> NodeSubtype { NodeSubtype::StateTransition(self.transition_type) }
