extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, http, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;
use uuid::Uuid;

mod models;
use crate::models::RSVP;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {
    let path_parameters = request.path_parameters();
    let uuid : Uuid = Uuid::parse_str(
        path_parameters.get("id").unwrap()
    ).unwrap();

    Ok(match RSVP::get(uuid) {
        Ok(response) => {
            http::Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .status(200)
                .body(json!(response).to_string())
                .unwrap()
        },
        Err(_err) => {
            http::Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .status(500)
                .body(json!({"message": "Something went wrong!"}).to_string())
                .unwrap()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{http, Body};

    #[test]
    #[ignore]
    fn patch_handler_handles() {

        let payload = r#"{
            "attending": true,
            "invitation_submitted": true,
            "reminder_submitted": true
        }"#;

        let request = http::Request::builder()
             .uri("https://api.com/")
             .method("PUT")
             .header("Content-Type", "application/json")
             .body(Body::from(payload.clone()))
             .expect("failed to build request");

        handler(request, Context::default()).expect("Expected an OK response");
    }
}
