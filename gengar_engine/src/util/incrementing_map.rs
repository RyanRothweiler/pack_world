use std::collections::HashMap;

pub struct IncrementingMap<T> {
    data: HashMap<usize, T>,
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

    pub fn get(&self, id: usize) -> &T {
        self.data.get(&id).unwrap()
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

        assert_eq!(*holder.get(id), 10.0);
        assert_eq!(*holder.get(second), 123.0);
    }
}
