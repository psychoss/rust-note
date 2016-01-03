extern crate note;
extern crate rusqlite;

use note::context::Context;
use note::server;
use rusqlite::Connection;
use std::env;
use std::process::Command;


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


    server::new_server("0.0.0.0:9091", context);
}

fn init_data_base(){
    let mut root = env::current_dir().unwrap();
    root.push("database/doc.db");
    
    if !root.is_file(){
        match Connection::open(root){
            Ok(v)=>{
                v.execute(r#""#,&[]);
            }
            Err(v)=>{
                println!("{:?}",v);
            }
        }
    }
}