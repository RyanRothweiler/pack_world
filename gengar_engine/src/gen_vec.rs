use crate::memory_arena::*;

pub struct GenVec<T>
where
    T: Copy,
{
    pub mem: *mut T,
    pub len: usize,
}

impl<T: Copy> GenVec<T> {
    pub fn new(input: &Vec<T>, arena: &MemoryArena) -> Self {
        let ret = Self {
            mem: arena.alloc_array::<T>(input.len()),
            len: input.len(),
        };

        for i in 0..input.len() {
            unsafe {
                *ret.mem.offset(i as isize) = input[i];
            }
        }

        ret
    }

    pub fn get(&self, idx: usize) -> Option<&mut T> {
        if idx >= self.len {
            return None;
        }

        unsafe {
            return Some(&mut *self.mem.offset(idx as isize));
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn new() {
        let memory = MemoryArena::new(64);

        let mut vec: Vec<i32> = vec![];
        vec.push(100);
        vec.push(1234);
        vec.push(99);

        let gv: GenVec<i32> = GenVec::new(&vec, &memory);
        assert_eq!(memory.bytes_used(), 4 * 3);

        assert_eq!(*gv.get(0).unwrap(), 100);
        assert_eq!(*gv.get(1).unwrap(), 1234);
        assert_eq!(*gv.get(2).unwrap(), 99);
        assert_eq!(gv.get(3), None);
    }
}
