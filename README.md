# fifo_cache

![license](https://img.shields.io/badge/license-MIT%2FApache%202.0-blue.svg)

A simple rust fifo cache

If too much entries were inserted into the cache, the oldest entries would be removed from the cache to keep the cache size no bigger than the maxium size.

## Examples

```rust
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

```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
