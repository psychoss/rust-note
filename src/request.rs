use context::Context;
use database::Db;
use file_server;
use post_handler;
use std::sync::Arc;
use tiny_http::{Method, Request, Response, StatusCode};
use url::Url;


const ROUTE_PUSH: &'static str = "/push";
const ROUTE_UPDATE: &'static str = "/update";
const ROUTE_QUERY: &'static str = "/query";
const ROUTE_QUERY_ONE: &'static str = "/query-one";

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
            _ => {
                error_send!(req, 400);
            }
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
                error_send!(req, 404);
            }
        }
    }
    fn post(&self, req: Request) {
        let uri: &str = &req.url().to_string();
        match uri {
            ROUTE_PUSH | ROUTE_UPDATE => {
                post_handler::push(req, &self.db);
            }
            ROUTE_QUERY=>{
                post_handler::query(req, &self.db);
            }
            ROUTE_QUERY_ONE=>{
                post_handler::query_one(req, &self.db);                
            }
            _ => {
                error_send!(req, 404);
            }
        }
    }
}
