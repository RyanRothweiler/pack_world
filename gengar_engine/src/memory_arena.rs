use std::{
    alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout},
    cell::RefCell,
    ptr::NonNull,
};

pub struct MemoryArena {
    block: RefCell<MemBlock>,
}

struct MemBlock {
    memory: *mut u8,
    offset: usize,
    size_bytes: usize,
}

impl MemoryArena {
    // Create a new arena with a given size
    pub fn new(bytes: usize) -> Self {
        let layout = Layout::from_size_align(bytes, 8).unwrap();
        let ptr: *mut u8 = unsafe { alloc_zeroed(layout) };
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        Self {
            block: RefCell::new(MemBlock {
                memory: ptr,
                offset: 0,
                size_bytes: bytes,
            }),
        }
    }

    // Allocate raw memory for a custom type T
    pub fn alloc<T>(&self, initial_val: T) -> &mut T {
        let mut mem_block = self.block.borrow_mut();

        // Calculate the required layout for the type
        let layout = Layout::new::<T>();

        // Ensure there’s enough space left in the arena
        if mem_block.offset + layout.size() > mem_block.size_bytes {
            panic!("Out of memory!");
        }

        // Get the raw pointer to the allocated memory
        let ptr = unsafe { mem_block.memory.add(mem_block.offset) as *mut T };

        // Update the offset to reflect the new allocation
        mem_block.offset += layout.size();

        unsafe {
            let owned_ptr = NonNull::new_unchecked(ptr);

            owned_ptr.as_ptr().write(initial_val);
            return &mut *owned_ptr.as_ptr();
        }
    }

    pub fn reset(&self) {
        self.block.borrow_mut().offset = 0;
    }

    pub fn bytes_avail(&self) -> usize {
        let mem_block = self.block.borrow();
        mem_block.size_bytes - mem_block.offset
    }

    pub fn print_diag(&self) {
        let mem_block = self.block.borrow();
        let perc_allocated: f64 = (mem_block.offset as f64 / mem_block.size_bytes as f64) * 100.0;
        println!(
            "Allocated {} bytes. {:.2}% allocated",
            mem_block.offset, perc_allocated
        );
    }
}

mod test {
    use super::*;

    struct TempGameState {
        pub val: i32,
    }

    impl TempGameState {
        pub fn new() -> Self {
            Self { val: 0 }
        }
    }

    #[test]
    fn alloc() {
        let arena_size = 8;
        let mut mem = MemoryArena::new(arena_size);
        let tgs = mem.alloc(TempGameState::new());

        let struct_bytes = std::mem::size_of::<TempGameState>();
        assert_eq!(mem.bytes_avail(), arena_size - struct_bytes);
    }

    #[test]
    fn access() {
        let arena_size = 1024;
        let mem = MemoryArena::new(arena_size);

        let tgs_one = mem.alloc(TempGameState::new());
        tgs_one.val = 100;

        let tgs_two = mem.alloc(TempGameState::new());
        tgs_two.val = 314;

        let tgs_three = mem.alloc(TempGameState::new());

        assert_eq!(tgs_one.val, 100);
        assert_eq!(tgs_two.val, 314);
        assert_eq!(tgs_three.val, 0);
    }
}
