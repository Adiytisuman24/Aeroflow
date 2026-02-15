// AeroFlow Runtime - Arena Memory Manager
// Zero-GC, predictable latency

pub struct Arena {
    buffer: Vec<u8>,
    position: usize,
}

impl Arena {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(size),
            position: 0,
        }
    }

    /// Allocate memory for a value of type T
    /// Note: This is an MVP simplification. Real one uses raw pointers and alignment.
    pub fn alloc<T: Copy>(&mut self, value: T) -> *mut T {
        let size = std::mem::size_of::<T>();
        if self.position + size > self.buffer.capacity() {
            panic!("Arena overflow");
        }
        
        // This is a hack for the prototype
        let ptr = self.buffer.as_mut_ptr() as usize + self.position;
        unsafe {
            std::ptr::write(ptr as *mut T, value);
        }
        self.position += size;
        ptr as *mut T
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }
}
