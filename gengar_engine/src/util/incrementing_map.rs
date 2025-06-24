use std::collections::HashMap;

/// A hashmap with a monotonically increasing id.
/// Insert into the map and get the id back. Use the ID as a handle into the map.
pub struct IncrementingMap<T> {
    // should add iterator so that this pub can be removed
    pub data: HashMap<usize, T>,

    id: usize,
}

impl<T> IncrementingMap<T> {
    pub fn new() -> IncrementingMap<T> {
        Self {
            data: HashMap::new(),
            id: 0,
        }
    }

    pub fn push(&mut self, item: T) -> usize {
        let id = self.data.len();
        self.data.insert(id, item);
        id
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.data.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.data.get_mut(&id)
    }

    pub fn remove(&mut self, id: usize) {
        self.data.remove(&id);
    }
}

mod test {
    use super::*;

    #[test]
    fn working() {
        let mut holder: IncrementingMap<f64> = IncrementingMap {
            data: HashMap::new(),
            id: 0,
        };

        let id = holder.push(10.0);
        let second = holder.push(123.0);

        assert_eq!(*holder.get(id).unwrap(), 10.0);
        assert_eq!(*holder.get(second).unwrap(), 123.0);
    }
}
