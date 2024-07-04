use cli::{
    Command,
    Main,
    Mod,
    Cli


};

use cli::{Cli, Command, Main, Mod};
use bin::Bitswapd;

fn main() {
    // Your main logic here
    let mut cli = Cli::new();
    let command = cli.parse();
    match command {
        Command::Main(Main::Bitswapd(Bitswapd::Start(mut args))) => {
            // Your logic here
            let mut vm = VM::new();
            vm.start(args);
        }
    }  
}

// #[cfg(test)]
fn test() {
    let mut vm = VM::new();
    vm.start();
}
