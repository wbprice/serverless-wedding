#[macro_use] extern crate cpython;
#[macro_use] extern crate lando;

use lando::Response;

mod models;

gateway! {
    "create" => |_, _| {
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
