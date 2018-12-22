#[macro_use]
extern crate serde_derive;

use lambda_http::{lambda, IntoResponse, Request, Body};
use lambda_runtime::{error::HandlerError, Context};

use serde;
use serde_json::{json, Value, Error};
use serde_dynamodb;
use uuid;
use rusoto_core;
use rusoto_dynamodb;

mod rsvp;

fn main() {
    lambda!(create_handler)
}

fn create_handler(
    request: Request,
    _context: Context,
) -> Result<impl IntoResponse, HandlerError> {

    println!("{:?}", request);

    return Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_handler() {

        let request = Request {
            body: Body::from("hello".to_string()),
            ..Request::default()
        };

        let response = create_handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();

        println!("{:?}", response);
    }
}