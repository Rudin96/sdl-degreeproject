use super::server;

pub fn startserver(mode: server::ServerType) {
    match mode {
        server::ServerType::LOCAL => server::createlan(),
        server::ServerType::WAN => server::createwan(),
        server::ServerType::OFFLINE => server::createoffline(),
    }
}