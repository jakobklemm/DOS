//! # Status

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub code: usize,
}

impl Default for Status {
    fn default() -> Self {
        Self { code: 200 }
    }
}
