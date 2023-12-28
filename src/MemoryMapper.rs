use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

struct MemoryMapper<T> {
    data: *mut T,
    size: usize,
}

impl<T> MemoryMapper<T> {
    
    fn new(size: usize) -> Self {
        let layout: Layout = Layout::array::<T>(size).unwrap();
        let data: *mut u8 = unsafe { alloc(layout) };

        if data.is_null() {
            panic!("Memory allocation failed");
        }

        MemoryMapper { data: data as *mut T, size }
    }

    
    fn write_data(&mut self, offset: usize, data: &[T]) {
        self.perform_operation(offset, data, |dest: *mut T, src: *const T, len: usize| {
            unsafe { ptr::copy_nonoverlapping(src, dest, len) };
        });
    }

    
    fn read_data(&self, offset: usize, size: usize) -> Vec<T> {
        let mut result: Vec<T> = Vec::with_capacity(size);
        unsafe {
            result.set_len(size);
        }

        self.perform_operation(offset, &result, |dest: *mut T, src: *const T, len: usize| {
            unsafe { ptr::copy_nonoverlapping(src, dest, len) };
        });

        result
    }

    
    fn perform_operation<F>(&self, offset: usize, data: &[T], operation: F)
    where
        F: Fn(*mut T, *const T, usize),
    {
        if offset + data.len() > self.size {
            panic!("Attempted to access beyond the allocated memory");
        }

        unsafe {
            let dest: *mut T = self.data.add(offset);
            let src: *const T = data.as_ptr();
            let len: usize = data.len();
            operation(dest, src, len);
        }
    }
}   

impl<T> Drop for MemoryMapper<T> {
    
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.data as *mut u8,
                Layout::array::<T>(self.size).unwrap(),
            );
        }
    }
}

fn main() {
    
    let mut mapper = MemoryMapper::<u32>::new(4);

    let data_to_write = [1, 2, 3, 4];
    mapper.write_data(0, &data_to_write);

    let read_data = mapper.read_data(0, data_to_write.len());
    println!("Read data: {:?}", read_data);
}
