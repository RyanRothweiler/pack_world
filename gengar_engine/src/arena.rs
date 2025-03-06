use std::{
    alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout},
    cell::Cell,
    ptr::NonNull,
};

pub struct Arena {
    memory: *mut u8,
    offset: usize,
    size_bytes: usize,
}

/*
struct MemBlock {
    memory: *mut u8,
    offset: usize,
    size_bytes: usize,
}
*/

impl Arena {
    // Create a new arena with a given size
    pub fn new(bytes: usize) -> Self {
        let layout = Layout::from_size_align(bytes, 8).unwrap();
        let ptr: *mut u8 = unsafe { alloc_zeroed(layout) };
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        Arena {
            memory: ptr,
            offset: 0,
            size_bytes: bytes,
        }
    }

    // Allocate raw memory for a custom type T
    pub fn alloc<T>(&mut self, initial_val: T) -> &mut T {
        // Calculate the required layout for the type
        let layout = Layout::new::<T>();

        // Ensure there’s enough space left in the arena
        if self.offset + layout.size() > self.size_bytes {
            panic!("Out of memory!");
        }

        // Get the raw pointer to the allocated memory
        let ptr = unsafe { self.memory.add(self.offset) as *mut T };

        // Update the offset to reflect the new allocation
        self.offset += layout.size();

        unsafe {
            let owned_ptr = NonNull::new_unchecked(ptr);

            owned_ptr.as_ptr().write(initial_val);
            return &mut *owned_ptr.as_ptr();
        }
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn bytes_avail(&self) -> usize {
        self.size_bytes - self.offset
    }

    pub fn print_diag(&self) {
        let perc_allocated: f64 = (self.offset as f64 / self.size_bytes as f64) * 100.0;
        println!(
            "Allocated {} bytes. {:.2}% allocated",
            self.offset, perc_allocated
        );
    }
}

#[derive(Debug)]
struct GameState {
    val: i32,
}

/*
fn test() {
    let mut arena = Arena::new(1024);

    // Allocate and initialize GameState
    let gs_one = arena.alloc(GameState { val: 0 });
    gs_one.val = 1; // This is fine, since gs_one is a mutable reference

    let gs_two = arena.alloc(GameState { val: 0 });
    gs_two.val = 1; // This is also fine

    gs_one.val = 2; // Modify gs_one again
    println!("{:?}", gs_one); // GameState { val: 2 }
    println!("{:?}", gs_two); // GameState { val: 1 }
}
*/

/*
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
        let mut mem = MemoryArena::new(arena_size);

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
*/
