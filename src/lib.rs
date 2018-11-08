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
        let payload : models::CreateRSVP = serde_json::from_slice(request.body()).unwrap_or_else(|error| {
            panic!("there was an error! {:?}", error);
        });

        println!("hello payload {:?}", payload);

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
