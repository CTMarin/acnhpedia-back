mod libacnh;

use std::{
    net::SocketAddr, 
};
use axum::{routing::{get, post}, Router, http::StatusCode, Json};
use axum_macros::debug_handler;
use serde::Deserialize;
use http::{Method};
use tower_http::cors::{Any, CorsLayer};
use libacnh::{logging, users, obtained};
use axum::extract::{Query};


#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/obtain", post(obtain))
        .route("/forfait", post(forfait))
        .route("/obtained", get(obtained))
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

async fn obtain(Json(payload): Json<ObtainPost>) -> Result<StatusCode, StatusCode> {
    logging::info(format!("Obtain card request!").as_str());

    match obtained::add_obtained_card(&payload.user, &payload.card_type, &payload.id) {
        Ok(200) => {
            logging::info(format!("Card {}:{} obtained", &payload.card_type, payload.id).as_str());
            Ok(StatusCode::OK)
        },
        Err(409) => {
            logging::error(format!("Card {}:{} has been already obtained", &payload.card_type, payload.id).as_str());
            Err(StatusCode::CONFLICT)
        },
        Err(500) => {
            logging::error("Internal server error");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        _ => Err(StatusCode::BAD_REQUEST)
    }
}

async fn forfait(Json(payload): Json<ForfaitPost>) -> Result<StatusCode, StatusCode> {
    logging::info(format!("Forfait card request!").as_str());

    match obtained::remove_obtained_card(&payload.user, &payload.card_type, &payload.id) {
        Ok(200) => {
            logging::info(format!("Card {}:{} forfait", &payload.card_type, payload.id).as_str());
            Ok(StatusCode::OK)
        },
        Err(404) => {
            logging::error(format!("Card {}:{} is not obtained", &payload.card_type, payload.id).as_str());
            Err(StatusCode::NOT_FOUND)
        },
        Err(500) => {
            logging::error("Internal server error");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        _ => Err(StatusCode::BAD_REQUEST)
    }
}

#[debug_handler]
async fn obtained(params: Query<ObtainedGet>) -> Json<Vec<String>> {
    logging::info(format!("Request for obtained cards!").as_str());

    if users::check_user(&params.0.user) {
        return Json(obtained::get_obtained_cards_by_type(&params.0.user, &params.0.card_type))
    } else {
        return Json(vec!());
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

#[derive(Deserialize)]
struct ObtainPost {
    user: String,
    card_type: String,
    id: String
}

#[derive(Deserialize)]
struct ForfaitPost {
    user: String,
    card_type: String,
    id: String
}

#[derive(Deserialize)]
struct ObtainedGet {
    user: String,
    card_type: String
}

