extern crate fifo_cache;

use std::sync::Arc;
use fifo_cache::FifoCache;

fn main() {
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
