use rusqlite::Connection;
use std::env;
use rusqlite::types::Null;

pub struct Db {
    pub con: Connection,
}
impl Db {
    pub fn save(&self, title: String, cat: String, content: String, create: i64, modified: i64)->i64 {


        let result = self.con.execute("INSERT OR REPLACE  INTO markdown VALUES ($1, $2, $3, $4, $5, \
                                  $6)",
                                &[&Null,
                                 &title,
                                 &cat,
                                 &content,
                                 &create,
                                 &modified]);
        match result {
            Ok(v) => v as i64,
            Err(_) => 0i64,
        }


    }
    pub fn new() -> Db {
        let mut root = env::current_dir().unwrap();
        root.push("database/doc.db");
        Db { con: Connection::open(root).unwrap() }
    }
}
