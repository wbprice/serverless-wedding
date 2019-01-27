extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, Request, RequestExt, Body, http, PathParameters, StrMap};
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
    context: Context,
) -> Result<impl IntoResponse, HandlerError> {

    let path_parameters = request.path_parameters();
    let payload : HashMap<String, bool> = request.payload()
        .unwrap()
        .unwrap();

    info!("{:?}", path_parameters);
    info!("{:?}", request);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
            .extension(PathParameters(StrMap {
                "id": "1234"
            }))
            .body(Body::from(payload.clone()))
            .expect("failed to build request");

        handler(request, Context::default()).expect("Expected an OK response");
    }
}