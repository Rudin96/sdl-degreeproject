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
    // stream: Stream,
    pub id: u8,
    pub connstate: ConnectionState
}

impl Client {
    // pub fn write<T>(&mut self, data: T) {
    //     self.stream.write(data);
    //     println!("Writing data to stream");
    // }

    // pub fn read<T>(&mut self) -> &T {
    //     self.stream.read::<T>()
    // }
    
    fn sendconnectionrequest(&mut self) {
        //TODO send a connectrequest packet to server
        let mut connpacket = ConnectionPacket::new();
        connpacket.status = ConnectionState::CONNECTING;
        let mut stream = Stream::new();
        stream.write(connpacket);
        self.commitdata(&mut stream);
        let mut buf = vec![0; BUF_SIZE];
        self.socket.recv(&mut buf).unwrap();
        let connpacket = Stream::readfrombuffer::<ConnectionPacket>(buf.as_mut_slice());
        println!("CLIENT: Received connection packet: {:?}", connpacket);
        self.connstate = connpacket.status;
        self.recieve();
        self.beginmainloop();
    }
    
    pub fn commitdata(&mut self, stream: &mut Stream) {
        println!("CLIENT: Commiting data to server");
        // println!("CLIENT: Sending packet with bytes: {:?}", &stream.getbuffer());
        self.socket.send_to(&stream.getbuffer(), self.ipaddress.as_str()).unwrap();
    }

    pub fn recieve(&self) {
        let selfsocket = self.socket.try_clone().unwrap();
        let bufsender = self.buffer.0.clone();
        println!("Client: Starting receive thread");
        thread::spawn(move || {
            loop {
                let mut stream = Stream::new();
                let mut buf = vec![0; BUF_SIZE];
                selfsocket.recv_from(&mut buf).expect("Client recieve error");
                stream.writetobuffer(buf.as_mut_slice());
                let wp = stream.read::<WorldPacket>();
                println!("CLIENT: Stream data {:?}", wp);
                bufsender.send(buf).unwrap();
            }
        });
    }
    
    // pub fn clearbuffer(&mut self) {
    //     self.stream.clear();
    // }

    pub fn beginmainloop(&mut self) {
        loop {
            let mut stream = Stream::new();
            match self.buffer.1.try_recv() {
                Ok(incbuf) => {

                },
                Err(e) => {
                    self.connstate = ConnectionState::DISCONNECTED;
                    // println!("CLIENT: Stopped receiving from server, disconnecting!");
                },
            }
            // self.stream.clear();
        }
    }

    pub fn connect(&mut self, ipaddress: String) {
        self.ipaddress = ipaddress.clone();
        self.ipaddress.push_str(&format!(":{PORT_NUMBER}"));
        self.socket.connect(&self.ipaddress).unwrap();
        self.sendconnectionrequest();
    }
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");
    Client { socket, 
        ipaddress: Ipv4Addr::UNSPECIFIED.to_string(), 
        id: 0, connstate: ConnectionState::DISCONNECTED,
        buffer: channel()
    }
}