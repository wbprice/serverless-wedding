extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, Request, Body};
use lambda_runtime::{error::HandlerError, Context};
use rusoto_core::{{Region}};
use serde_json::{{json}};
use std::ops::Deref;
use std::env;
use log::{{info, error}};

use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, PutItemError};
use serde_dynamodb;

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
    info!("the body {:?}", body);

    let people : Vec<rsvp::Person> = serde_json::from_slice(body).unwrap();
    info!("the people {:?}", people);

    match rsvp::Household::create_records(people) {
        Ok(response) => {
            info!("the response {:?}", response);
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
    fn handler_handles() {

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

        let response = handler(request, Context::default())
            .expect("expected Ok(_) value");
    }
}