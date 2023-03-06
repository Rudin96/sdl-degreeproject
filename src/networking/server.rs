use std::net::UdpSocket;

pub const IP_ADDRESS: &str = "127.0.0.1:1337";

pub enum ServerType {
    LOCAL,
    WAN,
    OFFLINE
}

pub fn createlan() {
    println!("Create Lan server at {IP_ADDRESS} on port: *port*");
    create().expect("server didn't create, WTF!?");
}

fn create() -> std::io::Result<()> {
    //TODO implement udp server host here.
}

pub fn createwan() {
    println!("Created server at {IP_ADDRESS} on port: *port*");
    create().expect("server didn't create, WTF!?");
}

pub fn createoffline() {
    println!("Created offline session");
}