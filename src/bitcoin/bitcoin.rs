use crate::bitcoin;

use bp_core::opret;
use bp_core::spk;
use bp_core::tx;
use bp_core::txout;
use bp_core::anchor;
use bp_core::proof;
use bpstd::tapret;
use bpstd::psbt;
use bp_core::mbc;
use bpstd::mpc;
use bp_core::psbt::maps;
use bp_core::psbt::keys;

impl anchor::Anchor {
    pub fn new(
        anchor_id: anchor::Id,
        anchor_tx: tx::Tx,
        anchor_tx_out: txout::TxOut,
        anchor_proof: proof::Proof,
        anchor_spk: spk::Spk,
        anchor_opret: opret::Opret,
        anchor_tapret: tapret::Tapret,
        anchor_mbc: mbc::Mbc,
        anchor_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            anchor_id,
            anchor_tx,
            anchor_tx_out,
            anchor_proof,
            anchor_spk,
            anchor_opret,
            anchor_tapret,
            anchor_mbc,
            anchor_mpc,
        }
    }
}
impl keys::Anchor {
    pub fn new(
        anchor_id: anchor::Id,
        anchor_tx: tx::Tx,
        anchor_tx_out: txout::TxOut,
        anchor_proof: proof::Proof,
        anchor_spk: spk::Spk,
        anchor_opret: opret::Opret,
        anchor_tapret: tapret::Tapret,
        anchor_mbc: mbc::Mbc,
        anchor_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            anchor_id,
            anchor_tx,
            anchor_tx_out,
            anchor_proof,
            anchor_spk,
            anchor_opret,
            anchor_tapret,
            anchor_mbc,
            anchor_mpc,
        }
    }
impl psbt::Anchor {
    pub fn new(
        anchor_id: anchor::Id,
        anchor_tx: tx::Tx,
        anchor_tx_out: txout::TxOut,
        anchor_proof: proof::Proof,
        anchor_spk: spk::Spk,
        anchor_opret: opret::Opret,
        anchor_tapret: tapret::Tapret,
        anchor_mbc: mbc::Mbc,
        anchor_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            anchor_id,
            anchor_tx,
            anchor_tx_out,
            anchor_proof,
            anchor_spk,
            anchor_opret,
            anchor_tapret,
            anchor_mbc,
            anchor_mpc,
        }
    }
}
impl psbt::Psbt {
    pub fn new(
        psbt_id: psbt::Id,
        psbt_tx: tx::Tx,
        psbt_tx_out: txout::TxOut,
        psbt_proof: proof::Proof,
        psbt_spk: spk::Spk,
        psbt_opret: opret::Opret,
        psbt_tapret: tapret::Tapret,
        psbt_mbc: mbc::Mbc,
        psbt_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            psbt_id,
            psbt_tx,
            psbt_tx_out,
            psbt_proof,
            psbt_spk,
            psbt_opret,
            psbt_tapret,
            psbt_mbc,
            psbt_mpc,
        }
    }
}
}
impl tx::Tx {
    pub fn new(
        tx_id: tx::Id,
        tx_tx: tx::Tx,
        tx_tx_out: txout::TxOut,
        tx_proof: proof::Proof,
        tx_spk: spk::Spk,
        tx_opret: opret::Opret,
        tx_tapret: tapret::Tapret,
        tx_mbc: mbc::Mbc,
        tx_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            tx_id,
            tx_tx,
            tx_tx_out,
            tx_proof,
            tx_spk,
            tx_opret,
            tx_tapret,
            tx_mbc,
            tx_mpc,
        }
    }
}
impl maps::Map {
    pub fn new(
        map_id: maps::Id,
        map_map: maps::Map,
        map_tx_out: txout::TxOut,
        map_proof: proof::Proof,
        map_spk: spk::Spk,
        map_opret: opret::Opret,
        map_tapret: tapret::Tapret,
        map_mbc: mbc::Mbc,
        map_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            map_id,
            map_map,
            map_tx_out,
            map_proof,
            map_spk,
            map_opret,
            map_tapret,
            map_mbc,
            map_mpc,
        }
    }
}
impl txout::TxOut {
    pub fn new(
        tx_out_id: txout::Id,
        tx_out_tx_out: txout::TxOut,
        tx_out_proof: proof::Proof,
        tx_out_spk: spk::Spk,
        tx_out_opret: opret::Opret,
        tx_out_tapret: tapret::Tapret,
        tx_out_mbc: mbc::Mbc,
        tx_out_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            tx_out_id,
            tx_out_tx_out,
            tx_out_proof,
            tx_out_spk,
            tx_out_opret,
            tx_out_tapret,
            tx_out_mbc,
            tx_out_mpc,
        }
    }
}
impl proof::Proof {
    pub fn new(
        proof_id: proof::Id,
        proof_proof: proof::Proof,
        proof_spk: spk::Spk,
        proof_opret: opret::Opret,
        proof_tapret: tapret::Tapret,
        proof_mbc: mbc::Mbc,
        proof_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            proof_id,
            proof_proof,
            proof_spk,
            proof_opret,
            proof_tapret,
            proof_mbc,
            proof_mpc,
        }
    }
}
impl spk::Spk {
    pub fn new(
        spk_id: spk::Id,
        spk_spk: spk::Spk,
        spk_opret: opret::Opret,
        spk_tapret: tapret::Tapret,
        spk_mbc: mbc::Mbc,
        spk_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            spk_id,
            spk_spk,
            spk_opret,
            spk_tapret,
            spk_mbc,
            spk_mpc,
        }
    }
}
impl opret::Opret {
    pub fn new(
        opret_id: opret::Id,
        opret_opret: opret::Opret,
        opret_tapret: tapret::Tapret,
        opret_mbc: mbc::Mbc,
        opret_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            opret_id,
            opret_opret,
            opret_tapret,
            opret_mbc,
            opret_mpc,
        }
    }
}
impl tapret::Tapret {
    pub fn new(
        tapret_id: tapret::Id,
        tapret_tapret: tapret::Tapret,
        tapret_mbc: mbc::Mbc,
        tapret_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            tapret_id,
            tapret_tapret,
            tapret_mbc,
            tapret_mpc,
        }
    }
}
impl mbc::Mbc {
    pub fn new(
        mbc_id: mbc::Id,
        mbc_mbc: mbc::Mbc,
        mbc_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            mbc_id,
            mbc_mbc,
            mbc_mpc,
        }
    }
}
impl mpc::Mpc {
    pub fn new(
        mpc_id: mpc::Id,
        mpc_mpc: mpc::Mpc,
    ) -> Self {
        Self {
            mpc_id,
            mpc_mpc,
        }
    }
}