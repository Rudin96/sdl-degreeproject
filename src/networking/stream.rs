use std::{mem::size_of};
use std::fmt::Debug;

use crate::constvalues::BUF_SIZE;

#[derive(Clone)]
pub struct Stream {
    data: Box<[u8; BUF_SIZE]>,
    size: usize,
    index: usize,
}

impl Stream {
    pub fn new() -> Stream {
        Stream {
            data: Box::new([0; BUF_SIZE]),
            size: 0,
            index: 0,
        }
    }

    pub fn writetobuffer(&mut self, bytes: &[u8]) {
        self.data.copy_from_slice(bytes);
    }

    pub fn clear(&mut self) {
        self.data = Box::new([0; BUF_SIZE]);
        self.index = 0;
        self.size = 0;
    }

    pub fn getbuffer(&self) -> Box<[u8]> {
        self.data.clone()
    }

    pub fn write<T>(&mut self, val: T)
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
        T: Default
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