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
    let socket = UdpSocket::bind(IP_ADDRESS)?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;

        for e in buf {
            println!("Message line in buffer is: {e}");
        }

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;

    Ok(())
}

pub fn createwan() {
    println!("Created server at {IP_ADDRESS} on port: *port*");
    create().expect("server didn't create, WTF!?");
}

pub fn createoffline() {
    println!("Created offline session");
}