pub struct Packet{
    x: i32,
    y: i32,
    z: i32
}

pub trait Write {
    fn write(buffer: &Buffer);
    fn write_int(buffer: &Buffer);
}

pub trait Read {
    fn read(buffer: &Buffer);
    fn read_int(buffer: &Buffer);
}