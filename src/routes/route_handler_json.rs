use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct JsonRequest {
    message: String,
}


#[derive(Deserialize, Serialize)]
pub struct JsonResponse {
    message: String,
    message_from_server: String,
}

pub async fn get_request_json(Json(body): Json<JsonRequest>) -> Json<JsonResponse> {
    Json(JsonResponse { 
        message: body.message, 
        message_from_server: "Response has been served".to_owned() 
    })
}