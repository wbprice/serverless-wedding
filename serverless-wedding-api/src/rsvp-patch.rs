extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, Request, RequestExt, Body, http};
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

    let path_parameters = request.path_parameters();
    
    match request.payload() {
        Ok(payload) => {
            let result : HashMap<String, bool> = payload.unwrap();
            dbg!(result);
        },
        Err(err) => {
            dbg!(err);
        }
    }

    dbg!(path_parameters);

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
            .uri("https://serverless-wedding-api.com/ac242e6f-269c-498b-aa5c-4b0535bd9366")
            .method("PUT")
            .header("Content-Type", "application/json")
            .body(Body::from(payload.clone()))
            .expect("failed to build request");

        handler(request, Context::default()).expect("Expected an OK response");
    }
}