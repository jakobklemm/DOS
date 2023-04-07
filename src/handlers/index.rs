//! # Index

use crate::models::status::Status;
use crate::TABError;
use tabresp::Response;

use axum::extract::State;

use crate::states::system::System;
use std::sync::{Arc, Mutex};

pub async fn handle_index() -> Result<Response<Status>, TABError> {
    let status = Status::default();
    Ok(status.into())
}
