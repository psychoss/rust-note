use context::Context;
use database::Db;
use file_server;

use std::sync::Arc;
use tiny_http::{Method, Request, Response, StatusCode};
use url::Url;

use post_handler;


const ROUTE_PUSH: &'static str = "/push";

macro_rules! send {
    ($a:expr,$b:expr) => (match $b {
       Ok(val) => {
        $a.respond(val);
       },
       Err(v) => {
                     let res=Response::new_empty(StatusCode(v));
                     $a.respond(res);
        }
    })
}

pub struct Req {
    context: Arc<Context>,
    db: Db,
}

impl Req {
    pub fn new(con: Arc<Context>) -> Req {
        Req {
            context: con,
            db: Db::new(),
        }
    }

    pub fn dispatch(&self, mut req: Request) {
        match req.method() {
            &Method::Get => {
                self.get(req);
            }
            &Method::Post => {
                self.post(req);
            }
            _ => {}
        }
    }

    fn get(&self, mut req: Request) {
        let uri: &str = &req.url().to_string();
        let mut p = self.context.root.clone();
        let url = Url::new(uri, &self.context);
        match url.path {
            Some(ref v) => {

                send!(req, file_server::serve(&req, v));
            }
            None => {
                error_end(req, 404);
            }
        }
    }
    fn post(&self, mut req: Request) {
        let uri: &str = &req.url().to_string();
        match uri {
            ROUTE_PUSH => {

                post_handler::push(req, &self.db);


            }
            _ => {
                error_end(req, 404);
            }
        }
    }
}


// p.push("index.html");
// //println!("{:?}", p.as_path());
// self.file_server(&p.as_path(), req);


// let mut ext = url.split(".").last().unwrap_or("");

// if ext.len() > 0 {

// let mut p = self.context.root.clone();

//     // Because the url startwiths "/"
//     // so must trim the first character
//     // before push into the path bufffer
//     let mut file_name =  unsafe { url.slice_unchecked(1, url.len()) };
//     file_name=util::truncate_before_by(file_name,'?');
//     //println!("{:?}",file_name);
//     p.push(file_name);

// self.file_server(&p.as_path(), req);

// }


fn error_end(req: Request, status_code: u16) {
    let rep = Response::new_empty(StatusCode(status_code));
    let _ = req.respond(rep);
}
