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
    println!("Handle Display");

    let mut ret = String::from("var UnminedPlayers = [\n");

    let l = players.0.len();

    for (i, (k, v)) in players.0.iter().enumerate() {
        println!("{:?}", v);
        let pl = JSONPosition::from_player(v).unwrap();
        ret += &(pl);
        if i == l - 1 {
            ret += "\n";
        } else {
            ret += ", \n";
        }
    }

    ret += "]\n";

    ret
}
