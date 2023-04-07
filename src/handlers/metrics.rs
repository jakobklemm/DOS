//! # Metrics

use axum::extract::State;

use crate::states::system::System;
use std::sync::{Arc, Mutex};

pub async fn handle_metrics(State(state): State<Arc<Mutex<System>>>) -> String {
    let data = state.lock().unwrap();
    // println!("{:?}", data);
    data.parse()
}
