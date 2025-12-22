pub(crate) mod error;

use std::collections::{HashMap};
use std::sync::{Arc};
use mockall::automock;
use tokio::sync::{RwLock};
use crate::repository::error::DatabaseError;
use crate::repository::error::DatabaseError::{AlreadyExists, NotFound, WriteBlocked};

pub(crate) type SharedRepository = Arc<dyn RepositoryApi>;

pub(crate) struct Repository {
    data: RwLock<HashMap<u32, RwLock<Vec<u8>>>>,
}

#[async_trait::async_trait]
pub(crate) trait RepositoryApi: Send + Sync {
    async fn get(&self, id: u32) -> Option<Vec<u8>>;
    async fn set(&self, id: u32, data: Vec<u8>) -> Result<(), DatabaseError>;
    async fn insert(&self, id: u32, data: Vec<u8>) -> Result<(), DatabaseError>;
    async fn remove(&self, id: u32) -> Result<(), DatabaseError>;
}

impl Repository {
    pub(crate) fn new() -> Self {
        Repository {
            data: RwLock::new(HashMap::new()),
        }
    }
}

#[automock]
#[async_trait::async_trait]
impl RepositoryApi for Repository {
    async fn get(&self, id: u32) -> Option<Vec<u8>> {
        let hash_map_guard = self.data.read().await;
        let rw_lock = hash_map_guard.get(&id)?;
        let data_guard = rw_lock.read().await;
        Some(data_guard.clone())
    }

    /// Returns a Result with a boolean indicating if a new entry was created (true = created)
    async fn set(&self, id: u32, mut data: Vec<u8>) -> Result<(), DatabaseError> {
        let hash_map_guard = self.data.read().await;

        let rw_lock = match hash_map_guard.get(&id) {
            Some(rw_lock) => rw_lock,
            None => return Err(NotFound(id)),
        };

        let mut guard = match rw_lock.try_write() {
            Ok(guard) => guard,
            Err(_) => return Err(WriteBlocked(id)),
        };

        guard.clear();
        guard.append(&mut data);
        Ok(())
    }

    async fn insert(&self, id: u32, data: Vec<u8>) -> Result<(), DatabaseError> {
        let mut hash_map_guard = self.data.write().await;

        if hash_map_guard.contains_key(&id) {
            return Err(AlreadyExists(id));
        }

        hash_map_guard.insert(id, RwLock::new(data));

        Ok(())
    }

    async fn remove(&self, id: u32) -> Result<(), DatabaseError> {
        let mut hash_map_guard = self.data.write().await;

        match hash_map_guard.remove(&id) {
            Some(_) => Ok(()),
            None => Err(NotFound(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        let db = Repository::new();

        db.data.write().await.insert(1, RwLock::new(b"hello".to_vec()));
        db.data.write().await.insert(2, RwLock::new(b"world".to_vec()));

        assert_eq!(db.get(1).await, Some(b"hello".to_vec()));
        assert_eq!(db.get(2).await, Some(b"world".to_vec()));
        assert_eq!(db.get(3).await, None);
    }

    #[tokio::test]
    async fn test_set() {
        let db = Repository::new();

        db.data.write().await.insert(1, RwLock::new(b"hello".to_vec()));

        db.set(1, b"updated hello".to_vec()).await.unwrap();

        assert_eq!(b"updated hello".to_vec(), db.data.read().await.get(&1).unwrap().read().await.to_vec());
    }

    #[tokio::test]
    async fn test_set_not_found() {
        let db = Repository::new();

        let err = db.set(1, b"updated hello".to_vec()).await.unwrap_err();

        assert_eq!(err, NotFound(1));
    }

    #[tokio::test]
    async fn test_insert() {
        let db = Repository::new();

        db.insert(1, b"hello".to_vec()).await.unwrap();

        assert_eq!(b"hello".to_vec(), db.data.read().await.get(&1).unwrap().read().await.to_vec());
    }

    #[tokio::test]
    async fn test_insert_already_exists() {
        let db = Repository::new();

        db.data.write().await.insert(1, RwLock::new(b"hello".to_vec()));

        let err = db.insert(1, b"new hello".to_vec()).await.unwrap_err();

        assert_eq!(err, AlreadyExists(1));
    }

    #[tokio::test]
    async fn test_remove() {
        let db = Repository::new();

        db.data.write().await.insert(1, RwLock::new(b"hello".to_vec()));

        db.remove(1).await.unwrap();

        assert!(db.data.read().await.get(&1).is_none());
    }

    #[tokio::test]
    async fn test_remove_not_found() {
        let db = Repository::new();

        db.data.write().await.insert(1, RwLock::new(b"hello".to_vec()));

        let err = db.remove(2).await.unwrap_err();

        assert_eq!(err, NotFound(2));
        assert!(db.data.read().await.get(&1).is_some());
    }
}