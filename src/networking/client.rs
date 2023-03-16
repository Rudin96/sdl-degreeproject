use std::{net::{Ipv4Addr, UdpSocket, SocketAddr, IpAddr, ToSocketAddrs}, thread, str::FromStr};

use crate::{constvalues::{self, PORT_NUMBER}, datatypes::vector::Vector2};

pub enum ConnectionState {
    DISCONNECTED,
    CONNECTING,
    CONNECTED
}

pub struct Client {
    socket: UdpSocket,
    ipaddress: String,
    pub id: i8,
    pub connstate: ConnectionState
}

impl Client {
    pub fn sendpos(&self, pos: Vector2) {
        let serstring = serde_json::to_string(&pos).unwrap();
        println!("Pos string: {}", serstring);
        self.socket.send_to(serstring.as_bytes(), self.ipaddress.as_str()).unwrap();
    }
    
    pub fn recieve<Func: Fn(&mut[u8]) + Send + 'static>(&self, function: Func) {
        let selfsocket = self.socket.try_clone().unwrap();
        thread::spawn(move || {
            loop {
                let mut buf = [0; constvalues::BUF_SIZE];
        
                let (number_of_bytes, _from) = selfsocket.recv_from(&mut buf).expect("Client recieve error");
                let mut filled_buf = &mut buf[..number_of_bytes];

                function(&mut filled_buf);
            }
        });
    }
    
    pub fn connect(&mut self, ipaddress: String) {
        let selfsocket = &self.socket;
        let connection_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(&ipaddress).unwrap()), PORT_NUMBER);
        selfsocket.connect(&connection_addr).expect("Couldnt connect to address!");
        let mut connectstream = [0; 4];
        connectstream[0] = 26;
        selfsocket.send_to(&connectstream, &connection_addr).expect("Connection request send error");

        println!("Sending connection request");

        self.waitforclientid();
    }

    fn waitforclientid(&mut self) {
        let selfsocket = &self.socket;
        let mut buf = [0; constvalues::BUF_SIZE];
        
        let (number_of_bytes, _from) = selfsocket.recv_from(&mut buf).expect("Client recieve error");
        let filled_buf = &mut buf[..number_of_bytes];

        
        if filled_buf[0] == 26 {
            println!("Receiving connection packet from server with id: {}", filled_buf[1]);
            self.id = filled_buf[1] as i8;
            self.ipaddress = _from.to_string();
        }
    }
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");
    let newclient = Client { socket: socket.try_clone().unwrap(), ipaddress: Ipv4Addr::UNSPECIFIED.to_string() , id: 0, connstate: ConnectionState::DISCONNECTED };
    newclient
}