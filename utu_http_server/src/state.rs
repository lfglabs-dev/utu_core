use std::sync::Arc;

use crate::logger::Logger;

pub struct AppState {
    pub logger: Logger,
}

impl AppState {
    pub async fn load() -> Arc<Self> {
        Arc::new(AppState {
            logger: Logger::new(),
        })
    }
}
