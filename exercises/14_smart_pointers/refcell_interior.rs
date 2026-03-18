// ========================================
// Exercise: RefCell Interior Mutability (ImplementFromScratch)
// ========================================
// Difficulty: Intermediate
// Module: 14 - Smart Pointers
//
// CONCEPT:
// RefCell<T> provides "interior mutability" -- it lets you mutate data even
// when there are immutable references to the RefCell itself. The borrow rules
// are enforced at RUNTIME instead of compile time:
//   - refcell.borrow()     -> returns Ref<T>    (shared/immutable borrow)
//   - refcell.borrow_mut() -> returns RefMut<T> (exclusive/mutable borrow)
//   - If you violate the rules (e.g., borrow_mut while already borrowed),
//     the program will panic at runtime.
//
// Common pattern: wrapping a HashMap in RefCell to create a lazy cache
// that can be populated through an immutable reference.
//
// YOUR TASK:
// Implement the Cache struct and its methods. The cache stores lazily
// computed values: the first time you request a key, it computes the value
// using a provided function and caches it. Subsequent requests return the
// cached value.
// ========================================

use std::cell::RefCell;
use std::collections::HashMap;

/// A cache that lazily computes and stores values.
/// Uses RefCell for interior mutability so that `get_or_compute` can
/// modify the internal cache through a shared (&self) reference.
struct Cache {
    store: RefCell<HashMap<String, String>>,
}

impl Cache {
    /// Creates a new, empty Cache.
    fn new() -> Cache {
        todo!()
    }

    /// Returns the cached value for `key` if it exists, otherwise calls
    /// `compute` to generate the value, stores it in the cache, and returns it.
    ///
    /// Note: This takes &self (not &mut self!) -- that's the whole point of
    /// RefCell. The caller doesn't need a mutable reference to populate the cache.
    fn get_or_compute<F>(&self, key: &str, compute: F) -> String
    where
        F: FnOnce() -> String,
    {
        todo!()
    }

    /// Returns the number of entries currently in the cache.
    /// Also uses &self (immutable reference) with RefCell::borrow().
    fn len(&self) -> usize {
        todo!()
    }

    /// Returns true if the cache contains the given key.
    fn contains(&self, key: &str) -> bool {
        todo!()
    }

    /// Clears all entries from the cache.
    /// Even though this modifies internal state, it takes &self thanks to RefCell.
    fn clear(&self) {
        todo!()
    }
}

/// A counter that tracks how many times it has been called.
/// Uses RefCell to allow counting through a shared reference.
struct CallCounter {
    count: RefCell<u32>,
}

impl CallCounter {
    /// Creates a new CallCounter starting at zero.
    fn new() -> CallCounter {
        todo!()
    }

    /// Increments the counter by 1. Takes &self, not &mut self.
    fn increment(&self) {
        todo!()
    }

    /// Returns the current count.
    fn get(&self) -> u32 {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_new_is_empty() {
        let cache = Cache::new();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_get_or_compute_stores_value() {
        let cache = Cache::new();
        let result = cache.get_or_compute("greeting", || "hello world".to_string());
        assert_eq!(result, "hello world");
        assert_eq!(cache.len(), 1);
        assert!(cache.contains("greeting"));
    }

    #[test]
    fn test_cache_returns_cached_value() {
        let cache = Cache::new();
        let counter = CallCounter::new();

        // First call -- compute is invoked
        let val1 = cache.get_or_compute("key1", || {
            counter.increment();
            "computed".to_string()
        });

        // Second call -- compute should NOT be invoked (cached)
        let val2 = cache.get_or_compute("key1", || {
            counter.increment();
            "should not see this".to_string()
        });

        assert_eq!(val1, "computed");
        assert_eq!(val2, "computed"); // Same value, from cache
        assert_eq!(counter.get(), 1); // Compute was only called once
    }

    #[test]
    fn test_cache_multiple_keys() {
        let cache = Cache::new();

        cache.get_or_compute("a", || "alpha".to_string());
        cache.get_or_compute("b", || "beta".to_string());
        cache.get_or_compute("c", || "gamma".to_string());

        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get_or_compute("a", || panic!("should be cached")), "alpha");
        assert_eq!(cache.get_or_compute("b", || panic!("should be cached")), "beta");
        assert_eq!(cache.get_or_compute("c", || panic!("should be cached")), "gamma");
    }

    #[test]
    fn test_cache_clear() {
        let cache = Cache::new();
        cache.get_or_compute("x", || "value".to_string());
        assert_eq!(cache.len(), 1);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(!cache.contains("x"));
    }

    #[test]
    fn test_cache_recompute_after_clear() {
        let cache = Cache::new();
        cache.get_or_compute("key", || "first".to_string());
        cache.clear();

        // After clear, the value must be recomputed
        let val = cache.get_or_compute("key", || "second".to_string());
        assert_eq!(val, "second");
    }

    #[test]
    fn test_call_counter() {
        let counter = CallCounter::new();
        assert_eq!(counter.get(), 0);

        counter.increment();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 3);
    }

    #[test]
    fn test_cache_shared_reference() {
        // The whole point: we can use &cache (not &mut cache) to modify contents.
        let cache = Cache::new();
        let cache_ref: &Cache = &cache;

        cache_ref.get_or_compute("through_ref", || "works!".to_string());
        assert_eq!(cache_ref.len(), 1);
        assert!(cache_ref.contains("through_ref"));
    }
}
