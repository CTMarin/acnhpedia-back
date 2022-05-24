mod libacnh;

use std::{
    net::SocketAddr, 
};
use axum::{routing::{post}, Router, http::StatusCode, Json};
use serde::Deserialize;
use http::{Method};
use tower_http::cors::{Any, CorsLayer};
use libacnh::{logging, users};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(Any)
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    logging::info(format!("Server listening on {}", addr).as_str());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn register(Json(payload): Json<RegisterPost>) -> Result<StatusCode, StatusCode> {
    logging::info("Register request recieved!");

    match users::register_user(&payload.email, &payload.username, &payload.password, &payload.rep_password) {
        Ok(200) => {
            logging::info("Register successful");
            Ok(StatusCode::OK)
        },
        Err(400) => {
            logging::error("Passwords do not match");
            Err(StatusCode::BAD_REQUEST)
        },
        Err(409) => {
            logging::error("User already exists");
            Err(StatusCode::CONFLICT)
        },
        _ => {
            logging::error("Error writing on DB");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn login(Json(payload): Json<LoginPost>) -> Result<StatusCode, StatusCode> {
    logging::info(format!("Login request recieved!").as_str());

    match users::check_login(&payload.email, &payload.password) {
        Ok(200) => {
            logging::info("Login successful");
            Ok(StatusCode::OK)
        },
        Err(401) => {
            logging::error("Wrong Password");
            Err(StatusCode::UNAUTHORIZED)
        },
        Err(404) => {
            logging::error("User Not Found");
            Err(StatusCode::NOT_FOUND)
        }
        _ => Err(StatusCode::BAD_REQUEST)
    }
}


#[derive(Deserialize)]
struct RegisterPost {
    email: String,
    username: String,
    password: String,
    rep_password: String
}

#[derive(Deserialize)]
struct LoginPost {
    email: String,
    password: String,
}


