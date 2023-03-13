use std::{net::{Ipv4Addr, UdpSocket, SocketAddr, IpAddr, ToSocketAddrs}, thread};

use sdl2::rect::Point;

const PORT_NUMBER: &str = ":1337";

pub struct Client {
    socket: UdpSocket,
    ipaddress: String,
    id: u8,
}

pub trait ToBytes {
    fn to_byte_slice(&self) -> &[u8];
}

// impl ToBytes for Point {
    // fn to_byte_slice(&self) -> &[u8] {
    //     let mut vector: Vec<u8> = Vec::new();
    //     vector.insert(0, self.x.try_into().unwrap());
    //     vector.insert(1, self.y.try_into().unwrap());
    //     let pointslice = vector.clone().as_slice();
    // }
// }

impl Client {
    pub fn send<T>(&self, val: &T) {
        let buf = [0; 128];
        self.socket.send_to(&buf, self.ipaddress.as_str()).unwrap();
    }
    
    pub fn recieve<Func: Fn(&mut[u8]) + Send + 'static>(&self, function: Func) {
        let selfsocket = self.socket.try_clone().unwrap();
        thread::spawn(move || {
            loop {
                let mut buf = [0; 128];
        
                let (number_of_bytes, from) = selfsocket.recv_from(&mut buf).expect("Client recieve error");
                let mut filled_buf = &mut buf[..number_of_bytes];

                function(&mut filled_buf);
            }
        });
    }
    
    pub fn connect(mut self, mut ipaddress: String) {
        let selfsocket = self.socket.try_clone().unwrap();
        ipaddress.push_str(PORT_NUMBER);
        selfsocket.connect(&ipaddress).expect("Couldnt connect to address!");
        let mut connectstream = [0; 4];
        connectstream[0] = 26;
        selfsocket.send_to(&connectstream, &ipaddress).expect("Connection request send error");


        selfsocket.recv_from(&mut connectstream).expect("Error receiving connection packet");
        if connectstream[0] == 26 {
            self.id = connectstream[1];
            println!("Client id from server is: {}", self.id);
        }
    }
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");

    Client { socket, ipaddress: sock_addr.to_string() , id: 0 }
}