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
            panic!("there was an error deserializing the request! {:?}", error);
        });

        let rsvp = rsvp::create_rsvp_record(payload).unwrap_or_else(|_| {
            panic!("Error at the disco");
        });

        match serde_json::to_string(&rsvp) {
            Ok(json) => {
                return Ok(Response::new(json))
            },
            Err(_) => {
                return Ok(Response::new("oh".to_string()))
            }
        }
    },

    "read" => |_, _| {
        let household_id = "4dac979d-8fe3-40e7-a00f-36b192c3a0ec";

        let rsvp_list = rsvp::list_household_rsvps(household_id.to_string()).unwrap_or_else(|error| {
            panic!("Something went badly getting an RSVP list");
        });

        match serde_json::to_string(&rsvp_list) {
            Ok(json) => {
                return Ok(Response::new(json))
            }, 
            Err(_) => {
                return Ok(Response::new("oh".to_string()))
            }
        }
    },

    "update" => |_, _| {
        Ok(Response::new("update"))
    },

    "delete" => |_, _| {
        Ok(Response::new("delete"))
    }
}
