use std::{
    collections::HashMap,
    fmt::Display,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct AMapMetrics {
    pub data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AMapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl Display for AMapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}

impl AMapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        Self {
            data: Arc::new(
                metric_names
                    .iter()
                    .map(|&key_name| (key_name, AtomicI64::new(0)))
                    .collect(),
            ),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("key {key} not found"))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
