use std::path::Path;

use rocksdb::{Options, DB};

use crate::models::board::Board;
use crate::models::list::TrelloList;

pub struct Cache {
    db: DB,
}

impl Cache {
    pub fn open(path: &Path) -> Result<Self, rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Cache { db })
    }

    fn board_key(board_id: &str) -> String {
        format!("cache/board/{}/", board_id)
    }

    fn list_key(list_id: &str) -> String {
        format!("cache/list/{}/", list_id)
    }

    pub fn get_board(&self, board_id: &str) -> Option<Board> {
        let key = Self::board_key(board_id);
        self.db
            .get(key.as_bytes())
            .ok()
            .flatten()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
    }

    pub fn put_board(&self, board_id: &str, board: &Board) -> Result<(), String> {
        let key = Self::board_key(board_id);
        let value = serde_json::to_vec(board).map_err(|e| e.to_string())?;
        self.db.put(key.as_bytes(), value).map_err(|e| e.to_string())
    }

    pub fn get_list(&self, list_id: &str) -> Option<TrelloList> {
        let key = Self::list_key(list_id);
        self.db
            .get(key.as_bytes())
            .ok()
            .flatten()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
    }

    pub fn put_list(&self, list_id: &str, list: &TrelloList) -> Result<(), String> {
        let key = Self::list_key(list_id);
        let value = serde_json::to_vec(list).map_err(|e| e.to_string())?;
        self.db.put(key.as_bytes(), value).map_err(|e| e.to_string())
    }

    pub fn clear(&self) -> Result<(), String> {
        let keys: Vec<Vec<u8>> = self
            .db
            .iterator(rocksdb::IteratorMode::Start)
            .filter_map(|r| r.ok().map(|(key, _)| key.to_vec()))
            .collect();
        for key in &keys {
            self.db.delete(key).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

pub fn default_cache_path() -> String {
    let cache_dir = dirs::cache_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    cache_dir
        .join("trello-rs")
        .to_string_lossy()
        .to_string()
}
