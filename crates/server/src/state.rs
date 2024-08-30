use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::extract::{ws, FromRef};
use tokio::sync::mpsc;

pub type Users = HashMap<String, mpsc::UnboundedSender<Result<Vec<u8>, (ws::CloseCode, String)>>>;

#[derive(Debug)]
pub struct StateValue {
    pub users: Mutex<Users>,
}

#[derive(Debug, Clone, FromRef)]
pub struct AppState(Arc<StateValue>);

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(&self) -> &StateValue {
        &self.0
    }
}

impl Default for AppState {
    fn default() -> Self {
        let users = Mutex::new(HashMap::new());
        Self(Arc::new(StateValue { users }))
    }
}
