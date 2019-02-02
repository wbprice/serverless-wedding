extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{json};
use log::{info, error};
use uuid::Uuid;

mod rsvp;

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

    match rsvp::RSVP::list_by_household_id(uuid) {
        Ok(rsvps) => Ok(json!(rsvps)),
        Err(_) => Ok(json!({"message": "Failed to retrieve RSVPs"}))
    }
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