extern crate note;
extern crate rusqlite;

use note::context::Context;
use note::server;
use rusqlite::Connection;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::process::Command;

// Everything starts here.

fn main() {

 

    init_data_base();

    let mut root = env::current_dir().unwrap();
    root.push("public");
    let context = Context::new(root);


    let out = Command::new("google-chrome")
                  .arg("http://localhost:9091")
                  .output();
    match out {
        Ok(v) => {
            println!("stdout: {}", String::from_utf8_lossy(&v.stdout));
        }
        Err(v) => {
            println!("{:?}", v);
        }
    };

// Start the server.
    server::new_server("0.0.0.0:9091", context);
}

// Initialize the database
fn init_data_base() {
    
    // Build the Path for where the database will be placed
    let mut root = env::current_dir().unwrap();
    root.push("database/doc.db");

// Only continue if the database is not exists.

    if !root.is_file() {
        match Connection::open(root) {
            Ok(v) => {
                match get_sql() {
                    Ok(ref sql) => {
                       for s in sql.split("---"){
                           // Execute the sql command which parse from 'sql' File
                        let _=   v.execute(s, &[]);
                       }
                       let _=v.close();
                    }
                    Err(_) => {}
                }
               
            }
            Err(v) => {
                println!("{:?}", v);
            }
        }
    }
}

fn get_sql() -> io::Result<String> {
    let mut root = env::current_dir().unwrap();
    
    // A file in which was some sql statement that divided by “---”
    root.push("database/sql");

    let mut file = try!(File::open(root));
    let mut buffer: Vec<u8> = Vec::new();
    // Read all into the buffer ,and then convert to string.
    try!(file.read_to_end(&mut buffer));
    unsafe { Ok(String::from_utf8_unchecked(buffer)) }
}
