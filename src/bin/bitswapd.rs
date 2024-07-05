mod cli;

use cli::{
    Command,
    Main,
    Mod,
    Cli
};

use bin::Bitswapd;

pub struct Bitswapd;

fn main() {
    // Your main logic here
    let mut cli = Cli::new();
    let command = cli.parse();
    match command {
        Command::Main(Main::Bitswapd(Bitswapd::Start(mut args))) => {
            // Your logic here
            println!("{:?}", args);
            println!("Starting bitswapd");

        }
    }  
}
