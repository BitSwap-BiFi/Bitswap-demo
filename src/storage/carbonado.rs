use carbonado::constants::MAGICNO
use storm::p2p::{Chunk, ChunkId, Container, ContainerId, ContainerInfo, Mesg, MesgId}
use rgb::schema::{verify,Root}

fn main() {
    // Connect to a Carbonado repository
    let repo = Repository::connect("postgres://localhost/mydb").unwrap();

    // Connect to a Storm server
    let client = Client::connect("127.0.0.1:6700").unwrap();

    // Connect to the Lightning Network via RGB
    let network = Network::connect("https://my-lnd-node.com").unwrap();

    // Use the protocols
    // let carbonado
       let storm
      let rgb 
}
