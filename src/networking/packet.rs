use std::collections::HashMap;

use super::client::ConnectionState;

#[derive(Default, Clone, Copy, Debug)]
pub struct ConnectionPacket {
    pub i: usize,
    pub status: ConnectionState,
}

#[derive(Clone, Debug)]
pub struct WorldPacket {
    pub pos: HashMap<usize, (i16, i16)>,
}

impl Default for WorldPacket {
    fn default() -> Self {
        Self { pos: HashMap::new() }
    }
}