use handler::{
    create_menu_item_handler,
    delete_menu_item_handler,
    get_menu_handler,
    get_menus_handler,
};

#[macro_use] extern crate rocket;

mod handler;
mod model;
mod response;

#[launch]
fn rocket() -> _ {
    let app_data = model::AppState::init();
    rocket::build().manage(app_data).mount(
        "/",
        routes![
            create_menu_item_handler,
            delete_menu_item_handler,
            get_menu_handler,
            get_menus_handler
        ],
    )
}
