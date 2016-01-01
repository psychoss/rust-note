extern crate note;

use std::env;

use note::server;
use note::context::Context;

fn main() {
let root=env::current_dir().unwrap();

let context=Context::new(root);
    server::new_server("0.0.0.0:9091",context);
}
