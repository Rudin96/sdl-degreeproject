use std::{net::{Ipv4Addr, UdpSocket, SocketAddr, IpAddr, ToSocketAddrs}, thread, str::FromStr, sync::mpsc::{Receiver, Sender, channel}};

use crate::{constvalues::{self, PORT_NUMBER}};

pub enum ConnectionState {
    DISCONNECTED,
    CONNECTING,
    CONNECTED
}

pub struct Client {
    socket: UdpSocket,
    ipaddress: String,
    buffer: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    pub id: u8,
    pub connstate: ConnectionState
}

fn checkifconnectionreq(packet: &[u8]) -> bool {
    if packet[0] == 26 {
        return true;
    } else {
        return false;
    }
}

impl Client {
    pub fn writetostream(&self, pos: (i32, i32)) {
        let serstring = serde_json::to_string(&pos).unwrap();
        self.buffer.0.send(serstring.into_bytes()).unwrap();
    }
    
    fn sendconnectionrequest(&self, ipaddress: String) {
        let mut connectstream = [0; 4];
        connectstream[0] = 26;
        let connection_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(&ipaddress).unwrap()), PORT_NUMBER);
        println!("Sending connection request");
        self.socket.connect(&connection_addr).expect("Couldnt connect to address!");
        self.socket.send_to(&connectstream, &connection_addr).expect("Connection request send error");
    }

    fn beginsendtoserver(&self) {
        // loop {
        //     self.socket.send_to(&r, self.ipaddress.as_str()).unwrap();
        // }
    }

    pub fn recieve(&self) {
        let selfsocket = self.socket.try_clone().unwrap();
        let netbuf = self.buffer.0.clone();
        thread::spawn(move || {
            loop {
                let mut buf = vec![0; 1024];
                println!("Client: Starting receive thread");
                let (number_of_bytes, _from) = selfsocket.recv_from(&mut buf).expect("Client recieve error");
                let filled_buf = &mut buf[..number_of_bytes];
                println!("Client: Received packet from {}", _from);
                if checkifconnectionreq(&filled_buf) {
                    println!("Connection successful");
                }
                netbuf.send(filled_buf.to_vec()).unwrap();
            }
        });
    }
    
    pub fn connect(&self, ipaddress: String) {
        self.recieve();
        self.sendconnectionrequest(ipaddress.to_string());
        self.beginsendtoserver();
    }
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");
    let newclient = Client { socket, ipaddress: Ipv4Addr::UNSPECIFIED.to_string() , id: 0, connstate: ConnectionState::DISCONNECTED, buffer: channel() };
    newclient
}