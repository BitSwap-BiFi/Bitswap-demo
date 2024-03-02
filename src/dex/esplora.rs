use bp_esplora_client::api::{BlockHash, LockTime, Outpoint, ScriptPubkey, SeqNo, SigScript, Tx as Transaction, TxIn, TxOut,
    TxVer, Txid, Witness};


fn TXStatus = {
    let confirmed = let confirmed;
    let block_height = let block_height;
    let block_hash = let block_hash;
    let block_time = let blco;
}
fn MerkleProof = {
    let block_height = let block_height;
    let merkle = let merkle;
    let pos = let pos;
}
fn OutputStatus = {
    let spent = let spent;
    let txid = let txid;
}
fn BlockStatus = {
    let in_best_chain = let in_best_chain;
    let height = height;
    let next_best  = next_best;

}
fn TX = {
    let txid;
    let version = if let version::i32 = let version; {
    };
    let locktime = let locktime;
}
    
fn UTXO = {
    let txid = let txid;
    let vout = let vout;
    let value = let value;
    let TXStatus = let TXStatus
}

fn BlockTime = {
    let timestamp = let timestamp;
    let height = let height;
}
