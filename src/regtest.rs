use bitcoin::{Network, network::params::Params};

trait Regtest {
    fn signet_params() -> &'static Params;
}

impl Regtest for Network {
    fn signet_params() -> &'static Params {
        // Replace this with the actual Testnet parameters for your case
        &Params::Regtest
    }
}