use context::Context;
use std::sync::Arc;
use std::path::Path;

use std::os::unix::fs::MetadataExt;
use std::fs::{self, File};
use tiny_http::{Method, Header, Request, Response, StatusCode};
use time::{self, Timespec};
use util::*;

pub struct Req {
    context: Arc<Context>,
}

impl Req {
    pub fn dispatch(&self, req: Request) {
        match req.method() {
            &Method::Get => {}
            _ => {}
        }
        self.parse_url(req);
    }
    pub fn new(con: Arc<Context>) -> Req {
        Req { context: con }
    }
    fn parse_url(&self, req: Request) {
        let url: &str = &req.url().to_string();
        match url {
            "/" => {
                let mut p = self.context.root.clone();
                p.push("index.html");
                println!("{:?}", p.as_path());
                self.file_server(&p.as_path(), req);
            }
            _ => {

                let ext = url.split(".").last().unwrap_or("");
                println!("{:?}{}", ext, url);
                if ext.len() > 0 {
                    let mut p = self.context.root.clone();

                    // Because the url startwiths "/"
                    // so must trim the first character
                    // before push into the path bufffer
                    let file_name = unsafe { url.slice_unchecked(1, url.len()) };
                    p.push(file_name);

                    self.file_server(&p.as_path(), req);

                }
            }
        }

    }

    fn file_server(&self, p: &Path, req: Request) {
        let last_modified = get_last_modified(p).unwrap();
        if !check_modifed(&req, &last_modified) {
            error_end(req, 304);
            return;
        }
        let file = match File::open(p) {
            Ok(v) => v,
            Err(_) => {
                error_end(req, 404);
                return;
            }
        };
        let mut res = Response::from_file(file);
        res = set_file_header(p, res);
        req.respond(res);
    }
}
fn error_end(req: Request, status_code: u16) {
    let rep = Response::new_empty(StatusCode(status_code));
    let _ = req.respond(rep);
}

// A simple way to set the response headers.

fn set_file_header(p: &Path, res: Response<File>) -> Response<File> {
    let ext = match p.extension() {
        Some(v) => v.to_str().unwrap(),
        None => "",
    };
    if ext.len() > 0 {
        let mut r = match ext {
            "css" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"text/css; charset=utf-8"[..])
                                    .unwrap())
            }
            "js" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/x-javascript"[..])
                                    .unwrap())
            }
            "html" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"text/html; charset=utf-8"[..])
                                    .unwrap())
            }   
            "ico" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"image/vnd.microsoft.icon"[..])
                                    .unwrap())
            }
            _ => res,
        };
        // 31536000 seconds => One Year
        r = r.with_header(Header::from_bytes(&b"Cache-Control"[..],
                                             &b"public, max-age=31536000"[..])
                              .unwrap());
        let last_modified = get_last_modified(p).unwrap();
        r = r.with_header(Header::from_bytes(&b"Date"[..], &last_modified.as_bytes()[..]).unwrap());
        r = r.with_header(Header::from_bytes(&b"Last-Modified"[..], &last_modified.as_bytes()[..])
                              .unwrap());

        return r;
    }
    res

}
fn check_modifed(r: &Request, last_modified: &str) -> bool {
    for v in r.headers() {
        if v.field.as_str().as_str() == "If-Modified-Since" {
            if v.value.as_str() == last_modified {
                return false;
            }
        }
    }
    true
}
