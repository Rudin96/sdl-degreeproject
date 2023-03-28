use super::client::ConnectionState;

#[derive(Default, Clone, Copy, Debug)]
pub struct ConnectionPacket {
    pub i: usize,
    pub status: ConnectionState,
}