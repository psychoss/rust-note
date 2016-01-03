use body_parser;
use rustc_serialize::json::Object;
use tiny_http::{Request, Response, StatusCode};
use database::Db;

pub fn push(mut req: Request, db: &Db) {

    match body_parser::parse(&mut req) {
        Some(v) => {
            // post_handler::push(v);
            let o = v.as_object();
            if o.is_none() {
                error_send!(req, 400);
                return;
            }
            let json = o.unwrap();
            let id = get_i64("_id", json);
            let title = get_string("title", json);
            let cat = get_string("cat", json);
            let content = get_string("content", json);
            let create = get_i64("create", json);
            let modified = get_i64("modified", json);
            if id == 0i64 {

                let res = Response::from_string(db.save(title, cat, content, create, modified)
                                                  .to_string());
                let_ = req.respond(res);
            } else {
                error_send!(req, 400);
            }

            // println!("{}-{}-{}-{}-{}", id,title,content,create,modified);

        }
        None => {
            error_send!(req, 400);
        }
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
