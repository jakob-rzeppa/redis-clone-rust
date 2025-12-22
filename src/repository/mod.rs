use std::collections::{HashMap};
use std::sync::{Arc};
use tokio::sync::{RwLock};

pub(crate) type SharedRepository = Arc<Repository>;

pub(crate) struct Repository {
    data: RwLock<HashMap<u32, RwLock<Vec<u8>>>>,
}

impl Repository {
    pub(crate) fn new() -> Self {
        Repository {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub(crate) async fn get(&self, id: u32) -> Option<Vec<u8>> {
        let hash_map_guard = self.data.read().await;
        let rw_lock = hash_map_guard.get(&id)?;
        let data_guard = rw_lock.read().await;
        Some(data_guard.clone())
    }

    /// Returns a Result with a boolean indicating if a new entry was created (true = created)
    pub(crate) async fn set(&self, id: u32, mut data: Vec<u8>) -> Result<(), anyhow::Error> {
        let hash_map_guard = self.data.read().await;

        let rw_lock = match hash_map_guard.get(&id) {
            Some(rw_lock) => rw_lock,
            None => return Err(anyhow::anyhow!("entry with id {} does not exist", id)),
        };

        let mut guard = match rw_lock.try_write() {
            Ok(guard) => guard,
            Err(_) => return Err(anyhow::Error::msg("someone else is currently writing")),
        };

        guard.clear();
        guard.append(&mut data);
        Ok(())
    }

    pub(crate) async fn insert(&self, id: u32, data: Vec<u8>) -> Result<(), anyhow::Error> {
        let mut hash_map_guard = self.data.write().await;

        if hash_map_guard.contains_key(&id) {
            return Err(anyhow::anyhow!("entry with id {} already exists", id));
        }

        hash_map_guard.insert(id, RwLock::new(data));

        Ok(())
    }

    pub(crate) async fn remove(&self, id: u32) -> Result<(), anyhow::Error> {
        let mut hash_map_guard = self.data.write().await;

        match hash_map_guard.remove(&id) {
            Some(_) => Ok(()),
            None => Err(anyhow::Error::msg("not found"))
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

        assert!(err.to_string().contains("entry with id 1 does not exist"));
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

        assert!(err.to_string().contains("entry with id 1 already exists"));
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

        assert!(err.to_string().contains("not found"));
        assert!(db.data.read().await.get(&1).is_some());
    }
}