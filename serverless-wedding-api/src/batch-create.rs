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
    _: Context
) -> Result<impl IntoResponse, HandlerError> {
    let body = request.body().deref();
    let people : Vec<rsvp::Person> = serde_json::from_slice(body).unwrap();

    match rsvp::RSVP::batch_create_records(people) {
        Ok(response) => {
            Ok(json!(response))
        },
        Err(error) => {
            error!("{}", error);
            Ok(json!({"message": "error creating records"}))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_create_handler_handles() {

        let payload = r#"[
            {
                "email_address": "example@email.com",
                "name": "Blaine Price"
            },
            {
                "email_address": "example@gmail.com",
                "name": "Cynthia Young"
            }
        ]"#;

        let request = Request::new(Body::from(payload));

        handler(request, Context::default()).expect("expected Ok(_) value");
    }
}