pub struct PosPacket {
    i: usize,
    x: i16,
    y: i16,
    z: i16
}

pub struct WorldPacket {
    i: usize,
    t: HashMap<i8, Vector2>
}

pub struct ConnectionPacket {
    i: usize,
    status: u8,
}

pub trait Write {
    fn WriteToStream<T>(val: &T, buffer: &Buffer);
}

pub struct Schema {
    player: Player,

}