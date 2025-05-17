// metrics data structure
// 基本功能：inc/dec/snapshot
use anyhow::Result;
use anyhow::anyhow;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&mut self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;

        Ok(())
    }

    // pub fn dec(&mut self, key: &str) {
    //     let mut data = self.data.lock().unwrap();
    //     let counter = data.entry(key.into()).or_insert(0);
    //     *counter -= 1;
    // }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}
