use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;

/// A simple fifo cache.
///
/// If too much entries were inserted into the cache,
/// the oldest entries would be removed from the cache to
/// keep the cache size no bigger than the maxium size.
#[derive(Debug)]
pub struct FifoCache<K: Eq + Hash, V> {
    size: usize,
    fifo: VecDeque<Rc<K>>,
    map: HashMap<Rc<K>, Arc<V>>,
}

impl<K: Eq + Hash, V> FifoCache<K, V> {
    /// Create a FifoCache with the size specified.
    pub fn new(size: usize) -> Self {
        assert_eq!(size > 0, true);
        FifoCache {
            size: size,
            fifo: VecDeque::with_capacity(size),
            map: HashMap::with_capacity(size),
        }
    }

    /// Gets a reference to the value in the entry.
    /// If returns None it means that the entry is not inserted into the cache
    /// or it's removed due to insert too much entries.
    pub fn get(&self, k: &K) -> Option<Arc<V>> {
        self.map.get(k).map(|v| v.clone())
    }

    /// insert a KV pair into the FifoCache
    /// If the FifoCache did not have this key present, None is returned.
    /// If the FifoCache did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    /// If the size of FifoCache is larger than the maxed size, the oldest entries
    /// would be removed.
    pub fn insert(&mut self, k: K, v: V) -> Option<Arc<V>> {
        use std::collections::hash_map::Entry;
        let k = Rc::new(k);
        let v = Arc::new(v);

        // need to update the entry
        if let Entry::Occupied(mut entry) = self.map.entry(k.clone()) {
            return Some(entry.insert(v));
        }

        // If the capacity is exceeded, remove old entry from the cache
        while self.fifo.len() > self.size - 1 {
            let entry = self
                .fifo
                .pop_front()
                .expect("failed to pop_front from FifoCache");
            self.map
                .remove(&entry)
                .expect("failed to remove from FifoCache");
        }

        // need to insert the entry
        let k1 = k.clone();
        self.fifo.push_back(k1);
        self.map.insert(k, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut cache = FifoCache::<usize, usize>::new(2);
        // test empty cache
        assert_eq!(cache.get(&0), None);
        // insert entry
        assert_eq!(cache.insert(1, 1), None);
        // insert entry
        assert_eq!(cache.insert(2, 2), None);
        // this will update the entry
        assert_eq!(cache.insert(2, 4), Some(Arc::new(2)));
        // this will flush out the first entry
        assert_eq!(cache.insert(3, 3), None);
        // entry 1 is flushed out
        assert_eq!(cache.get(&1), None);
    }
}
