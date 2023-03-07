use std::{net::{UdpSocket, SocketAddr, Ipv4Addr, IpAddr}, thread};

pub enum ServerType {
    LOCAL,
    WAN,
    OFFLINE
}

pub fn createlan() {
    println!("Create Lan server at *LAN IP HERE*");
    create("127.0.0.1:1337").expect("server didn't create, WTF!?");
}

fn create(ip_address: &str) -> std::io::Result<()> {
    {
        let handle = thread::spawn(|| {
            let socket = UdpSocket::bind("127.0.0.1:1337").expect("couldn't bind to address");
            let mut buf = [0; 1024];
            loop {
                let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
                let filled_buf = &mut buf[..number_of_bytes];
                let decodedmessage = String::from_utf8(filled_buf.to_vec()).expect("Couldnt convert message!");
                println!("Received message from client with addr: {} that says: {}", src_addr, decodedmessage);
            }
        });

        // handle.join().unwrap();
    }
    Ok(())
}

pub fn createwan() {
    println!("Created server at *LAN IP HERE*");
    create("127.0.0.1:1337").expect("server didn't create, WTF!?");
}

pub fn createoffline() {
    println!("Created offline session");
}