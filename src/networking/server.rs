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
                let mut buf = vec![0; 512];
                match socketclone.recv_from(&mut buf) {
                    Ok(d) => {
                        println!("SERVER: RECEIVED DATA");
                        let filled_buffer = &buf[..d.0];
                        stream.writetobuffer(filled_buffer);
                        // println!("SERVER: Received packet: {:?}, from {}", stream.read::<ConnectionPacket>(), from);
                        let mut connpacket = stream.read::<ConnectionPacket>();
                        connpacket.status = ConnectionState::CONNECTED;
                        connpacket.i = clindex;
                        conclientssender.send((d.1, clindex)).unwrap();
                        clindex += 1;
                        stream.write(connpacket);
                        Self::send(&socketclone, &stream, &d.1);
                    },
                    Err(d) => {println!("SERVER: Error receiving data from client, disconnect here")},
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
                self.worldpacket.pos.push((c.1, 0, 0));
            },
            Err(_) => {},
        }
    }
    
    fn senddatatoclients(&self) {
        for c in self.connectedclients.iter() {
            let mut stream = Stream::new();
            println!("SERVER: Sending wpacket: {:?}", &self.worldpacket);
            stream.write(&self.worldpacket);
            Self::send(&self.socket, &stream, c.0);
            // println!("SERVER: Sending world state to {}", c.0);
        }
    }
    
    fn send(socket: &UdpSocket, stream: &Stream, to: &SocketAddr) {
        socket.send_to(&stream.getbuffer(), to).unwrap();
    }

    fn new() -> Server {
        Server { connectedclientssender: channel(),
                socket: UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, PORT_NUMBER))).expect("Couldnt bind socket"),
                connectedclients: HashMap::new(),
                worldpacket: WorldPacket::default()
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