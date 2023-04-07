use rgb-core::{Contract}
use rgb-lightning-sample::{PaymentInfo, TypeAssignments , PeerManager}

impl TypedAssignments {
    pub fn zero_balanced_static(
        inputs: Vec<value::Revealed>,
        allocations_ours: BTreeMap<seal::Revealed, AtomicValue>,
        allocations_theirs: BTreeMap<SealEndpoint, AtomicValue>,
    ) -> Self {
        if allocations_ours.len() + allocations_theirs.len() == 0 {
            return Self::Value(vec![]);
        }

        let count = allocations_theirs.len() + allocations_ours.len();
        let secret_key = SecretKey([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 0,
        ]);
        let mut blinding_factors = vec![secret_key; count];

        let mut blinding_inputs: Vec<_> = inputs.iter().map(|inp| inp.blinding.into()).collect();
        if blinding_inputs.is_empty() {
            blinding_inputs.push(secp256k1zkp::key::ONE_KEY);
        }

        if !blinding_factors.is_empty() {
            blinding_factors.pop();
            let blinding_correction = SECP256K1_ZKP
                .blind_sum(blinding_inputs.clone(), blinding_factors.clone())
                .expect("SECP256K1_ZKP failure has negligible probability");
            blinding_factors.push(blinding_correction);
        }

        let mut blinding_iter = blinding_factors.into_iter();

        let mut set: Vec<Assignment<_>> = allocations_ours
            .into_iter()
            .map(|(seal, amount)| Assignment::Revealed {
                seal,
                state: value::Revealed {
                    value: amount,
                    blinding: blinding_iter
                        .next()
                        .expect("Internal inconsistency in `AssignmentsVariant::zero_balanced`")
                        .into(),
                },
            })
            .collect();
        set.extend(allocations_theirs.into_iter().map(|(seal_proto, amount)| {
            let state = value::Revealed {
                value: amount,
                blinding: blinding_iter
                    .next()
                    .expect("Internal inconsistency in `AssignmentsVariant::zero_balanced`")
                    .into(),
            };
            match seal_proto {
                SealEndpoint::ConcealedUtxo(seal) => Assignment::ConfidentialSeal { seal, state },
                SealEndpoint::WitnessVout {
                    method,
                    vout,
                    blinding,
                } => Assignment::Revealed {
                    seal: seal::Revealed {
                        method,
                        txid: None,
                        vout,
                        blinding,
                    },
                    state,
                },
            }
        }));

        Self::Value(set)
    }
