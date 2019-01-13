extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, Request, Body};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{json};
use std::ops::Deref;
use log::{info, error};

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
    let person: rsvp::Person = serde_json::from_slice(body_slice).unwrap();

    match rsvp::RSVP::create_record(person) {
        Ok(rsvp) => {
            Ok(json!(rsvp))
        },
        Err(_errcar) => {
            Ok(json!({"message": "Error"}))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_handler_handles() {

        let payload = r#"{
            "email_address": "example@email.com",
            "name": "Blaine Price"
        }"#;

        let request = Request::new(Body::from(payload));

        handler(request, Context::default()).expect("expected Ok(_) value");
    }
}