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
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {

    let body_slice = request.body().deref();
    let person: rsvp::Person = serde_json::from_slice(body_slice).unwrap_or_else(|err| {
        println!("{:?}", err);
        panic!("Couldn't work with that");
    });

    match create_rsvp_record(person) {
        Ok(rsvp) => {
            Ok(json!(rsvp))
        },
        Err(_errcar) => {
            Ok(json!({"message": "Error"}))
        }
    }
}

pub fn create_rsvp_record(person: rsvp::Person) -> Result<rsvp::RSVP, PutItemError>{
    let rsvp = rsvp::RSVP::new(person);
    let client = DynamoDbClient::new(Region::UsEast1);

    let input = PutItemInput {
        item: serde_dynamodb::to_hashmap(&rsvp).unwrap(),
        table_name: env::var("RSVP_TABLE_NAME").unwrap(),
        ..PutItemInput::default()
    };
    
    match client.put_item(input).sync() {
        Ok(_) => {
            return Ok(rsvp);
        },
        Err(err) => {
            println!("{:?}", err);
            return Err(err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_handles() {
        let request = Request::new(Body::from("{\"name\": \"blaine\", \"email_address\": \"example@email.com\"}"));

        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();

        println!("{:?}", response);
    }
}
