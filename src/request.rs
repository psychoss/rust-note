use context::Context;
use database::Db;
use file_server;

use std::sync::Arc;
use tiny_http::{Method, Request, Response, StatusCode};
use url::Url;

use post_handler;


const ROUTE_PUSH: &'static str = "/push";

// A macro for send.
macro_rules! send {
    ($a:expr,$b:expr) => (match $b {
       Ok(val) => {
        let _=$a.respond(val);
       },
       Err(v) => {
                     let res=Response::new_empty(StatusCode(v));
                     let _=$a.respond(res);
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

    pub fn dispatch(&self, req: Request) {
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

    fn get(&self, req: Request) {
        let uri: &str = &req.url().to_string();
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
    fn post(&self, req: Request) {
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

fn error_end(req: Request, status_code: u16) {
    let rep = Response::new_empty(StatusCode(status_code));
    let _ = req.respond(rep);
}
