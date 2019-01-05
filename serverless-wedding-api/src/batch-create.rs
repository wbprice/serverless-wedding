use lambda_http::{lambda, IntoResponse, Request, Body};
use lambda_runtime::{error::HandlerError, Context};
use rusoto_core::{{Region}};
use serde_json::{{json}};
use std::ops::Deref;
use std::env;

use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, PutItemError};
use serde_dynamodb;

mod rsvp;

fn main() {
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context
) -> Result<impl IntoResponse, HandlerError> {

    let body_slice = request.body().deref();
    let people : rsvp::People = serde_json::from_slice(body_slice).unwrap_or_else(|err| {
        panic!(err);
    });

    Ok(json!({"message": "OK!"}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_handles() {

        let payload = r#"
        {
            "people": [
                {
                    "email_address": "example@email.com",
                    "name": "Blaine Price"
                },
                {
                    "email_address": "example@gmail.com",
                    "name": "Cynthia Young"
                }
            ]
        }
        "#;

        let request = Request::new(Body::from(payload));

        let response = handler(request, Context::default())
            .expect("expected Ok(_) value");
    }
}