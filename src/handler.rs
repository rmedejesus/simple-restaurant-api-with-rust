use crate::{
    model::{AppState, Menu},
    response::{StatusResponse, SingleMenuResponse, MenuData, MenuListResponse},
};
use rocket::{
    delete, get, http::Status, post, response::status::Custom, serde::json::Json, State,
};
use rand::Rng;
use itertools::Itertools;

#[allow(non_snake_case)]
#[post("/menus?<limit>", data = "<body>")]
pub async fn create_menu_item_handler(
    limit: Option<usize>,
    body: Json<Vec<Menu>>,
    data: &State<AppState>,
) -> Result<Json<MenuListResponse>, Custom<Json<StatusResponse>>> {
    let mut vec = data.simple_restaurant_menu_db.lock().unwrap();
    let mut menuVector: Vec<Menu> = Vec::new();

    let menusRequest = body.to_owned();

    let uniqueTableNumbers = menusRequest.0.iter().unique_by(|p| &p.tableNumber).collect::<Vec<_>>();
    let maxLimit = limit.unwrap_or(100);

    if uniqueTableNumbers.len() > maxLimit {
        let error_response = StatusResponse {
            status: "fail".to_string(),
            message: format!("There is a limit of only {} specific table numbers", maxLimit),
        };
        return Err(Custom(Status::Conflict, Json(error_response)));
    }

    for mut menuRequest in menusRequest.0 {
        for menu in vec.iter() {
            if menu.name == menuRequest.name && menu.tableNumber == menuRequest.tableNumber {
                let error_response = StatusResponse {
                    status: "fail".to_string(),
                    message: format!("Menu with name '{}' already exists on table number {}", menu.name, menu.tableNumber),
                };
                return Err(Custom(Status::Conflict, Json(error_response)));
            }
        }

        if menuRequest.prepareTime.is_some() {
            let mut rng = rand::thread_rng();
            let randomPreparationTime: usize = rng.gen_range(5..15);
            menuRequest.prepareTime = Some(randomPreparationTime);
        }

        vec.push(menuRequest.clone());
        menuVector.push(menuRequest.clone());
    }

    let json_response = MenuListResponse {
        status: "success".to_string(),
        results: menuVector.len(),
        menus: menuVector,
    };
    Ok(Json(json_response))
}

#[allow(non_snake_case)]
#[get("/menus?<tableNumber>")]
pub async fn get_menus_handler(
    tableNumber: usize,
    data: &State<AppState>,
) -> Result<Json<MenuListResponse>, Custom<Json<StatusResponse>>> {
    let vec = data.simple_restaurant_menu_db.lock().unwrap();

    let menus: Vec<Menu> = vec.clone().into_iter().filter(|x| x.tableNumber == tableNumber).collect();

    if menus.len() == 0 {
        let error_response = StatusResponse {
            status: "fail".to_string(),
            message: format!("No menu items found on table number {}", tableNumber),
        };
        return Err(Custom(Status::NotFound, Json(error_response)))
    }

    let json_response = MenuListResponse {
        status: "success".to_string(),
        results: menus.len(),
        menus,
    };
    Ok(Json(json_response))
}

#[allow(non_snake_case)]
#[get("/menus/<name>?<tableNumber>")]
pub async fn get_menu_handler(
    name: &str,
    tableNumber: usize,
    data: &State<AppState>,
) -> Result<Json<SingleMenuResponse>, Custom<Json<StatusResponse>>> {
    let vec = data.simple_restaurant_menu_db.lock().unwrap();

    for menu in vec.iter() {
        if menu.name == name && menu.tableNumber == tableNumber {
            let json_response = SingleMenuResponse {
                status: "success".to_string(),
                data: MenuData { menu: menu.clone() },
            };

            return Ok(Json(json_response));
        }
    }

    let error_response = StatusResponse {
        status: "fail".to_string(),
        message: format!("Menu item with name '{}' not found on table number {}", name, tableNumber),
    };
    Err(Custom(Status::NotFound, Json(error_response)))
}

#[allow(non_snake_case)]
#[delete("/menus/<name>?<tableNumber>")]
pub async fn delete_menu_item_handler(
    name: &str,
    tableNumber: usize,
    data: &State<AppState>,
) -> Result<Json<StatusResponse>, Custom<Json<StatusResponse>>> {
    let mut vec = data.simple_restaurant_menu_db.lock().unwrap();

    for menu in vec.iter_mut() {
        if menu.name == name && menu.tableNumber == tableNumber {
            vec.retain(|menu| menu.name != name);

            let success_response = StatusResponse {
                status: "success".to_string(),
                message: format!("Menu item with name '{}' has been removed on table number {}", name, tableNumber),
            };

            return Ok(Json(success_response));
        }
    }

    let error_response = StatusResponse {
        status: "fail".to_string(),
        message: format!("Menu item with name '{}' not found on table number {}", name, tableNumber),
    };
    Err(Custom(Status::NotFound, Json(error_response)))
}