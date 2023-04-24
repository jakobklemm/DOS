//! # Submit

use crate::TABError;
use crate::states::defense::Defense;
use crate::states::players::{Source, Player, Players};
use tabresp::Response;

use axum::extract::State;

use crate::states::system::Element;

use std::fs;
use std::sync::{Arc, Mutex};

use crate::models::status::Status;

pub async fn handle_me_submission(
    State(state): State<Arc<crate::routes::State>>,
    body: String,
) -> Result<Response<Status>, TABError> {
    println!("ME Submission");
    let data = &mut *state.system.lock().unwrap();

    let lines: Vec<&str> = body.split("\n").collect();
    for l in lines {
        if let Ok(e) = Element::from(l.to_string()) {
            data.push(e);
        }
    }

    // println!("{:?}", data.0.len());
    data.0.sort();
    data.0.reverse();
    // let p = serde_json::to_string_pretty(&data.0)?;
    // let _ = fs::write("me.json", &p);

    let status = Status::default();
    Ok(status.into())
}

pub async fn handle_pos_submission(
    State(state): State<Arc<crate::routes::State>>,
    body: String,
) -> Result<Response<Status>, TABError> {
    println!("Position Submission");
    let data = &mut *state.players.lock().unwrap();
    let data = &mut data.0;

    let lines: Vec<&str> = body.split("\n").collect();

    let mut source = Source::new();

    source.parse(*lines.get(0).ok_or(TABError::default())?);

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let (name, distance) = Player::parse(*line)?;
        if let Some(p) = data.get_mut(&name) {
            if let Some(d) = p.distances.get_mut(&source) {
                *d = distance;
            } else {
                // player exists but not source
                p.distances.insert(source.clone(), distance);
            }
        } else {
            // Player doesn't exist
            let player = Player::new((&name).clone().to_string(), source.clone(), distance);
            data.insert((&name).clone().to_string(), player);
        }
    }

    // println!("{:?}", data);

    let status = Status::default();
    Ok(status.into())
}