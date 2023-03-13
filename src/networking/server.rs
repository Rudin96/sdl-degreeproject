use std::{net::{UdpSocket, Ipv4Addr, SocketAddr}, thread, collections::HashMap, vec};

use sdl2::rect::Point;

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
        thread::spawn(move || -> &str {
            let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 1337))).expect("Couldnt bind socket");
            loop {
                let mut buf = [0; 4];
                let (number_of_bytes, from) = socket.recv_from(&mut buf).expect("Error receiving data");
                
                let filled_buffer = &mut buf[..number_of_bytes];

                //Check if connectionerequest packet
                if filled_buffer[0] == 26 && number_of_bytes == 2 {
                    let _ = &connectedclients.insert(from, client_id);
                    filled_buffer[1] = client_id;
                    println!("Client id sent from server is: {client_id}");
                    client_id += 1;
                    playerpositions.push(Point::new(0, 0));
                    socket.send_to(&filled_buffer, from).expect("Couldnt send connectionpacket back to client");
                    continue;
                }


                
                // let decoded_buffer = String::from_utf8_lossy(filled_buffer);
                for e in &connectedclients {
                    socket.send_to(&filled_buffer, e.0).expect("Couldnt send back to sender");
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