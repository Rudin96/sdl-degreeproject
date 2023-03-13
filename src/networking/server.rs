use std::{net::{UdpSocket, Ipv4Addr, SocketAddr}, thread, collections::HashMap};

use sdl2::rect::Point;
use serde::{Serialize, ser::SerializeStruct};

use crate::constvalues;

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
    let mut playerpositions: Vec<Point> = Vec::new();
    let mut client_id = 0;
    thread::spawn(move || {
        let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 1337))).expect("Couldnt bind socket");
        loop {
            let mut buf = [0; constvalues::MAX_PLAYERS * 2 + 1];
            let (number_of_bytes, from) = socket.recv_from(&mut buf).expect("Error receiving data");
            
            let filled_buffer = &mut buf[..number_of_bytes];
            
            //Check if connectionerequest packet
            if filled_buffer[0] == 26 {
                let _ = &connectedclients.insert(from, client_id);
                filled_buffer[1] = client_id;
                println!("Sending {} to client: {}", client_id, from);
                client_id += 1;
                playerpositions.push(Point::new(0, 0));
                socket.send_to(&filled_buffer, from).expect("Couldnt send connectionpacket back to client");
            }
            
            // let decoded_buffer = String::from_utf8_lossy(filled_buffer);
            for e in &connectedclients {
                buf[0] = 17;

                let mut counter = 1;
                for i in &playerpositions {
                    buf[counter] = i.x() as u8;
                    buf[counter + 1] = i.y() as u8;
                    counter += 2;
                }
                socket.send_to(&buf, e.0).expect("Couldnt send back to sender");
                println!("Sending data back to clients");
                // println!("Server: Sending {} back to client: {}", decoded_buffer, e.0);
            }
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