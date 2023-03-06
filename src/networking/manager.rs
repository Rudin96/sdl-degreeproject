use std::net::UdpSocket;

use super::server;

pub fn startserver(mode: server::ServerType) {
    match mode {
        server::ServerType::LOCAL => server::createlan(),
        server::ServerType::WAN => server::createwan(),
        server::ServerType::OFFLINE => server::createoffline(),
    }
}

pub fn senddata<T>(data: T) {
    let socket = UdpSocket::bind(server::IP_ADDRESS).expect("Couldnt bind to ip address");

    let message = "Test message!";

    

    socket.send_to(message.as_bytes(), server::IP_ADDRESS).expect("Couldnt send message to server, please investigate!");
}