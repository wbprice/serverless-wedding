#[macro_use]
extern crate cpython;
#[macro_use]
extern crate lando;

use lando::RequestExt;

gateway!(|request, _| {
    println!("{:?}", request.path_parameters());
    Ok(lando::Response::new(format!(
        "hello {}",
        request
            .path_parameters()
            .get("name")
            .cloned()
            .unwrap_or_else(|| "stranger".to_owned())
    )))
});
