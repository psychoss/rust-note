use context::Context;
use std::sync::Arc;
use std::path::Path;

use std::fs::File;
use tiny_http::{Method, Request, Response};


pub struct Req {
    context: Arc<Context>,
}

impl Req {
    pub fn dispatch(&self, req: Request) {
        match req.method() {
            &Method::Get => {}
            _ => {}
        }
        self.parse_url(req)
    }
    pub fn new(con: Arc<Context>) -> Req {
        Req { context: con }
    }
    fn parse_url(&self, req: Request) {
        let url={req.url().clone()};
        if url == "/" {
            let p = self.context.root.as_path();
            self.file_server(&p,req);
        } else {
            let ext = url.split(".").last().unwrap();

        }
    }
    fn file_server(&self, p: &Path, req: Request) {
        let file = match File::open(p) {
            Ok(v) => v,
            Err(_) => {
                return;
            }
        };
        let res = Response::from_file(file);
        req.respond(res);
    }
}
