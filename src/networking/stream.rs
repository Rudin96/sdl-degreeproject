use std::{mem::size_of};

const BUFFER_SIZE: usize = 128;

pub struct Stream {
    data: Box<[u8; BUFFER_SIZE]>,
    size: usize,
    index: usize,
}

impl Stream {
    pub fn new() -> Stream {
        Stream {
            data: Box::new([0; BUFFER_SIZE]),
            size: 0,
            index: 0,
        }
    }

    pub fn newbuf(&mut self) {
        self.data = Box::new([0; BUFFER_SIZE]);
        self.size = 0;
        self.index = 0;
    }

    pub fn clear(&mut self) {
        self.data = Box::new([0; BUFFER_SIZE]);
        self.index = 0;
        self.size = 0;
    }

    pub fn write<T>(&mut self, val: T)
    where
        T: Copy,
    {
        unsafe {
            let data_ptr = self.data.as_mut_ptr().add(self.index);
            let val_ptr = &val as *const T as *const u8;
            std::ptr::copy_nonoverlapping(val_ptr, data_ptr, size_of::<T>());
        }
        self.index += size_of::<T>();
    }

    pub fn read<T>(&mut self) -> T
    where
        T: Copy,
        T: Default,
    {
        let mut val: T = T::default();
        unsafe {
            let data_ptr = self.data.as_ptr().add(self.size);
            let val_ptr = &mut val as *mut T as *mut u8;
            std::ptr::copy_nonoverlapping(data_ptr, val_ptr, size_of::<T>());
        }
        self.size += size_of::<T>();
        val
    }
}

#[derive(Clone, Copy)]
struct Test {
    a: f32,
    b: f32,
    c: f32,
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Test {
    fn default() -> Test {
        Test {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

struct GameObject {
    x: f32,
    y: f32,
    z: f32,
    test: Test,
}

impl GameObject {
    fn serialize(&mut self, b: &mut Stream) {
        b.write(self.test);
    }

    fn deserialize(&mut self, b: &mut Stream) {
        self.test = b.read();
    }
}