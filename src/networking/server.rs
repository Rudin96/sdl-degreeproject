use std::{net::{UdpSocket, Ipv4Addr, SocketAddr}, thread, collections::HashMap};

use serde::{Serialize, ser::SerializeStruct};

use crate::{constvalues::{self, BUF_SIZE}, datatypes::vector::Vector2};

pub struct PlayerData {
    pos: [u8]
}

impl Serialize for PlayerData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("PlayerData", 1)?;
        s.serialize_field("pos", &self.pos)?;
        s.end()
    }
}

pub enum ServerType {
    LOCAL,
    WAN,
    OFFLINE
}

pub fn createlan() {
    println!("Creating lan server..");
    create(&ServerType::LOCAL).expect("server didn't create, WTF!?");
    println!("LAN Server created and listening!");
}

fn create(_servertype: &ServerType) -> std::io::Result<()> {
    let mut connectedclients = HashMap::new();
    let mut playerpositions: HashMap<u8, Vector2> = HashMap::new();
    let mut client_id = 0;
    thread::spawn(move || {
        let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 1337))).expect("Couldnt bind socket");
        loop {
            let mut buf = [0; BUF_SIZE];
            let (number_of_bytes, from) = socket.recv_from(&mut buf).expect("Error receiving data");
            
            let filled_buffer = &mut buf[..number_of_bytes];
            
            //Check if connectionerequest packet
            if filled_buffer[0] == 26 {
                let _ = &connectedclients.insert(from, client_id);
                filled_buffer[1] = client_id;
                println!("Sending {} to client: {}", client_id, from);
                playerpositions.insert(client_id, Vector2 { x: 0, y: 0 });
                client_id += 1;
                socket.send_to(&filled_buffer, from).expect("Couldnt send connectionpacket back to client");
            }

            //TODO check incoming data
            let playerPosDes: Vector2 = serde_json::from_slice(&filled_buffer).unwrap();
            
        }
        });
        // handle.join().unwrap();
    Ok(())
}

pub fn createwan() {
    println!("Created server at *LAN IP HERE*");
    // create(&ServerType::WAN).expect("server didn't create, WTF!?");
}

pub fn createoffline() {
    println!("Created offline session");
}