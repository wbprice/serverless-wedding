use lambda_http::{lambda, IntoResponse, Request, Body};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{{json}};
use std::ops::Deref;

mod rsvp;

fn main() {
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {
    let body = request.body().deref();
    let new_rsvp: rsvp::NewRSVP = serde_json::from_slice(body).unwrap_or_else(|err| {
        println!("error: {:?}", err);
        panic!("oh no");
    });

    let rsvp = rsvp::RSVP::new(new_rsvp);

    Ok(json!(rsvp))
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
