use std::collections::HashMap;

use super::client::ConnectionState;

#[derive(Default, Clone, Copy, Debug)]
pub struct ConnectionPacket {
    pub i: usize,
    pub status: ConnectionState,
}

#[derive(Debug, Default, Clone)]
pub struct TestPacket {
    pub pos: (i16, i16),
    pub plist: [(i16, i16); 10]
}

#[derive(Clone, Copy, Debug)]
pub struct WorldPacket {
    pub pos: Vec<(usize, i16, i16)>,
}

impl Default for WorldPacket {
    fn default() -> Self {
        Self { pos: Vec::new() }
    }
}