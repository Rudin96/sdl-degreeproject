pub mod networking;
pub mod datatypes;

pub mod constvalues {
    pub const MAX_PLAYERS: usize = 4;
    pub const PORT_NUMBER: u16 = 1337;
    pub const BUF_SIZE: usize = 128;
    pub const SERVER_TICK_RATE: u64 = 20;
}