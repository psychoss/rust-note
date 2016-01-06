use rusqlite::Connection;
use std::env;
use rusqlite::types::Null;

pub struct Db {
    pub con: Connection,
}
#[derive(RustcEncodable)]
pub struct NoteItem {
    id: i64,
    title: String,
    cat: String,
}
// impl ToJson for NoteItem {
//     fn to_json(&self) -> Json {
//         Json::String(format!("{}+{}+{}i", self. id, self.title,self.cat))
//     }
// }
impl Db {
    pub fn last_insert_id(&self) -> i64 {
        self.con.last_insert_rowid()
    }
    pub fn save(&self,
                title: String,
                cat: String,
                content: String,
                create: i64,
                modified: i64)
                -> i64 {


        let result = self.con
                         .execute("INSERT OR REPLACE  INTO markdown VALUES ($1, $2, $3, $4, $5, \
                                   $6)",
                                  &[&Null, &title, &cat, &content, &create, &modified]);
        match result {
            Ok(v) => v as i64,
            Err(_) => 0i64,
        }
    }
    pub fn update(&self,
                  id: i64,
                  title: String,
                  cat: String,
                  content: String,
                  modified: i64)
                  -> i64 {
        let result = self.con.execute(" UPDATE  markdown SET title=$1, category=$2,content=$3, \
                                       modified=$4 WHERE _id = $5",
                                      &[&title, &cat, &content, &modified, &id]);
        match result {
            Ok(v) => v as i64,
            Err(_) => 0i64,
        }
    }
    pub fn get_list(&self) -> Option<Vec<NoteItem>> {
        let mut stm = self.con
                          .prepare("SELECT _id, title, category FROM markdown WHERE category = \
                                    '' ORDER BY title")
                          .unwrap();
        let it = stm.query_map(&[], |row| {
            NoteItem {
                id: row.get::<i64>(0),
                title: row.get(1),
                cat: row.get(2),
            }
        });
        match it {
            Ok(v) => {
                let mut vec: Vec<NoteItem> = Vec::new();
                for variable in v {
                    vec.push(variable.unwrap());
                }
                Some(vec)
            }
            Err(_) => None,
        }
    }
    pub fn get_list_by(&self, cat: String) -> Option<Vec<NoteItem>> {
        let mut stm = self.con
                          .prepare("SELECT _id, title, category FROM markdown WHERE category = \
                                    $1  ORDER BY title ")
                          .unwrap();
        let it = stm.query_map(&[&cat], |row| {
            NoteItem {
                id: row.get::<i64>(0),
                title: row.get(1),
                cat: row.get(2),
            }
        });
        match it {
            Ok(v) => {
                let mut vec: Vec<NoteItem> = Vec::new();
                for variable in v {
                    vec.push(variable.unwrap());
                }
                Some(vec)
            }
            Err(_) => None,
        }
    }
    pub fn get_one(&self, id: i64) -> String {
        println!("Query {:?}", id);
        match self.con.query_row("SELECT content FROM markdown WHERE _id = $1",
                                 &[&id],
                                 |row| row.get(0)) {
            Ok(v) => v,
            Err(_) => "".to_string(),
        }
    }
    pub fn new() -> Db {
        let mut root = env::current_dir().unwrap();
        root.push("database/doc.db");
        Db { con: Connection::open(root).expect("Cant open the website.") }
    }
}
