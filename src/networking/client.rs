use std::{net::{Ipv4Addr, UdpSocket, SocketAddr, IpAddr, ToSocketAddrs}, thread, str::FromStr, sync::mpsc::{Receiver, Sender, channel}, fmt::Debug};


use crate::{constvalues::{self, PORT_NUMBER, BUF_SIZE}, networking::packet::WorldPacket};

use super::{stream::Stream, packet::ConnectionPacket};

#[derive(Default, Copy, Clone, Debug)]
pub enum ConnectionState {
    #[default] DISCONNECTED,
    CONNECTING,
    CONNECTED
}

pub struct Client {
    socket: UdpSocket,
    buffer: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    ipaddress: String,
    stream: Stream,
    pub id: u8,
    pub connstate: ConnectionState
}

impl Client {
    pub fn write<T: Copy>(&mut self, data: T) {
        self.stream.write(data);
        println!("Writing data to stream");
    }

    pub fn read<T: Default>(&mut self) -> T {
        self.stream.read::<T>()
    }
    
    fn sendconnectionrequest(&mut self) {
        //TODO send a connectrequest packet to server
        let mut connpacket = ConnectionPacket::default();
        connpacket.status = ConnectionState::CONNECTING;
        self.stream.write(connpacket);
        self.commitdata();
        let mut buf = vec![0; 128];
        self.socket.recv(&mut buf).unwrap();
        self.stream.writetobuffer(buf.as_slice());
        let connpacket = self.stream.read::<ConnectionPacket>();
        println!("CLIENT: Received connection packet: {:?}", connpacket);
        self.connstate = connpacket.status;
        self.recieve();
        self.beginmainloop();
    }
    
    pub fn commitdata(&mut self) {
        println!("CLIENT: Commiting data to server");
        // println!("CLIENT: Sending packet with bytes: {:?}", &self.stream.getbuffer());
        self.socket.send_to(&self.stream.getbuffer(), self.ipaddress.as_str()).unwrap();
    }

    pub fn recieve(&self) {
        let selfsocket = self.socket.try_clone().unwrap();
        let bufsender = self.buffer.0.clone();
        println!("Client: Starting receive thread");
        thread::spawn(move || {
            loop {
                let mut buf = vec![0; BUF_SIZE];
                selfsocket.recv_from(&mut buf).expect("Client recieve error");
                bufsender.send(buf).unwrap();
            }
        });
    }
    
    pub fn clearbuffer(&mut self) {
        self.stream.clear();
    }

    pub fn beginmainloop(&mut self) {
        loop {
            let mut stream = Stream::new();
            match self.buffer.1.try_recv() {
                Ok(incbuf) => {
                    stream.writetobuffer(&incbuf.as_slice());
                    let worldpacket = stream.read::<WorldPacket>();
                    println!("CLIENT: Received Worldpacket {:?} from server", worldpacket);
                },
                Err(e) => {

                },
            }
            // self.stream.clear();
        }
    }

    pub fn connect(&mut self, ipaddress: String) {
        self.ipaddress = ipaddress.clone();
        self.ipaddress.push_str(&format!(":{PORT_NUMBER}"));
        self.sendconnectionrequest();
    }
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");
    let newclient = Client { socket, 
        ipaddress: Ipv4Addr::UNSPECIFIED.to_string(), 
        id: 0, connstate: ConnectionState::DISCONNECTED, 
        stream: Stream::new(),
        buffer: channel()
     };
    newclient
}