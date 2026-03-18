// ========================================
// Solution: RefCell Interior Mutability
// ========================================

use std::cell::RefCell;
use std::collections::HashMap;

struct Cache {
    store: RefCell<HashMap<String, String>>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            store: RefCell::new(HashMap::new()),
        }
    }

    fn get_or_compute<F>(&self, key: &str, compute: F) -> String
    where
        F: FnOnce() -> String,
    {
        // First, check if the key exists using an immutable borrow.
        // We scope the borrow so it's dropped before we potentially borrow_mut.
        {
            let store = self.store.borrow();
            if let Some(value) = store.get(key) {
                return value.clone();
            }
        }
        // Key not found -- compute the value and insert it.
        // borrow_mut() gives us a RefMut<HashMap<...>> that we can write to.
        let value = compute();
        self.store.borrow_mut().insert(key.to_string(), value.clone());
        value
    }

    fn len(&self) -> usize {
        // borrow() returns a Ref<HashMap> -- shared/immutable access.
        self.store.borrow().len()
    }

    fn contains(&self, key: &str) -> bool {
        self.store.borrow().contains_key(key)
    }

    fn clear(&self) {
        // borrow_mut() returns a RefMut<HashMap> -- exclusive/mutable access.
        self.store.borrow_mut().clear();
    }
}

struct CallCounter {
    count: RefCell<u32>,
}

impl CallCounter {
    fn new() -> CallCounter {
        CallCounter {
            count: RefCell::new(0),
        }
    }

    fn increment(&self) {
        // Interior mutability: mutate count through a shared &self reference.
        *self.count.borrow_mut() += 1;
    }

    fn get(&self) -> u32 {
        // Dereference the Ref<u32> to copy out the value.
        *self.count.borrow()
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

        let val1 = cache.get_or_compute("key1", || {
            counter.increment();
            "computed".to_string()
        });

        let val2 = cache.get_or_compute("key1", || {
            counter.increment();
            "should not see this".to_string()
        });

        assert_eq!(val1, "computed");
        assert_eq!(val2, "computed");
        assert_eq!(counter.get(), 1);
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
        let cache = Cache::new();
        let cache_ref: &Cache = &cache;

        cache_ref.get_or_compute("through_ref", || "works!".to_string());
        assert_eq!(cache_ref.len(), 1);
        assert!(cache_ref.contains("through_ref"));
    }
}
