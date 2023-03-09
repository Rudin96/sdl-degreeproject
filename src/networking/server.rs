use std::{net::{UdpSocket, Ipv4Addr, SocketAddr}, thread, collections::HashMap};

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
        thread::spawn(move || -> &str {
            let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 1337))).expect("Couldnt bind socket");
            loop {
                let mut buf = [0; 128];
                let (number_of_bytes, from) = socket.recv_from(&mut buf).expect("Error receiving data");
                
                let _ = &connectedclients.insert(from, "a client");
                let filled_buffer = &mut buf[..number_of_bytes];
                let decoded_buffer = String::from_utf8_lossy(filled_buffer);
                // println!("Server: receiving data: {} from IP: {}", decoded_buffer, from);
                // println!("Client count: {}", connectedclients.len());
                for e in &connectedclients {
                    socket.send_to(&filled_buffer, e.0).expect("Couldnt send back to sender");
                    println!("Server: Sending {} back to client: {}", decoded_buffer, e.0);
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