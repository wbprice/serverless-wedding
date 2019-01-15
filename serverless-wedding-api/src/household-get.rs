extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, Request, Body};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{json};
use url::{Url, ParseError};
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
    let uri = Url::parse(&request.uri().to_string()).unwrap_or_else(|_| {
        panic!("Couldn't parse uri");
    });

    let path_segments = uri.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap_or_else(|| {
        panic!("uri didn't contain segments");
    });
    
    let uuid = Uuid::parse_str(&path_segments[1].to_string()).unwrap_or_else(|_| {
        panic!("UUID not provided");
    });

    let rsvps = rsvp::list_by_household_id(uuid);

    Ok(json!(rsvps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_handler_handles() {

        let mut request = Request::new(Body::default());
        *request.uri_mut() = "https://api.slswedding.com/household/f2de1c2b-b6e2-4caa-a5b2-fd7fe8ba7efe".parse().unwrap();

        handler(request, Context::default()).expect("expected Ok(_) value");
    }
}