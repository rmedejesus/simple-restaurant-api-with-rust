# Simple Restaurant API with RUST

**Introduction**

This is a backend API for a restaurant menu application developed with RUST.\
The programming language and tools used for developing this application are:\
- RUST with Rocket Framework
- Microsoft Visual Studio Code
- Postman (for simultaneous testing of API endpoints)

The console application consists of the following modules:
- `main.rs` module which serves as the entry point for the route paths of the several API endpoints
- `model.rs` module which serves as the structure for the restaurant menu and also setting up the in-memory database to be used for storing the objects
- `response.rs` module which serves as the model for the response object to be returned from the request
- `handler.rs` module which serves as the service model for the API endpoints, business logic and data manipulations.

**Building and Running the Application**

- unpack the zip file
- open the root folder in a code editor or IDE (in my case, I used Visual Studio Code as my main development tool)
- make sure to have RUST installed in your machine
- build the application by running the command `cargo build`, it will create the target folder and build the dependencies used by the application
- run the application by typing the command `cargo run`, it will be served in the default ip and port `http://127.0.0.1:8000`
- the endpoint for the create `/post` is `http://127.0.0.1:8000/menus`, if the client wants to set a limit to how many specific table numbers are only allowed, a request parameter may be attached (e.g. `http://127.0.0.1:8000/menus?limit=1`)
- the endpoint for the list `/get` is `http://127.0.0.1:8000/menus?tableNumber={n}`, where `{n}` is the request parameter which refers to the specific table number we want to get the menu items from
- the endpoint for the single `/get` is `http://127.0.0.1:8000/menus/{name}?tableNumber={n}`, where `{name}` is the specific menu item name we want to get and `{n}` is the request parameter which refers to the specific table number we want to get the menu item from
- the endpoint for the delete `/delete` is `http://127.0.0.1:8000/menus/{name}?tableNumber={n}`, where `{name}` is the specific menu item name we want to delete and `{n}` is the request parameter which refers to the specific table number the item will be deleted from

**Testing the Application**

I have attached a `Simple Restaurant API Tests.postman_collection`, which can be run on the Postman application to test the endpoint results. The collection has 10 sample requests in it and I have done several tests with this collection by running them simultaneously to test how the application responds for simultaneous requests. For the last request example, I made this as a test when the user inputs a limit to the specified table numbers and should return a conflict if the request has overlapped the limit and with this, all tests had a success response of 200 (Ok) except for that 409 conflict to test the limit parameter.

**Other Notes**

- `/post` requests made by the client are of Json structure body types and passed in to the endpoint as a Rust object struct
- I used the `serde` libraries for serializing and deserializing objects to and from json
- I also applied some error handlers and return the specific error message for the constraints
- For the `prepareTime` field, I assumed that if the optional field has a value from the client request, then it will just created a random integer starting from 5 to 15 and then save it statically in the database.
- I have added the `tableNumber` as request parameter for all endpoints except the `/post` because I am assuming that every item should be identified first by it's corresponding table number before its name.
- When calling the `/post` endpoint, users can add at least 1 or more menu items.
- I applied the `#[allow(non_snake_case)]` attribute to almost all of my functions and structs because I was always used to `using camelCase` instead of the `snake_case`
- I applied asynchronous programming for concurrency and in my research, this uses the `Future` trait rather than blocking the current thread, as to what synchronous programming does and makes runtime performance slower.
- Before I uploaded and compressed the project, I did a `cargo clean` to remove unused dependencies during deployment since once we run `cargo build`, it will compile the dependencies again, making the project folder have a big size.