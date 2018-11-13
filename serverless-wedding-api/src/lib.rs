#[macro_use] extern crate cpython;
#[macro_use] extern crate lando;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate uuid;

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

        let rsvp = rsvp::create_rsvp(payload);

        println!("hello rsvp {:?}", rsvp);

        Ok(Response::new("create"))
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
