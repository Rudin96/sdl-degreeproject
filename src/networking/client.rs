use std::{net::{Ipv4Addr, UdpSocket, SocketAddr, IpAddr}, thread};

const PORT_NUMBER: &str = ":1337";

pub struct Client {
    socket: UdpSocket,
    ipaddress: String
}

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
    
    pub fn connect(&self, ipaddress: &str) {
        let mut ip = ipaddress.to_owned();
        let selfsocket = self.socket.try_clone().unwrap();
        ip.push_str(PORT_NUMBER);
        selfsocket.connect(String::from(&ip)).expect("Couldnt connect to address!");
    }
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");
    let ipaddress = socket.peer_addr().unwrap().to_string();

    Client { socket, ipaddress}
}