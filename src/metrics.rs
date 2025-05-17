// metrics data structure
// 基本功能：inc/dec/snapshot
use anyhow::Result;

use dashmap::DashMap;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&mut self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;

        Ok(())
    }

    // pub fn dec(&mut self, key: &str) {
    //     let mut data = self.data.lock().unwrap();
    //     let counter = data.entry(key.into()).or_insert(0);
    //     *counter -= 1;
    // }

    // pub fn snapshot(&self) -> Result<DashMap<String, i64>> {
    //     Ok(self
    //         .data
    //         .read()
    //         .map_err(|e| anyhow!(e.to_string()))?
    //         .clone())
    // }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
