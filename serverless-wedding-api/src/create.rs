#[macro_use]
extern crate serde_derive;

use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};

use serde;
// use serde_json::json;
use serde_dynamodb;
use uuid;
use rusoto_core;
use rusoto_dynamodb;

mod rsvp;

fn main() {
    lambda!(read_handler)
}

fn read_handler(
    request: Request,
    context: Context,
) -> Result<impl IntoResponse, HandlerError> {

    // let payload : rsvp::NewRSVP = serde_json::from_slice(request.body()).unwrap_or_else(|error| {
    //     panic!("there was an error deserializing the request! {:?}", error);
    // });

    // let rsvp = rsvp::create_rsvp_record(payload).unwrap_or_else(|_| {
    //     panic!("Error at the disco");
    // });

    // return Ok(json!(rsvp));
    return Ok(())
}
