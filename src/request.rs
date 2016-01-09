use context::Context;
use database::Db;
use file_server;
use post_handler;
use std::sync::Arc;
use tiny_http::{Method, Request, Response, StatusCode};
use url::Url;
use util;



const ROUTE_PUSH: &'static str = "/push";
const ROUTE_UPDATE: &'static str = "/update";
const ROUTE_QUERY: &'static str = "/query";
const ROUTE_QUERY_ONE: &'static str = "/query-one";
const ROUTE_QUERY_CAT: &'static str = "/query-cat";
const ROUTE_QUERY_CAT_LIST: &'static str = "/query-cat-list";



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
        let path = take_or!(url.path, util::end_with_code(req, 404));
        let res = take_or!(file_server::serve(&req, &path),
                          util::end_with_code(req, 404)=>);
        let _ = req.respond(res);
    }
    fn post(&self, req: Request) {
        let uri: &str = &req.url().to_string();
        match uri {
            ROUTE_QUERY_CAT_LIST => {
                post_handler::query_cat_list(req, &self.db);
            }
            ROUTE_PUSH | ROUTE_UPDATE => {
                post_handler::push(req, &self.db);
            }
            ROUTE_QUERY => {
                post_handler::query(req, &self.db);
            }
            ROUTE_QUERY_ONE => {
                post_handler::query_one(req, &self.db);
            }
            ROUTE_QUERY_CAT => {
                post_handler::query_cat(req, &self.db);
            }
            _ => {
                error_send!(req, 404);
            }
        }
    }
}
