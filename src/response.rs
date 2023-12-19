use crate::model::Menu;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct MenuData {
    pub menu: Menu,
}

#[derive(Serialize, Debug)]
pub struct SingleMenuResponse {
    pub status: String,
    pub data: MenuData,
}

#[derive(Serialize, Debug)]
pub struct MenuListResponse {
    pub status: String,
    pub results: usize,
    pub menus: Vec<Menu>,
}