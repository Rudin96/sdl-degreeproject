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
    pub fn writepoint(&self, pos: (i32, i32)) {
        let mut buf = Vec::<u8>::new();
        let c: u8 = 76;
        buf.push(c);
        let pb1 = pos.0.to_ne_bytes();
        let pb2 = pos.1.to_ne_bytes();
        buf.append(&mut pb1.to_vec());
        buf.append(&mut pb2.to_vec());
        println!("CLIENT: Pos buffer looks like: {:?}", buf);
        self.buffer.0.send(buf).unwrap();
    }
    
    fn sendconnectionrequest(&mut self, ipaddress: String) {
        let socketclone = self.socket.try_clone().unwrap();
        println!("{ipaddress}");
        let connection_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(&ipaddress).unwrap()), PORT_NUMBER);
        println!("Sending connection request");
        socketclone.connect(&connection_addr).expect("Couldnt connect to address!");
        let buf = vec![0; 4];
        self.buffer.0.send(buf).unwrap();
        self.ipaddress.push_str(format!(":{}", PORT_NUMBER).as_str());
    }

    pub fn commitdata(&self) {
        let buf = self.buffer.1.recv().unwrap();
        println!("CLIENT: Self IP is: {}", self.ipaddress);
        self.socket.send_to(&buf, self.ipaddress.as_str()).unwrap();
    }

    pub fn recieve(&self) {
        let selfsocket = self.socket.try_clone().unwrap();
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
            }
        });
    }
    
    pub fn connect(&mut self, ipaddress: String) {
        self.recieve();
        self.ipaddress = ipaddress.clone();
        self.sendconnectionrequest(ipaddress.to_string());
    }
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");
    let newclient = Client { socket, ipaddress: Ipv4Addr::UNSPECIFIED.to_string() , id: 0, connstate: ConnectionState::DISCONNECTED, buffer: channel() };
    newclient
}