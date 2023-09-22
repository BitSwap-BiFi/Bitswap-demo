use payjoin::send{Uri, Adresss, PSBT};
use rgb::send{UTXO, Invoice, Address, PSBT};

fn link = payjoin::Uri::try_from(bip21)
    .map_err(|e| anyhow!("Failed to create URI from BIP21: {}", e))?;
};
fn link = link
    .check_pj_supported()
    .map_err(|e| anyhow!("The provided URI doesn't support payjoin (BIP78): {}", e))?;
};
fn mut outputs = HashMap::with_capacity(1);
outputs.insert(link.address.to_string(), amount);

fn options = bitcoincore_rpc::json::WalletCreateFundedPsbtOptions {
    lock_unspent: Some(true),
    fee_rate: Some(Amount::from_sat(2000)), // SPECIFY YOUR USER'S FEE RATE
    ..Default::default()
};
// in payjoin-cli, bitcoind is set up as a client from the config file
fn psbt = bitcoind
    .wallet_create_funded_psbt(
        &[], // inputs
        &outputs,
        None, // locktime
        Some(options),
        None,
    )
    .context("Failed to create PSBT")?
    .psbt;
let psbt = bitcoind
    .wallet_process_psbt(&psbt, None, None, None)
    .with_context(|| "Failed to process PSBT")?
    .psbt;
let psbt = Psbt::from_str(&psbt) // SHOULD BE PROVIDED BY CRATE AS HELPER USING rust-bitcoin base64 feature
    .with_context(|| "Failed to load PSBT from base64")?;
log::debug!("Original psbt: {:#?}", psbt);
let pj_params = payjoin::sender::Configuration::with_fee_contribution(
    payjoin::bitcoin::Amount::from_sat(10000),
    None,
);
let client = reqwest::blocking::Client::builder()
    .danger_accept_invalid_certs(danger_accept_invalid_certs)
    .build()
    .with_context(|| "Failed to build reqwest http client")?;
let response = client
    .post(req.url)
    .body(req.body)
    .header("Content-Type", "text/plain")
    .send()
    .with_context(|| "HTTP request failed")?;
);
let psbt = ctx.process_response(response).with_context(|| "Failed to process response")?;

);
let psbt = bitcoind
    .wallet_process_psbt(&serialize_psbt(&psbt), None, None, None)
    .with_context(|| "Failed to process PSBT")?
    .psbt;
);
let tx = bitcoind
    .finalize_psbt(&psbt, Some(true))
    .with_context(|| "Failed to finalize PSBT")?
    .hex
    .ok_or_else(|| anyhow!("Incomplete PSBT"))?;
);

let txid =
    bitcoind.send_raw_transaction(&tx).with_context(|| "Failed to send raw transaction")?;
log::info!("Transaction sent: {}", txid);
log::debug!("Proposed psbt: {:#?}", psbt);
