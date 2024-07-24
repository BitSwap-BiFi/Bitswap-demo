use bitcoin::{Network, network::params::Params};

trait Testnet {
    fn testnet_params() -> &'static Params;
}

impl testnet for Network {
    fn testnet_params() -> &'static Params {
        // Replace this with the actual Testnet parameters for your case
        &Params::Testnet
    }
}

