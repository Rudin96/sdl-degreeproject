use std::{net::{SocketAddr, IpAddr, Ipv4Addr, UdpSocket}, thread};

pub fn connect() -> std::io::Result<()> {
    {
        let handle = thread::spawn(|| {
        let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
        let socket = UdpSocket::bind(sock_addr).expect("Couldnt bind to socket on client");
        socket.connect("127.0.0.1:1337").expect("Couldnt connect to address!");

        let addr = socket.local_addr().expect("Couldnt connect to local address");
        let remoteaddr = socket.peer_addr().expect("Couldnt determine remote address");

        println!("Socket address is: {addr}");
        println!("Remote Socket address is: {remoteaddr}");

        
        loop {
            let mut message: String = String::from("");
            std::io::stdin().read_line(&mut message).expect("Not a valid entry");

            let bytemessage = message.as_bytes();
            socket.send_to(bytemessage, "127.0.0.1:1337").expect("Couldnt send message to server");
            println!("Sending message to host!");
        }
    });
    // handle.join().unwrap();
    }
    Ok(())
}