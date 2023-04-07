//! # Metrics

use axum::extract::State;

use crate::states::{system::System, defense::Defense};
use std::sync::{Arc, Mutex};

pub async fn handle_metrics(State(state): State<Arc<crate::routes::State>>) -> String {
    let data = state.system.lock().unwrap();
    // println!("{:?}", data);
    let first = data.parse();

    let players = state.players.lock().unwrap();

    for (k, v) in players.0.iter() {
        println!("Name: {}", k);
        let _ = v.position();
    }

    first
}
