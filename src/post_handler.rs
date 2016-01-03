use body_parser;
use rustc_serialize::json::Object;
use tiny_http::Request;
use database::Db;

pub fn push(mut req: Request, db: &Db) -> i64 {

    match body_parser::parse(&mut req) {
        Some(v) => {
            // post_handler::push(v);
            let json = v.as_object().unwrap();
            println!("{:?}", json);
            let id = get_i64("_id", json);
            let title = get_string("title", json);
            let cat = get_string("cat", json);
            let content = get_string("content", json);
            let create = get_i64("create", json);
            let modified = get_i64("modified", json);
            if id == 0i64 {
                db.save(title, cat, content, create, modified)

            } else {
                0i64
            }
            // println!("{}-{}-{}-{}-{}", id,title,content,create,modified);

        }
        None => 0i64,
    }


}


fn get_i64(key: &str, json: &Object) -> i64 {
    match json.get(key) {
        Some(v) => v.as_i64().unwrap_or(0i64),
        None => 0i64,
    }
}

fn get_string(key: &str, json: &Object) -> String {
    match json.get(key) {
        Some(v) => {
            match v.as_string() {
                Some(v_s) => v_s.to_string(),
                None => "".to_string(),
            }
        }
        None => "".to_string(),
    }

}
