use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};
use tide::log;

/// Default cache duration of 1 hour
pub const DEFAULT_CACHE_DURATION_SECS: u64 = 3600;

/// Generic cache entry that stores a value with a timestamp
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub value: T,
    pub timestamp: SystemTime,
}

/// Generic cache for any serializable type
pub struct Cache<K, V> 
where 
    K: Eq + Hash + Clone + ToString,
    V: Clone,
{
    cache: Mutex<HashMap<K, CacheEntry<V>>>,
    ttl: Duration,
}

impl<K, V> Cache<K, V> 
where 
    K: Eq + Hash + Clone + ToString, 
    V: Clone,
{
    /// Create a new cache with the specified TTL
    pub fn new(ttl_secs: u64) -> Self {
        Cache {
            cache: Mutex::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_secs),
        }
    }
    
    /// Create a new cache with the default TTL (1 hour)
    pub fn default() -> Self {
        Self::new(DEFAULT_CACHE_DURATION_SECS)
    }
    
    /// Get a value from the cache if it exists and is not expired
    pub fn get(&self, key: &K) -> Option<V> {
        let cache_lock = self.cache.lock().unwrap();
        
        if let Some(entry) = cache_lock.get(key) {
            if let Ok(elapsed) = entry.timestamp.elapsed() {
                if elapsed < self.ttl {
                    log::info!("Cache hit for key {}", key.to_string());
                    return Some(entry.value.clone());
                } else {
                    log::info!("Cache expired for key {}", key.to_string());
                }
            }
        } else {
            log::info!("Cache miss for key {}", key.to_string());
        }
        
        None
    }
    
    /// Insert a value into the cache
    pub fn insert(&self, key: K, value: V) {
        let mut cache_lock = self.cache.lock().unwrap();
        
        cache_lock.insert(key.clone(), CacheEntry {
            value,
            timestamp: SystemTime::now(),
        });
        
        log::info!("Cache updated for key {}", key.to_string());
    }
    
    /// Remove a key from the cache
    pub fn remove(&self, key: &K) {
        let mut cache_lock = self.cache.lock().unwrap();
        cache_lock.remove(key);
        log::info!("Cache entry removed for key {}", key.to_string());
    }
    
    /// Clear the entire cache
    pub fn clear(&self) {
        let mut cache_lock = self.cache.lock().unwrap();
        cache_lock.clear();
        log::info!("Cache cleared");
    }
}

/// Create a lazily-initialized global cache instance
#[macro_export]
macro_rules! define_global_cache {
    ($name:ident, $key_type:ty, $value_type:ty, $ttl_secs:expr) => {
        pub static $name: LazyLock<Cache<$key_type, $value_type>> = LazyLock::new(|| {
            Cache::new($ttl_secs)
        });
    };
    
    ($name:ident, $key_type:ty, $value_type:ty) => {
        pub static $name: LazyLock<Cache<$key_type, $value_type>> = LazyLock::new(|| {
            Cache::default()
        });
    };
} 