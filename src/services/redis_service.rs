use redis::{aio::ConnectionManager, AsyncCommands, RedisError};
use serde::{de::DeserializeOwned, Serialize};

/// Redis cache service for managing cached data
#[derive(Clone)]
pub struct RedisService {
    client: ConnectionManager,
    default_ttl: usize, // Time-to-live in seconds
}

impl RedisService {
    /// Create a new Redis service
    pub async fn new(redis_url: &str, default_ttl: usize) -> Result<Self, RedisError> {
        let client = redis::Client::open(redis_url)?;
        let connection_manager = ConnectionManager::new(client).await?;
        
        tracing::info!("✅ Redis connection established");
        
        Ok(Self {
            client: connection_manager,
            default_ttl,
        })
    }

    /// Get a value from cache
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RedisError> {
        let mut conn = self.client.clone();
        let value: Option<String> = conn.get(key).await?;
        
        match value {
            Some(json) => {
                match serde_json::from_str(&json) {
                    Ok(data) => {
                        tracing::debug!("🎯 Cache HIT: {}", key);
                        Ok(Some(data))
                    }
                    Err(e) => {
                        tracing::warn!("Failed to deserialize cached value for key {}: {}", key, e);
                        Ok(None)
                    }
                }
            }
            None => {
                tracing::debug!("❌ Cache MISS: {}", key);
                Ok(None)
            }
        }
    }

    /// Set a value in cache with default TTL
    pub async fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), RedisError> {
        self.set_with_ttl(key, value, self.default_ttl).await
    }

    /// Set a value in cache with custom TTL
    pub async fn set_with_ttl<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: usize,
    ) -> Result<(), RedisError> {
        let mut conn = self.client.clone();
        let json = serde_json::to_string(value)
            .map_err(|e| RedisError::from((redis::ErrorKind::IoError, "serialization error", e.to_string())))?;
        
        conn.set_ex::<_, _, ()>(key, json, ttl_seconds as u64).await?;
        Ok(())
    }

    /// Delete a key from cache
    pub async fn delete(&self, key: &str) -> Result<(), RedisError> {
        let mut conn = self.client.clone();
        conn.del::<_, ()>(key).await?;
        tracing::debug!("🗑️  Cache DELETE: {}", key);
        Ok(())
    }

    /// Delete multiple keys matching a pattern
    pub async fn delete_pattern(&self, pattern: &str) -> Result<(), RedisError> {
        let mut conn = self.client.clone();
        let keys: Vec<String> = conn.keys(pattern).await?;
        
        if !keys.is_empty() {
            conn.del::<_, ()>(&keys).await?;
            tracing::debug!("🗑️  Cache DELETE pattern: {} ({} keys)", pattern, keys.len());
        }
        
        Ok(())
    }

    /// Check if a key exists
    pub async fn exists(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.client.clone();
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }

    /// Get TTL for a key
    pub async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
        let mut conn = self.client.clone();
        let ttl: i64 = conn.ttl(key).await?;
        Ok(ttl)
    }

    /// Clear all cache (use with caution!)
    pub async fn flush_all(&self) -> Result<(), RedisError> {
        let mut conn = self.client.clone();
        redis::cmd("FLUSHDB").query_async::<_, ()>(&mut conn).await?;
        tracing::warn!("🧹 Cache FLUSHED: All keys deleted");
        Ok(())
    }
}

/// Cache key builder for consistent key naming
pub struct CacheKey;

impl CacheKey {
    /// Generate cache key for rooms list
    pub fn rooms_list() -> String {
        "rooms:list".to_string()
    }

    /// Generate cache key for a specific room
    pub fn room(room_id: i32) -> String {
        format!("room:{}", room_id)
    }

    /// Generate cache key for room messages
    pub fn room_messages(room_id: i32) -> String {
        format!("room:{}:messages", room_id)
    }

    /// Generate cache key for user
    pub fn user(user_id: i32) -> String {
        format!("user:{}", user_id)
    }

    /// Pattern to match all room keys
    pub fn rooms_pattern() -> String {
        "room*".to_string()
    }
}
