use std::{collections::HashMap, mem::size_of};

use crate::constvalues::MAX_PLAYERS;

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

impl Serialize for WorldPacket {
    fn to_bytes (&self) -> &[u8] {
        
    }
}

impl Deserialize for WorldPacket {
    fn into<T: Clone>(data: &[u8]) -> T {
        unsafe {
            let x = &*(data.as_mut_ptr().add(size_of::<T>()) as *mut T);
            x.clone()
        }
    }
}

pub trait Serialize {
    fn to_bytes (&self) -> &[u8];
}

pub trait Deserialize {
    fn into<T: Clone>(data: &[u8]) -> T;
}