use std::{net::{Ipv4Addr, UdpSocket, SocketAddr, IpAddr}, thread};

const PORT_NUMBER: &str = ":1337";

pub fn connect(ipaddress: &str) -> std::io::Result<()> {
    let mut ip = ipaddress.to_owned();
    ip.push_str(PORT_NUMBER);
    println!("IP is: {ip}");
        thread::spawn(move || {
            let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
            let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");
            socket.connect(String::from(&ip)).expect("Couldnt connect to address!");

            let addr = socket.local_addr().expect("Couldnt connect to local address");
            let remoteaddr = socket.peer_addr().expect("Couldnt determine remote address");

            println!("Client: Socket address is: {addr}");
            println!("Client: Remote Socket address is: {remoteaddr}");

            loop {
                let mut message: String = String::from("");
                std::io::stdin().read_line(&mut message).expect("Not a valid entry");
                let bytemessage = message.as_bytes();
                socket.send_to(bytemessage, &ip).expect("Couldnt send message to server");
                println!("Client: Sending message to host!");

                let mut buf = [0; 1024];

                let (number_of_bytes, from) = socket.recv_from(&mut buf).expect("Client recieve error");

                let filled_buffer = &buf[..number_of_bytes];

                println!("Client: Received {} from {}", String::from_utf8_lossy(filled_buffer), from);
            }
        });
    // handle.join().unwrap();
    Ok(())
}

fn send_greetings(socket: &UdpSocket) {
    
}