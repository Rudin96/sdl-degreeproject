use std::net::Ipv4Addr;

use super::{server, client};

pub fn replicateobject(id: u8){
    println!("Replicating {id}");
}

// pub fn init() {
//     server::createwan();
//     client::connect(&Ipv4Addr::LOCALHOST.to_string()).expect("Couldnt connect to localhost");
// }