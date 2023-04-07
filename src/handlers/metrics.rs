//! # Metrics

use axum::extract::State;

use crate::states::{system::System, defense::Defense};
use std::sync::{Arc, Mutex};

pub async fn handle_metrics(State(state): State<Arc<crate::routes::State>>) -> String {
    let data = state.system.lock().unwrap();
    // println!("{:?}", data);
    data.parse()
}
