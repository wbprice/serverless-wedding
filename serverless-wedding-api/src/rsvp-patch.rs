extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use std::collections::HashMap;
use serde_json::json;
use log::{info, error};
use uuid::Uuid;

mod rsvp;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {

    let path_parameters = request.path_parameters();
    let payload : HashMap<String, bool> = request.payload()
        .unwrap()
        .unwrap();


    let uuid : Uuid = Uuid::parse_str(
        path_parameters.get("id").unwrap()
    ).unwrap();

    info!("Uuid is: {:?}", uuid);
    info!("Payload is: {:?}", payload);

    match rsvp::RSVP::patch(uuid, payload) {
        Ok(response) => Ok(json!(response)),
        Err(error) => {
            error!("There was a problem! {:?}", error);
            Ok(json!({"message": "There was a problem!"}))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{http, request, Body};
    use serde::de::Deserialize;
    use std::error::Error;

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
