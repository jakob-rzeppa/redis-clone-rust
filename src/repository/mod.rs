use std::collections::{HashMap};
use std::sync::{Arc};
use tokio::sync::{RwLock};

pub(crate) type SharedRepository = Arc<Repository>;

pub(crate) struct Repository {
    data: HashMap<u32, RwLock<Vec<u8>>>,
}

impl Repository {
    pub(crate) async fn get(&self, id: u32) -> Option<Vec<u8>> {
        let rw_lock = self.data.get(&id)?;
        let guard = rw_lock.read().await;
        Some(guard.clone())
    }

    /// Returns a Result with a boolean indicating if a new entry was created (true = created)
    pub(crate) async fn set(&mut self, id: u32, mut data: Vec<u8>) -> Result</* was new entry created? */ bool, anyhow::Error> {
        let rw_lock = match self.data.get(&id) {
            Some(rw_lock) => rw_lock,
            None => {
                self.data.insert(id, RwLock::new(data));
                return Ok(true);
            }
        };

        let mut guard = match rw_lock.try_write() {
            Ok(guard) => guard,
            Err(_) => return Err(anyhow::Error::msg("someone else is currently writing")),
        };

        guard.clear();
        guard.append(&mut data);
        Ok(false)
    }

    pub(crate) async fn remove(&mut self, id: u32) -> Result<(), anyhow::Error> {
        match self.data.remove(&id) {
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
        let mut db = Repository {
            data: HashMap::new(),
        };

        db.data.insert(1, RwLock::new(b"hello".to_vec()));
        db.data.insert(2, RwLock::new(b"world".to_vec()));

        assert_eq!(db.get(1).await, Some(b"hello".to_vec()));
        assert_eq!(db.get(2).await, Some(b"world".to_vec()));
        assert_eq!(db.get(3).await, None);
    }

    #[tokio::test]
    async fn test_set() {
        let mut db = Repository {
            data: HashMap::new(),
        };

        db.data.insert(1, RwLock::new(b"hello".to_vec()));

        let was_created = db.set(1, b"updated hello".to_vec()).await.unwrap();

        assert_eq!(b"updated hello".to_vec(), db.data.get(&1).unwrap().read().await.to_vec());
        assert_eq!(was_created, false);
    }

    #[tokio::test]
    async fn test_set_previously_empty() {
        let mut db = Repository {
            data: HashMap::new(),
        };

        let was_created = db.set(1, b"updated hello".to_vec()).await.unwrap();

        assert_eq!(b"updated hello".to_vec(), db.data.get(&1).unwrap().read().await.to_vec());
        assert_eq!(was_created, true);
    }

    #[tokio::test]
    async fn test_remove() {
        let mut db = Repository {
            data: HashMap::new(),
        };

        db.data.insert(1, RwLock::new(b"hello".to_vec()));

        db.remove(1).await.unwrap();

        assert!(db.data.get(&1).is_none());
    }

    #[tokio::test]
    async fn test_remove_not_found() {
        let mut db = Repository {
            data: HashMap::new(),
        };

        db.data.insert(1, RwLock::new(b"hello".to_vec()));

        let err = db.remove(2).await.unwrap_err();

        assert!(err.to_string().contains("not found"));
        assert!(db.data.get(&1).is_some());
    }
}