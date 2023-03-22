pub struct Stream {
    data: Box<u8>,
    read_ptr: Box<u8>,
    write_ptr: Box<u8>,
    end: Box<u8>
}

impl Stream {
    pub fn new() -> Stream {
        Stream {data: Box::new(0), read_ptr: Box::new(0), write_ptr: Box::new(0), end: Box::new(0) }
    }

    pub fn write<T>(&self, t: T) {
        
    }
}