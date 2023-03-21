use std::{net::{UdpSocket, Ipv4Addr, SocketAddr, SocketAddrV4}, thread, collections::HashMap, sync::{Arc, Mutex, mpsc::{channel, Receiver, Sender}}, time::Duration};
use datetime::{LocalDate, Month, DatePiece, LocalDateTime, LocalTime};

use crate::{constvalues::{BUF_SIZE, PORT_NUMBER, SERVER_TICK_RATE}};

pub struct Server {
    netbuffer: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    connectedclients: Arc<Mutex<HashMap<SocketAddr, LocalDateTime>>>,
    socket: UdpSocket
}

impl Server {
    fn beginlisten(&self) {
        //Spawn listen thread
        println!("Server: Starting listener thread..");
        let socketclone = self.socket.try_clone().unwrap();
        let conclientsclone = self.connectedclients.clone();
        let netbufclone = self.netbuffer.0.clone();
        thread::spawn(move || {
            loop {
                let mut buf = vec![0; 512];
                let (nob, from) = socketclone.recv_from(&mut buf).unwrap();
                let filled_buffer = &buf[..nob];
                println!("Received packet: {:?} from {}", filled_buffer, from);
                conclientsclone.lock().unwrap().insert(from, LocalDateTime::now());
                netbufclone.send(filled_buffer.to_vec()).unwrap();
            }
        });
    }
    
    fn beginloopingsend(&self) {
        println!("Server: Starting sender..");
        loop {
            let res = self.netbuffer.1.recv().unwrap();
            println!("Sending packet {:?} back to clients", res);
        }
    }

    fn new() -> Server {
        Server { connectedclients: Arc::new(Mutex::new(HashMap::new())), 
            netbuffer: channel(), 
            socket: UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, PORT_NUMBER))).expect("Couldnt bind socket") }
    }
}

pub enum ServerType {
    LOCAL,
    WAN,
    OFFLINE
}

fn create(_servertype: &ServerType) -> Server {
    let server = Server::new();
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