// std::Alloc uses the System allocator by default.

mod global_alloc_basic {
    use std::alloc::{GlobalAlloc, Layout, System};
    // What this does?
    // This code creates a custom allocator that uses the System allocator to allocate and deallocate memory.
    // Size of the memory block is determined by the layout parameter.
    // here we are allocating total 4 bytes of memory for i32 type.
    struct MyAllocator;

    unsafe impl GlobalAlloc for MyAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            System.alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            System.dealloc(ptr, layout)
        }
    }

    pub fn test() {
        let my_allocator = MyAllocator;
        let layout = Layout::new::<i32>();
        let ptr = unsafe { my_allocator.alloc(layout) };
        unsafe { my_allocator.dealloc(ptr, layout) };
    }
}


mod global_alloc_struct {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::ptr;

    // What this does?
    // This code creates a custom allocator that uses the System allocator to allocate and deallocate memory.
    // Size of the memory block is determined by the layout parameter.
    // here we are allocating total 1024 bytes of memory for i32 type.


    struct MyAllocator {
        memory: *mut u8,
        size: usize,
    }

    impl MyAllocator {
        fn new(size: usize) -> Self {
            let layout = Layout::from_size_align(size, 1).unwrap();
            let memory = unsafe { System.alloc(layout) };
            MyAllocator { memory, size }
        }
    }

    unsafe impl GlobalAlloc for MyAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            if layout.size() <= self.size {
                self.memory
            } else {
                ptr::null_mut()
            }
        }

        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
            // Do nothing
        }
    }

    impl Drop for MyAllocator {
        fn drop(&mut self) {
            let layout = Layout::from_size_align(self.size, 1).unwrap();
            unsafe { System.dealloc(self.memory, layout) }; // this code deallocates the memory block
        }
    }

    pub fn test() {
        let my_allocator = MyAllocator::new(1024);
        let layout = Layout::new::<i32>();
        let ptr = unsafe { my_allocator.alloc(layout) };
        unsafe { my_allocator.dealloc(ptr, layout) }; // this code does not deallocate the memory block
    }
}


pub fn test() {
    global_alloc_basic::test();
    global_alloc_struct::test();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        test();
    }
}