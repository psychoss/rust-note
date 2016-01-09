use body_parser;
use rustc_serialize::json::{self, Object};
use tiny_http::{Request, Response, StatusCode};
use database::Db;
use util::end_with_code;

// Insert the data into database
pub fn push(mut req: Request, db: &Db) {
    // Parse the body from the request
    // if failed,then end the request with status code 400 (Bad Request)
    let body = take_or!(body_parser::parse(&mut req), end_with_code(req, 400));

    // Convert the body to BTreeMap
    // if failed,then end the request with status code 400 (Bad Request)
    let json = take_or!(body.as_object(), end_with_code(req, 400));

    // Parse the field value from json into proper type
    let id = get_string("_id", json).parse::<i64>().unwrap_or(0);
    let title = get_string("title", json);
    let cat = get_string("cat", json);
    let content = get_string("content", json);
    let modified = get_i64("modified", json);

    // if the 'id' equal 0,
    //
    if id == 0i64 {
        let create = get_i64("create", json);
        let r = db.save(title, cat, content, create, modified);
        if r != 1 {
            error_send!(req, 500);
        } else {
            let res = Response::from_string(db.last_insert_id().to_string());
            let _ = req.respond(res);
        }
    } else {
        let r = db.update(id, title, cat, content, modified);
        if r != 1 {
            error_send!(req, 500);
        } else {
            error_send!(req, 200);
        }
    }

}

pub fn query(req: Request, db: &Db) {
    match db.get_list() {
        Some(v) => {
            match json::encode(&v) {
                Ok(v_s) => {
                    let res = Response::from_string(v_s);
                    let _ = req.respond(res);
                }
                Err(_) => {
                    error_send!(req, 500);
                }
            }
        }
        None => {
            error_send!(req, 500);

        }
    }
}
pub fn query_cat_list(req: Request, db: &Db) {
    match db.get_cat() {
        Some(v) => {
            match json::encode(&v) {
                Ok(v_s) => {
                    let res = Response::from_string(v_s);
                    let _ = req.respond(res);
                }
                Err(_) => {
                    error_send!(req, 500);
                }
            }
        }
        None => {
            error_send!(req, 500);

        }
    }
}
pub fn query_cat(mut req: Request, db: &Db) {
    match body_parser::parse(&mut req) {
        Some(v) => {
            let o = v.as_object();
            if o.is_none() {
                error_send!(req, 400);
                return;
            }

            let json = o.unwrap();
            let cat = get_string("cat", json);

            if cat.len() > 0 {
                match db.get_list_by(cat) {
                    Some(v) => {
                        match json::encode(&v) {
                            Ok(v_s) => {
                                let res = Response::from_string(v_s);
                                let _ = req.respond(res);
                            }
                            Err(_) => {
                                error_send!(req, 500);
                            }
                        }
                    }
                    None => {
                        error_send!(req, 500);

                    }
                }
            } else {
                error_send!(req, 400);
            }
        }
        None => {
            error_send!(req, 400);
        }
    }
}

pub fn query_one(mut req: Request, db: &Db) {


    let body = take_or!(body_parser::parse(&mut req), end_with_code(req, 400));
    let json = take_or!(body.as_object(), end_with_code(req, 400));
    let id = get_string("_id", json).parse::<i64>().unwrap_or(0);

    if id != 0i64 {
        let r = db.get_one(id);

        let res = Response::from_string(r);
        let _ = req.respond(res);
    } else {
        end_with_code(req, 400);
    }



}
#[inline]
fn get_i64(key: &str, json: &Object) -> i64 {
    match json.get(key) {
        Some(v) => v.as_i64().unwrap_or(0i64),
        None => 0i64,
    }
}

#[inline]
fn get_string(key: &str, json: &Object) -> String {
    match json.get(key) {
        Some(v) => v.as_string().unwrap_or("").to_string(),
        None => "".to_string(),
    }

}
