//! Routes

use axum::{
    routing::{get, post, IntoMakeService},
    Router, Extension,
};

use std::sync::{Arc, Mutex};

use crate::handlers::{index::handle_index, metrics::handle_metrics, submit::{handle_me_submission, handle_pos_submission}, display::handle_display};

use crate::states::{defense::Defense, system::System, players::Players};

pub struct Application {
    router: Router,
}

pub struct State {
    pub system: Arc<Mutex<System>>,
    pub defense: Arc<Mutex<Defense>>, 
    pub players: Arc<Mutex<Players>>
}

impl State {
    fn new() -> Self {
        Self {
            system: Arc::new(Mutex::new(System(Vec::new()))),
            defense: Arc::new(Mutex::new(Defense)),
            players: Arc::new(Mutex::new(Players::new()))
        }
    }
}

impl Application {
    pub fn into_make_service(self) -> IntoMakeService<Router> {
        self.router.into_make_service()
    }

    pub fn new() -> Self {
        let state = State::new();

        let router = Router::new()
            .route("/", get(handle_index))
            .route("/submit/me", post(handle_me_submission))
            .route("/submit/pos", post(handle_pos_submission))
            .route("/metrics", get(handle_metrics))
            .route("/display", get(handle_display))
            .with_state(Arc::new(state));

        Self { router }
    }
}
