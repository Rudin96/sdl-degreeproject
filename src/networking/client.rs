use std::{net::{Ipv4Addr, UdpSocket, SocketAddr, IpAddr}, thread};

use crate::{constvalues, datatypes::vector::Vector2};


pub struct Client {
    socket: UdpSocket,
    ipaddress: String,
    id: i8,
}

impl Client {
    pub fn sendpos(&self, pos: Vector2) {
        let serstring = serde_json::to_string(&pos).unwrap();
        self.socket.send_to(serstring.as_bytes(), self.ipaddress.as_str()).unwrap();
    }
    
    pub fn recieve<Func: Fn(&mut[u8]) + Send + 'static>(&self, function: Func) {
        let selfsocket = self.socket.try_clone().unwrap();
        thread::spawn(move || {
            loop {
                let mut buf = [0; constvalues::MAX_PLAYERS * 2 + 1];
        
                let (number_of_bytes, _from) = selfsocket.recv_from(&mut buf).expect("Client recieve error");
                let mut filled_buf = &mut buf[..number_of_bytes];

                function(&mut filled_buf);
            }
        });
    }
    
    pub fn connect(&self, mut ipaddress: String) {
        let selfsocket = self.socket.try_clone().unwrap();
        ipaddress.push_str(constvalues::PORT_NUMBER);
        selfsocket.connect(&ipaddress).expect("Couldnt connect to address!");
        let mut connectstream = [0; 4];
        connectstream[0] = 26;
        selfsocket.send_to(&connectstream, &ipaddress).expect("Connection request send error");

        println!("Sending connection request");
    }

    pub fn waitforclientid(&self) -> i8 {
        let selfsocket = self.socket.try_clone().unwrap();
        let mut buf = [0; constvalues::MAX_PLAYERS * 2 + 1];
        
        let (number_of_bytes, _from) = selfsocket.recv_from(&mut buf).expect("Client recieve error");
        let filled_buf = &mut buf[..number_of_bytes];

        if filled_buf[0] == 26 {
             println!("Receiving connection packet from server with id: {}", filled_buf[1]);
             return filled_buf[1] as i8
        } else {
            -1
        }
    }
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");
    let connectionip = String::from("127.0.0.1");
    println!("Socket address is: {}", socket.local_addr().unwrap().to_string());
    let mut newclient = Client { socket, ipaddress: connectionip.to_string() , id: 0 };
    newclient.connect(connectionip.to_string());
    newclient.ipaddress.push_str(constvalues::PORT_NUMBER);
    newclient.id = newclient.waitforclientid();
    newclient
}