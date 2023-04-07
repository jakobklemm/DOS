//! # Submit

use crate::TABError;
use tabresp::Response;

use axum::extract::State;

use crate::states::system::Element;

use std::fs;
use std::sync::{Arc, Mutex};

use crate::models::status::Status;
use crate::states::system::System;

pub async fn handle_me_submission(
    State(state): State<Arc<Mutex<System>>>,
    body: String,
) -> Result<Response<Status>, TABError> {
    let mut data = state.lock().unwrap();

    let lines: Vec<&str> = body.split("\n").collect();
    for l in lines {
        if let Ok(e) = Element::from(l.to_string()) {
            data.push(e);
        }
    }

    println!("{:?}", data.0.len());
    data.0.sort();
    data.0.reverse();
    let p = serde_json::to_string_pretty(&data.0)?;
    let _ = fs::write("me.json", &p);

    let status = Status::default();
    Ok(status.into())
}
