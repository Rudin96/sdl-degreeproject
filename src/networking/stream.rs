#[derive(Debug, Default)]
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

    pub fn write<T>(&mut self, data: T) {
        let p: *const T = &data;
        *self.write_ptr += p as u8;
        *self.data += p as u8;
        println!("data looks like: {:?}", *self.data);
    }

    pub fn read<T: std::default::Default>(&self) -> &T {
        let size = std::mem::size_of::<T>();
        
        let r = *self.data as *mut T;
        let dr = unsafe { &*r };
        dr
    }
}