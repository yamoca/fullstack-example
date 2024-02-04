//! Run with
//!
//! ```not_rust
//! cargo run -p example-readme
//! 
//! curl -X POST -H "Content-Type: application/json" -d '{"username": "leo"}' http://127.0.0.1:3000/users
//! ```

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(root))                      // `GET /` goes to `root`
        .route("/users", post(create_user));        // `POST /users` goes to `create_user`

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// this argument tells axum to parse the request body
// as JSON into a `CreateUser` type
async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };
    println!("created user {:#?}", user);

    // this will be converted into a JSON response with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
#[derive(Debug)]
struct User {
    id: u64,
    username: String,
}
