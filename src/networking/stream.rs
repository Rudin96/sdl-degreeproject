use crate::constvalues::BUF_SIZE;

#[derive(Debug)]
pub struct Stream {
    data: Box<[u8; BUF_SIZE]>,
    rindex: usize,
    windex: usize,
}

impl Stream {
    pub fn new() -> Stream {
        Stream {data: Box::new([0; BUF_SIZE]), rindex: 0, windex: 0 }
    }

    pub fn write<T>(&mut self, data: T) {
        let dptr: *const T = &data;
        self.data[self.windex] = dptr as u8;
        self.windex += 1;
    }

    pub fn read<T: std::default::Default + std::fmt::Debug>(&mut self) -> T {
        println!("Data looks like: {:?}", self.data);
        let ddref = self.data[self.rindex];
        self.rindex += 1;
        let b: Box<T> = Box::new();
        T::default()
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }
}