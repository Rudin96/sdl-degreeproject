use std::collections::HashMap;

use super::client::ConnectionState;

#[derive(Clone, Debug)]
pub struct ConnectionPacket {
    pub i: usize,
    pub status: ConnectionState,
}

#[derive(Clone, Debug)]
pub struct WorldPacket {
    pub pos: HashMap<usize, (i32, i32)>,
}

impl WorldPacket {
    pub fn new() -> Self {
        Self { pos: HashMap::new() }
    }
}
impl ConnectionPacket {
    pub fn new() -> Self {
        Self { i: 0, status: ConnectionState::DISCONNECTED }
    }
}