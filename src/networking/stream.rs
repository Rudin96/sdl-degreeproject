use std::{mem::{size_of, size_of_val}, fmt::Debug};

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
            *((self.data.as_mut_ptr().add(self.index)) as *mut T) = val;
            // let data_ptr = self.data.as_mut_ptr().add(self.index);
            // let val_ptr = &val as *const T as *const u8;
            // std::ptr::copy_nonoverlapping(val_ptr, data_ptr, size_of::<T>());
            self.index += size_of::<T>();
        }
    }
    
    pub fn read<T>(&mut self) -> &T
    {
        unsafe {
            // let data_ptr = self.data.as_mut_ptr().add(self.size);
            // let val: &T = &*(data_ptr as *mut T);
            // println!("READER: val ref is: {:#?}", val);
            // self.size += 1;
            // val
            let x = &*((self.data.as_mut_ptr().add(self.size)) as *mut T);
            self.size += size_of::<T>();
            x
        }
    }

    pub fn readfrombuffer<T>(&mut self, buf: &[u8]) -> &T {
        unsafe {
            &*(buf.as_ptr() as *const T)
        }
    }
}