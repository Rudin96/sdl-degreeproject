use std::{net::{UdpSocket, Ipv4Addr, SocketAddr, SocketAddrV4}, thread, collections::HashMap};

pub enum ServerType {
    LOCAL,
    WAN,
    OFFLINE
}

pub fn createlan() {
    println!("Creating lan server..");
    create(&ServerType::LOCAL).expect("server didn't create, WTF!?");
}

fn create(servertype: &ServerType) -> std::io::Result<()> {
    let mut connectedclients = HashMap::new();
        thread::spawn(move || -> &str {
            let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 1337))).expect("Couldnt bind socket");
            loop {
                let mut buf = [0; 1024];
                let (number_of_bytes, from) = socket.recv_from(&mut buf).expect("Error receiving data");
                
                connectedclients.insert(from, "a client");
                let filled_buffer = &mut buf[..number_of_bytes];
                let decoded_buffer = String::from_utf8_lossy(filled_buffer);
                println!("Server: receiving data: {} from IP: {}", decoded_buffer, from);
                println!("Server: Sending {} back to client", decoded_buffer);
                socket.send_to(&filled_buffer, from).expect("Couldnt send back to sender");
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