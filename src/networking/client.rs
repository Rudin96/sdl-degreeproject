use std::{net::{Ipv4Addr, UdpSocket, SocketAddr, IpAddr, ToSocketAddrs}, thread, str::FromStr, sync::mpsc::{Receiver, Sender, channel}};


use crate::{constvalues::{self, PORT_NUMBER, BUF_SIZE}};

use super::{stream::Stream, packet::ConnectionPacket};

#[derive(Default, Copy, Clone, Debug)]
pub enum ConnectionState {
    #[default] DISCONNECTED,
    CONNECTING,
    CONNECTED
}

pub struct Client {
    socket: UdpSocket,
    ipaddress: String,
    buffer: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    stream: Stream,
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
    pub fn write<T: Copy>(&mut self, data: T) {
        self.stream.write(data);
        println!("Writing data to stream");
    }

    pub fn read<T: Default + Copy>(&mut self) -> T {
        self.stream.read::<T>()
    }
    
    fn sendconnectionrequest(&mut self) {
        //TODO send a connectrequest packet to server
        let mut connpacket = ConnectionPacket::default();
        connpacket.status = ConnectionState::CONNECTING;
        self.stream.write(connpacket);
        self.commitdata();
    }
    
    pub fn commitdata(&mut self) {
        println!("Commiting data to server");
        println!("CLIENT: Sending connectionpacket: {:?}, with bytes: {:?}", self.stream.read::<ConnectionPacket>() , self.stream.getbuffer());
        self.socket.send_to(&self.stream.getbuffer(), self.ipaddress.as_str()).unwrap();
    }

    pub fn recieve(&self) {
        let selfsocket = self.socket.try_clone().unwrap();
        let mut streamclone = self.stream.clone();
        thread::spawn(move || {
            loop {
                let mut buf = vec![0; BUF_SIZE];
                println!("Client: Starting receive thread");
                let (number_of_bytes, _from) = selfsocket.recv_from(&mut buf).expect("Client recieve error");
                streamclone.writetobuffer(buf.as_slice());
                let connpacket = streamclone.read::<ConnectionPacket>();
                println!("Client: Received packet {:?} from {}", connpacket, _from);
            }
        });
    }
    
    pub fn clearbuffer(&mut self) {
        self.stream.clear();
    }

    pub fn connect(&mut self, ipaddress: String) {
        self.recieve();
        self.ipaddress = ipaddress.clone();
        self.ipaddress.push_str(&format!(":{PORT_NUMBER}"));
        self.sendconnectionrequest();
        // self.commitdata();
    }
}

pub fn init() -> Client {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket = UdpSocket::bind(sock_addr).expect("Error binding to socket");
    let newclient = Client { socket, ipaddress: Ipv4Addr::UNSPECIFIED.to_string() , id: 0, connstate: ConnectionState::DISCONNECTED, buffer: channel(), stream: Stream::new() };
    newclient
}