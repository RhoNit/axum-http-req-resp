mod routes_handler;
mod route_handler_text;
mod route_handler_json;

use axum::{Router, routing::{get, post}};

use self::{
    routes_handler::get_passwd, 
    route_handler_text::get_request_text, 
    route_handler_json::get_request_json
};


pub fn create_router() -> Router {
    Router::new()
        .route("/secret", get(get_passwd))
        .route("/get_text", post(get_request_text))
        .route("/get_json", post(get_request_json))
}