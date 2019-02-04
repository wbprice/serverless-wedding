extern crate log;
extern crate simple_logger;

use std::error::Error;
use lambda_http::{lambda, IntoResponse, http, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{json, Value};
use uuid::Uuid;

mod models;
use crate::models::Household;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {
    let path_parameters = request.path_parameters();
    let uuid : Uuid = Uuid::parse_str(
        path_parameters.get("id").unwrap()
    ).unwrap();

    Ok(match Household::get(uuid) {
        Ok(rsvps) => {
            http::Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .status(200)
                .body(json!(rsvps).to_string())
                .unwrap()
        },
        Err(err) => {
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
    use lambda_http::Body;

    #[test]
    #[ignore]
    fn create_handler_handles() {

        let mut request = Request::new(Body::default());
        *request.uri_mut() = "https://api.slswedding.com/household/3eb28445-7698-4a00-b071-49da8eaac944".parse().unwrap();

        handler(request, Context::default()).expect("Expected an OK response");
    }
}