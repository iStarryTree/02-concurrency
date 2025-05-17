use anyhow::{Result, anyhow};
use std::{
    collections::HashMap,
    fmt,
    sync::{
        Arc,
        atomic::{AtomicI64, Ordering},
    },
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        Self {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self.data.get(key).ok_or(anyhow!("key {} not found", key))?;
        counter.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
