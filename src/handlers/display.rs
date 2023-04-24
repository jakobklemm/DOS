//! # Metrics

use axum::extract::State;
use serde::Serialize;

use crate::{
    error::TABError,
    states::{defense::Defense, players::Player, system::System},
};
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
struct JSONPosition {
    name: String,
    x: f64,
    z: f64,
}

impl JSONPosition {
    fn from_player(player: &Player) -> Result<String, TABError> {
        let p = player.position()?;
        let jp = Self {
            name: player.name.clone(),
            x: p.x as f64,
            z: p.z as f64,
        };

        let s = serde_json::to_string_pretty(&jp)?;
        Ok(s)
    }
}

pub async fn handle_display(State(state): State<Arc<crate::routes::State>>) -> String {
    let players = state.players.lock().unwrap();

    let mut ret = String::from("var UnminedPlayers = [\n");

    for (k, v) in players.0.iter() {
        if v.position().unwrap().x == 0 && v.position().unwrap().z == 0 {
            continue;
        }
        let pl = JSONPosition::from_player(v).unwrap();
        ret += &(pl);
        ret += "\n";
    }

    ret += "]\n";

    ret
}
