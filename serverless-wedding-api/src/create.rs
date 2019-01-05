use lambda_http::{lambda, IntoResponse, Request, Body};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{{json}};
use std::ops::Deref;

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
    let new_rsvp: rsvp::NewRSVP = serde_json::from_slice(body_slice).unwrap_or_else(|err| {
        println!("error: {:?}", err);
        panic!("oh no");
    });

    match create_rsvp_record(new_rsvp) {
        Ok(rsvp) => {
            Ok(json!(rsvp))
        },
        Err(error) => {
            Ok(json!({message: "Error"}))
        }
    }
}

pub fn create_rsvp_record(new_rsvp: NewRSVP) -> Result<RSVP, PutItemError>{
    let rsvp = rsvp::RSVP::new(new_rsvp);
    let rsvp : RSVP = create_rsvp(new_rsvp);
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
        let expected = json!({
            "message": "Go Serverless v1.0! Your function executed successfully!"
        })
        .into_response();

        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
