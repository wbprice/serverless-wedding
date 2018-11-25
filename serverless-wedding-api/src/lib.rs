#[macro_use] extern crate cpython;
#[macro_use] extern crate lando;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_dynamodb;
#[macro_use] extern crate uuid;
#[macro_use] extern crate rusoto_core;
#[macro_use] extern crate rusoto_dynamodb;

extern crate serde;
extern crate serde_json;

use lando::Response;
use serde_json::{Value, Error};

mod rsvp;

gateway! {
    "create" => |request, _context| {
        let payload : rsvp::NewRSVP = serde_json::from_slice(request.body()).unwrap_or_else(|error| {
            panic!("there was an error! {:?}", error);
        });

        let rsvp = rsvp::create_rsvp_record(payload).unwrap_or_else(|error| {
            panic!("There was a problem creating the record: {:?}", error);
        });

        match serde_json::to_string(&rsvp) {
            Ok(json) => {
                return Ok(Response::new(json))
            },
            Err(_) => {
                return Ok(Response::new("oh".to_string()));
            }
        }
    },

    "read" => |_, _| {
        Ok(Response::new("read"))
    },

    "update" => |_, _| {
        Ok(Response::new("update"))
    },

    "delete" => |_, _| {
        Ok(Response::new("delete"))
    }
}
