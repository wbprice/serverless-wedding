#[macro_use] extern crate cpython;
#[macro_use] extern crate lando;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use lando::Response;
use serde_json::{Value, Error};

mod models;

gateway! {
    "create" => |request, _context| {
        let rsvp : models::CreateRSVP = serde_json::from_slice(request.body()).unwrap();
        println!("{:#?}", rsvp);
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
