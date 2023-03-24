use std::{net::{UdpSocket, Ipv4Addr, SocketAddr}, thread, collections::HashMap, sync::{Arc, Mutex, mpsc::{channel, Receiver, Sender}}};
use datetime::{LocalDateTime};

use crate::{constvalues::{PORT_NUMBER}};

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
        thread::spawn(move || {
            loop {
                let mut buf = vec![0; 512];
                let (nob, from) = socketclone.recv_from(&mut buf).unwrap();
                let filled_buffer = &buf[..nob];
                println!("Server: Received packet: {:?} from {}", filled_buffer, from);
                conclientsclone.lock().unwrap().insert(from, LocalDateTime::now());
            }
        });
    }
    
    fn beginloopingsend(&self) {
        let socketclone = self.socket.try_clone().unwrap();
        let conclientsclone = self.connectedclients.clone();
        println!("Server: Starting sender..");
        let conclients = conclientsclone.lock().unwrap();
        loop {
            for c in conclients.iter() {
                let buf = self.netbuffer.1.try_recv().unwrap();
                socketclone.send_to(&buf, c.0).unwrap();
            }
            // println!("Send loop");
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

fn create(_servertype: &ServerType) -> std::io::Result<()> {
    let mut connectedclients = HashMap::new();
    let mut playerpositions: HashMap<u8, Custom_Vector2> = HashMap::new();
    let mut client_id = 0;
    thread::spawn(move || {
        let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, PORT_NUMBER))).expect("Couldnt bind socket");
        loop {
            let mut buf = [0; BUF_SIZE];
            let (number_of_bytes, from) = socket.recv_from(&mut buf).expect("Error receiving data");
            
            let filled_buffer = &mut buf[..number_of_bytes];
            
            //Check if connectionerequest packet
            if filled_buffer[0] == 26 {
                let _ = &connectedclients.insert(from, client_id);
                filled_buffer[1] = client_id;
                println!("Sending {} to client: {}", client_id, from);
                playerpositions.insert(client_id, Custom_Vector2 { x: 0, y: 0 });
                client_id += 1;
                socket.send_to(&filled_buffer, from).expect("Couldnt send connectionpacket back to client");
                for c in &connectedclients {
                    socket.send_to(serde_json::to_string(&playerpositions).unwrap().as_bytes(), c.0).unwrap();
                }
                continue;
            }

            //TODO check incoming data
            // println!("received pos string {} from client {}", String::from_utf8_lossy(&filled_buffer), from.to_string());
            let playerpos_des: Custom_Vector2 = serde_json::from_slice(&filled_buffer).unwrap();
            playerpositions.insert(*connectedclients.get(&from).unwrap(), playerpos_des);
            for c in &connectedclients {
                socket.send_to(serde_json::to_string(&playerpositions).unwrap().as_bytes(), c.0).unwrap();
            }
        }
        });
        // handle.join().unwrap();
    Ok(())
}

pub fn createwan() {
    println!("Created server at *LAN IP HERE*");
    create(&ServerType::WAN);
}

pub fn createoffline() {
    println!("Created offline session");
}