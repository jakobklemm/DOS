//! # Index

use crate::models::status::Status;
use crate::TABError;
use tabresp::Response;

use axum::extract::State;

use crate::states::system::System;
use std::sync::{Arc, Mutex};

pub async fn handle_index(
    State(state): State<Arc<Mutex<System>>>,
) -> Result<Response<Status>, TABError> {
    let mut data = state.lock().unwrap();
    let status = Status::default();
    Ok(status.into())
}
