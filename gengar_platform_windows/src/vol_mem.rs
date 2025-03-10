#![allow(
    unused_imports,
    clippy::all,
    dead_code,
    unused_variables,
    unused_macros,
    static_mut_refs,
    unused_mut,
    unreachable_code
)]

use std::{
    alloc::{GlobalAlloc, Layout, System},
    backtrace::Backtrace,
    collections::HashMap,
    io::Write,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        LazyLock, Mutex, OnceLock,
    },
};

/// Total number of allowed trackers total in the entire program.
pub const TRACKER_SLOTS_COUNT: usize = 32;

/// Get the current function name
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }};
}

/// Track the memory in this block
#[macro_export]
macro_rules! memory_track {
    ($x:expr) => {
        let lemming = MemoryBlockLemming::new($x.into());
        // This unsafe must come after, otherwise the memory used by the tracker will be counted.

        unsafe {
            // todo use the right id from the lemming
            CURRENT_TRACKERS[CURRENT_TRACKERS_LEN] = lemming.block_id;

            CURRENT_TRACKERS_LEN += 1;
            assert!(CURRENT_TRACKERS_LEN < TRACKER_SLOTS_COUNT);
        }
    };
}

pub struct TrackingAlloc;

pub struct TrackingBlock {
    // pub display_id: String,
    pub tracker_index: i32,
}

pub static TRACKING_BLOCKS: LazyLock<Mutex<HashMap<String, TrackingBlock>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub static NEXT_BLOCK_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
pub static BLOCK_IDS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));

#[derive(Copy, Clone)]
pub struct BlockInfo {
    // in bytes
    pub allocated_memory: i64,
}

pub static mut TRACKERS: [BlockInfo; TRACKER_SLOTS_COUNT] = [BlockInfo {
    allocated_memory: 0,
}; TRACKER_SLOTS_COUNT];

pub static mut CURRENT_TRACKERS_LEN: usize = 0;
pub static mut CURRENT_TRACKERS: [i32; TRACKER_SLOTS_COUNT] = [-1; TRACKER_SLOTS_COUNT];

unsafe impl GlobalAlloc for TrackingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            // Update memory for all living trackers
            for i in 0..CURRENT_TRACKERS_LEN {
                TRACKERS[CURRENT_TRACKERS[i] as usize].allocated_memory += layout.size() as i64;
            }
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);

        // Update memory for all living trackers
        for i in 0..CURRENT_TRACKERS_LEN {
            TRACKERS[CURRENT_TRACKERS[i] as usize].allocated_memory -= layout.size() as i64;
        }
    }
}

/*
#[global_allocator]
static A: TrackingAlloc = TrackingAlloc;
*/

pub struct MemoryBlockLemming {
    pub block_id: i32,
}

impl MemoryBlockLemming {
    pub fn new(id: String) -> Self {
        // println!("starting track {:?}", id);

        let block_ids: &mut HashMap<String, TrackingBlock> = &mut TRACKING_BLOCKS.lock().unwrap();
        if !block_ids.contains_key(&id) {
            let mut next_block_id = NEXT_BLOCK_ID.lock().unwrap();

            let new_block = TrackingBlock {
                tracker_index: *next_block_id,
            };
            block_ids.insert(id.clone(), new_block);

            *next_block_id += 1;
        }

        let this_block_id = block_ids
            .get(&id)
            .expect("Block is added above")
            .tracker_index;

        Self {
            block_id: this_block_id,
        }
    }
}

impl Drop for MemoryBlockLemming {
    fn drop(&mut self) {
        unsafe {
            CURRENT_TRACKERS_LEN -= 1;
        }
    }
}
