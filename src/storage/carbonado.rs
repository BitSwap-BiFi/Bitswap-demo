use carbonado::constants::MAGICNO
use rgb::schema::{verify,Root}

fn main() {
    // Connect to a Carbonado repository
    let repo = Repository::connect("postgres://localhost/mydb").unwrap();

    // Connect to the Lightning Network via RGB
    let network = Network::connect("https://my-lnd-node.com").unwrap();

    // Use the protocols
      let carbonado
      let rgb 
}
