extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, http, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{json};
use std::ops::Deref;
use log::{error};

mod models;
use crate::models::{Household, Person};

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context
) -> Result<impl IntoResponse, HandlerError> {
    let body = request.body().deref();
    let people : Vec<Person> = serde_json::from_slice(body).unwrap();

    Ok(match Household::create(people) {
        Ok(response) => {
            http::Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .status(200)
                .body(json!(response).to_string())
                .unwrap()
        },
        Err(error) => {
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
    use lambda_http::{Body};

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
