use std::{net::{Ipv4Addr, UdpSocket, SocketAddr, IpAddr}, thread};

use crate::constvalues;

pub struct Client {
    socket: UdpSocket,
    ipaddress: String,
    id: u8,
}

impl Client {
    pub fn send<T>(&self, _val: &T) {
        let buf = [0; 4];
        self.socket.send_to(&buf, self.ipaddress.as_str()).unwrap();
    }
    
    pub fn recieve<Func: Fn(&mut[u8]) + Send + 'static>(mut self, function: Func) {
        let selfsocket = self.socket.try_clone().unwrap();
        thread::spawn(move || {
            loop {
                let mut buf = [0; constvalues::MAX_PLAYERS * 2 + 1];
        
                let (number_of_bytes, _from) = selfsocket.recv_from(&mut buf).expect("Client recieve error");
                let mut filled_buf = &mut buf[..number_of_bytes];

                if filled_buf[0] == 26 {
                    println!("Receiving connection packet from server with id: {}", filled_buf[1]);
                    self.id = filled_buf[1];
                    continue;
                }

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
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");

    Client { socket, ipaddress: sock_addr.to_string() , id: 0 }
}