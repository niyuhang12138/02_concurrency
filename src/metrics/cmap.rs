//! metrics data structure
//! 基本功能: inc / dec / snapshot

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt::Display, sync::Arc};

#[derive(Debug)]
pub struct CMapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl Default for CMapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for CMapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl Display for CMapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}

impl CMapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}
