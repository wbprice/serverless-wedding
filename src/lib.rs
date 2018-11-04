#[macro_use] extern crate cpython;
#[macro_use] extern crate lando;

use lando::Response;

mod models;

gateway! {
    "create" => |request, _context| {
        print!("Hey CloudFront, this is {:?}", request);
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
