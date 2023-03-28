use std::{net::{UdpSocket, Ipv4Addr, SocketAddr}, thread, collections::HashMap, sync::{Arc, Mutex, mpsc::{channel, Receiver, Sender}}};
use std::any::type_name;
use datetime::{LocalDateTime};

use crate::{constvalues::{PORT_NUMBER, BUF_SIZE}, networking::{packet::ConnectionPacket}};
use super::client::ConnectionState;

use super::stream::Stream;

pub struct Server {
    connectedclients: Arc<Mutex<HashMap<SocketAddr, LocalDateTime>>>,
    socket: UdpSocket,
    stream: Stream
}

impl Server {
    fn beginlisten(&self) {
        //Spawn listen thread
        println!("Server: Starting listener thread..");
        let socketclone = self.socket.try_clone().unwrap();
        let conclientsclone = self.connectedclients.clone();
        let mut stream = self.stream.clone();
        thread::spawn(move || {
            loop {
                let mut buf = vec![0; 512];
                let mut conclients = conclientsclone.lock().unwrap();
                let (nob, from) = socketclone.recv_from(&mut buf).unwrap();
                println!("RECEIVED DATA");
                let filled_buffer = &buf[..nob];
                stream.writetobuffer(filled_buffer);
                println!("Server: Received packet: {:?} with bytes: {:?} from {}", stream.read::<ConnectionPacket>(), stream.getbuffer(), from);
                let mut connpacket = stream.read::<ConnectionPacket>();
                connpacket.status = ConnectionState::CONNECTED;
                conclients.insert(from, LocalDateTime::now());
                connpacket.i = conclients.len();
                stream.clear();
                stream.write(connpacket);
                socketclone.send_to(&stream.getbuffer(), from).unwrap();
                println!("Amount of clients: {}", conclients.len());
            }
        });
    }

    fn new() -> Server {
        Server { connectedclients: Arc::new(Mutex::new(HashMap::new())), 
            socket: UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, PORT_NUMBER))).expect("Couldnt bind socket"),
            stream: Stream::new() }
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
    // server.beginloopingsend();
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