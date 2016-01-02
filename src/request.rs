use context::Context;
use file_server;
use header::*;
use std::fs::{self, File};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use time::{self, Timespec};
use tiny_http::{Method, Header, Request, Response, StatusCode};
use url::Url;
use util;
macro_rules! send {
    ($a:expr,$b:expr,$c:expr) => (match $b {
       Some(val) => {
        $a.respond(val);
       },
       None => {
                     let res=Response::new_empty(StatusCode($c));
                     $a.respond(res);
        }
    })
}

pub struct Req {
    context: Arc<Context>,
}

impl Req {
    pub fn new(con: Arc<Context>) -> Req {
        Req { context: con }
    }

    pub fn dispatch(&self, req: Request) {
        match req.method() {
            &Method::Get => {
                self.get(req);
            }
            _ => {}
        }
    }

    pub fn get(&self, req: Request) {
        let uri: &str = &req.url().to_string();
        let mut p = self.context.root.clone();
        let url = Url::new(uri, &self.context);
        match url.path {
            Some(ref v) => {
                send!(req, file_server::serve(v), 404);
            }
            None => {
                error_end(req,404);
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


fn check_modified(r: &Request, last_modified: &str) -> bool {
    for v in r.headers() {
        if v.field.equiv(&"If-Modified-Since") {
            if v.value.as_str() == last_modified {
                return false;
            }
        }
    }
    true
}
