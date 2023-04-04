use std::collections::HashMap;

use super::client::ConnectionState;

#[derive(Clone, Debug)]
pub struct ConnectionPacket {
    pub i: usize,
    pub status: ConnectionState,
}

#[derive(Debug, Default, Clone)]
pub struct TestPacket {
    pub pos: (i16, i16),
    pub plist: [(i16, i16); 10]
}

#[derive(Clone, Debug)]
pub struct WorldPacket {
    pub pos: HashMap<usize, (i32, i32)>,
}

impl Default for WorldPacket {
    fn default() -> Self {
        Self { pos: HashMap::new() }
    }
}
impl ConnectionPacket {
    pub fn new() -> Self {
        Self { i: 0, status: ConnectionState::CONNECTING }
    }
}