use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Menu {
    pub id: Option<String>,
    pub name: String,
    pub tableNumber: usize,
    pub cookingTime: String,
    pub prepareTime: Option<usize>
}

pub struct AppState {
    pub simple_restaurant_menu_db: Arc<Mutex<Vec<Menu>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            simple_restaurant_menu_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}