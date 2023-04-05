use std::{net::{UdpSocket, Ipv4Addr, SocketAddr}, thread::{self, JoinHandle}, collections::HashMap, sync::{Arc, Mutex, mpsc::{channel, Receiver, Sender}}, time::Duration};
use datetime::{LocalDateTime};

use crate::{constvalues::{PORT_NUMBER, BUF_SIZE}, networking::{packet::ConnectionPacket}};
use super::{client::ConnectionState, packet::WorldPacket};

use super::stream::Stream;

pub struct Server {
    connectedclientssender: (Sender<(SocketAddr, usize)>, Receiver<(SocketAddr, usize)>),
    connectedclients: HashMap<SocketAddr, usize>,
    worldpacket: WorldPacket,
    socket: UdpSocket
}

impl Server {
    fn beginlisten(&self) {
        //Spawn listen thread
        println!("Server: Starting listener thread..");
        let socketclone = self.socket.try_clone().unwrap();
        let mut clindex: usize = 0;
        let conclientssender = self.connectedclientssender.0.clone();
        thread::spawn(move || {
            loop {
                let mut stream = Stream::new();
                let mut buf = vec![0; BUF_SIZE];
                match socketclone.recv_from(&mut buf) {
                    Ok(d) => {
                        let mut connpacket = Stream::readfrombuffer::<ConnectionPacket>(&buf).clone();
                        conclientssender.send((d.1, clindex)).unwrap();
                        connpacket.i = clindex;
                        connpacket.status = ConnectionState::CONNECTED;
                        let conpacketclone = connpacket.clone();
                        stream.write(connpacket);
                        println!("SERVER: Sending connection data to clients: {:?}", conpacketclone);
                        Self::send(&socketclone, &stream.getbuffer(), &d.1);
                        clindex += 1;
                    },
                    Err(d) => {
                        println!("SERVER: Error receiving data from client, disconnect here, message: {:?}", d.into_inner());
                    },
                }
            }
        });
    }
    
    fn beginmainloop(&mut self) {
        loop {
            self.handleincomingconnections();
            self.senddatatoclients();
            thread::sleep(Duration::from_secs_f32(1.0 / 30.0));
        }
    }
    
    fn handleincomingconnections(&mut self) {
        match self.connectedclientssender.1.try_recv() {
            Ok(c) => {
                println!("SERVER: New connection receieved from {} at {:?}", c.0, c.1);
                    self.connectedclients.insert(c.0, c.1);
                    self.worldpacket.pos.insert(c.1, (0, 0));
            },
            Err(_) => {},
        }
    }
    
    fn senddatatoclients(&mut self) {
        for c in self.connectedclients.iter() {
            let mut stream = Stream::new();
            let worldpacketclone = self.worldpacket.clone();
            stream.write(worldpacketclone);
            let data = &*stream.getbuffer();
            // println!("SERVER: Sending data with length: {:?}", Stream::readfrombuffer::<WorldPacket>(data));
            // println!("SERVER: Sending data: {:?}", data);
            Self::send(&self.socket, data, c.0);
        }
    }
    
    fn send(socket: &UdpSocket, data: &[u8], to: &SocketAddr) {
        // println!("SERVER: Sending world state {:?} to {}", Stream::readfrombuffer::<WorldPacket>(data), to);
        socket.send_to(data, to).unwrap();
    }

    fn new() -> Server {
        Server { connectedclientssender: channel(),
                socket: UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, PORT_NUMBER))).expect("Couldnt bind socket"),
                connectedclients: HashMap::new(),
                worldpacket: WorldPacket::new()
            }
    }
}

pub enum ServerType {
    LOCAL,
    WAN,
    OFFLINE
}

fn create(_servertype: &ServerType) -> Server {
    let mut server = Server::new();
    server.beginlisten();
    server.beginmainloop();
    server
}

pub fn createlan() -> JoinHandle<()> {
    println!("Creating lan server..");
    let handle = thread::spawn(|| {
        create(&ServerType::LOCAL);
    });
    println!("LAN Server created and listening!");
    handle
}

pub fn createwan() {
    println!("Created server at *LAN IP HERE*");
    create(&ServerType::WAN);
}

pub fn createoffline() {
    println!("Created offline session");
}