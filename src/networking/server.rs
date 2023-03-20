use std::{net::{UdpSocket, Ipv4Addr, SocketAddr, SocketAddrV4}, thread, collections::HashMap, sync::{Arc, Mutex, mpsc::{channel, Receiver, Sender}}};
use datetime::{LocalDate, Month, DatePiece, LocalDateTime, LocalTime};

use crate::{constvalues::{BUF_SIZE, PORT_NUMBER}};

pub struct Server {
    netbuffer: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    connectedclients: Arc<Mutex<HashMap<SocketAddr, LocalDateTime>>>
}

impl Server {
    fn beginlisten(&self) {
        //Spawn listen thread
        let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, PORT_NUMBER))).expect("Couldnt bind socket");
        let netsenderclone = self.netbuffer.0.clone();
        let connclientsclone = self.connectedclients.clone();
        println!("Server: Starting listener thread..");
        thread::spawn(move || {
            loop {
                let mut buf = [0; 1024];
                let (bytes, from) = socket.recv_from(&mut buf).unwrap();
                let mut connclients = connclientsclone.lock().unwrap();
                println!("Server: Received packet from {from}");
                connclients.insert(from, LocalDateTime::now());
                let filled_buf = &mut buf[..bytes];
                netsenderclone.send(filled_buf.to_vec()).unwrap();
            }
        });
    }
    
    fn beginloopingsend(&self) {
        let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, PORT_NUMBER + 1))).expect("Couldnt create listener");
        println!("Server: Starting sender..");
        loop {
            let netbuf = self.netbuffer.1.recv().unwrap();
            let connclients = self.connectedclients.lock().unwrap();
            println!("Connected clients: {}", connclients.len());
            for (k, v) in connclients.iter() {
                println!("Server: Sending packet: {:?} to a {} here", netbuf, k);
                socket.send_to(&netbuf, k).unwrap();
            }
        }
    }
}

pub enum ServerType {
    LOCAL,
    WAN,
    OFFLINE
}

fn create(_servertype: &ServerType) -> Server {
    let mut server = Server {connectedclients: Arc::new(Mutex::new(HashMap::new())), netbuffer: channel()};
    server.beginlisten();
    server.beginloopingsend();
    server
}

pub fn createlan() {
    println!("Creating lan server..");
    create(&ServerType::LOCAL);
    println!("LAN Server created and listening!");
}


pub fn createwan() {
    println!("Created server at *LAN IP HERE*");
    create(&ServerType::WAN);
}

pub fn createoffline() {
    println!("Created offline session");
}