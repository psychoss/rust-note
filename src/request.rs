use context::Context;
use database::Db;
use file_server;
use post_handler;
use std::sync::Arc;
use tiny_http::{Method, Request, Response, StatusCode};
use url::Url;
use util;


// A few consts to map the  uri of the request

// Insert into or update the database
const ROUTE_PUSH: &'static str = "/push";
const ROUTE_UPDATE: &'static str = "/update";

//
const ROUTE_QUERY: &'static str = "/query";
const ROUTE_QUERY_ONE: &'static str = "/query-one";
const ROUTE_QUERY_CAT: &'static str = "/query-cat";
const ROUTE_QUERY_CAT_LIST: &'static str = "/query-cat-list";


// A simple wrapper for the request.

pub struct Req {
    // The Context of the server
    // contains a field which represents the path in where the static  files have been placed
    context: Arc<Context>,
    // Database wrapper
    db: Db,
}

impl Req {
    // Get a instance of the Req struct.
    pub fn new(con: Arc<Context>) -> Req {
        Req {
            context: con,
            db: Db::new(),
        }
    }

    // Dispatch the request to  its corresponding handler provider。
    pub fn dispatch(&self, req: Request) {

        // Pattern matching the HTTP METHOD
        match req.method() {
            &Method::Get => {
                self.get(req);
            }
            &Method::Post => {
                self.post(req);
            }
            _ => {
                util::end_with_code(req, 400);
            }
        }
    }

    // Handle the request if it is a HTTP GET request.
    fn get(&self, req: Request) {
        let uri: &str = &req.url().to_string();
        let url = Url::new(uri, &self.context);
        let path = take_or!(url.path, util::end_with_code(req, 404));
        let res = match file_server::serve(&req, &path) {
            Ok(v) => v,
            Err(v) => {
                util::end_with_code(req, v);
                return;
            }
        };

        let _ = req.respond(res);
    }

    // Handle the request if it is a HTTP POST request.
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
                util::end_with_code(req, 404);
            }
        }
    }
}
