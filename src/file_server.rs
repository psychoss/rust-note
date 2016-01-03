use std::path::Path;
use std::fs::File;
use tiny_http::{Request,Response};
use header;
use util;

pub fn serve(req:&Request,p: &Path) -> Result<Response<File>, u16> {
    let last_modified = util::get_last_modified(p).unwrap();
    if !check_modified(&req, &last_modified) {

        return Err(304);
    }
    match File::open(p) {
        Ok(v) => {

            let mut res = Response::from_file(v);
            header::set_file_header(p, &last_modified,&mut res);
            Ok(res)
        }
        Err(_) => Err(404),
    }

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
