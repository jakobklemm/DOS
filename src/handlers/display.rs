//! # Metrics

use axum::extract::State;

use crate::states::{system::System, defense::Defense};
use std::sync::{Arc, Mutex};

pub async fn handle_display(State(state): State<Arc<crate::routes::State>>) -> String {
    let players = state.players.lock().unwrap();

    let mut ret = String::new();

    for (k, v) in players.0.iter() {
        let p = v.position().unwrap();
        ret += &(k.to_owned() + ": " + &p.to_string());
    }

    ret
}
