//! Routes

use axum::{
    routing::{get, post, IntoMakeService},
    Router,
};
use tracing::info;

use std::sync::{Arc, Mutex};

use crate::handlers::{index::handle_index, metrics::handle_metrics, submit::handle_me_submission};

use crate::states::{defense::Defense, system::System};

pub struct Application {
    router: Router,
}

impl Application {
    pub fn into_make_service(self) -> IntoMakeService<Router> {
        self.router.into_make_service()
    }

    pub fn new() -> Self {
        let system = System(Vec::new());
        let system_state = Arc::new(Mutex::new(system));

        let defense = Defense;
        let defense_state = Arc::new(Mutex::new(defense));

        let router = Router::new()
            .route("/", get(handle_index))
            .route("/submit/me", post(handle_me_submission))
            .route("/metrics", get(handle_metrics))
            .with_state(system_state)
            .with_state(defense_state);

        Self { router }
    }
}
