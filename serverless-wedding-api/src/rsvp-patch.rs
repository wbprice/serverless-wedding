extern crate log;
extern crate simple_logger;

use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use std::collections::HashMap;
mod rsvp;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {

    let path_parameters = extract_path_parameters(&request);
    let payload : HashMap<String, bool> = request.payload()
        .unwrap()
        .unwrap();

    dbg!(path_parameters);
    dbg!(payload);

    Ok(())
}

fn extract_path_parameters<R>(request: &R) -> HashMap<String, String> where R: RequestExt {
    let mut result = HashMap::new();
        result.insert(String::from("hello"), String::from("world"));

    let params = request.path_parameters();
    dbg!(params);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http;

    #[test]
    fn patch_handler_handles() {

        struct MockPathParams(HashMap<String, Vec<String>>);

        impl RequestExt for MockPathParams {
            fn path_parameters(&self) -> StrMap {
                self.0.into()
            }

            fn query_string_parameters(&self) -> StrMap {
                !unreachable();
            }
        }

        let payload = r#"{
            "attending": true,
            "invitation_submitted": true,
            "reminder_submitted": true
        }"#;

        let request = http::Request::builder()
             .uri("https://api.com/")
             .method("PUT")
             .header("Content-Type", "application/json")
             .body(Body::from(payload.clone()))
             .expect("failed to build request");

        handler(request, Context::default()).expect("Expected an OK response");
    }
}
