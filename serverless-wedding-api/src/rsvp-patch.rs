extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, Request, Body};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{json, Value};
use url::{Url, ParseError};
use std::ops::Deref;
use log::{info, error};
use uuid::Uuid;
use std::collections::HashMap;
mod rsvp;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {

    let body = request.body().deref();
    let payload: HashMap<String, bool> = serde_json::from_slice(body).unwrap_or_else(|e| {
        panic!("Do better! {:?}", e);
    });

    match Url::parse(&request.uri().to_string()) {
        Ok(uri) => {
            match uri.path_segments().map(|c| c.collect::<Vec<_>>()) {
                Some(path_segments) => {
                    match Uuid::parse_str(&path_segments[1].to_string()) {
                        Ok(uuid) => {
                            match rsvp::RSVP::patch(uuid, payload) {
                                Ok(rsvps) => Ok(json!(rsvps)),
                                Err(_) => Ok(json!({"message": "Failed to retrieve RSVPs"}))
                            }
                        },
                        Err(_) => Ok(json!({"message": "UUID not provided"}))
                    }
                },
                None => Ok(json!({"message": "No URI segments found."}))
            }
        },
        Err(_) => Ok(json!({"message": "Couldn't parse the URI"}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patch_handler_handles() {

        let payload = r#"[
            "attending": true,
            "invitation_submitted": true,
            "reminder_submitted": true
        ]"#;

        let mut request = Request::new(Body::from(payload));
        *request.uri_mut() = "https://api.slswedding.com/household/3eb28445-7698-4a00-b071-49da8eaac944".parse().unwrap();

        handler(request, Context::default()).expect("Expected an OK response");
    }
}