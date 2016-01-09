use std::path::Path;
use std::fs::File;
use tiny_http::{Request, Response};
use header;
use util;

pub fn serve(req: &Request, p: &Path) -> Result<Response<File>, u16> {
    let last_modified = take_or!(util::get_last_modified(p), Err(404));
    if !check_modified(&req, &last_modified) {
        return Err(304);
    }
    let file = take_or!(File::open(p), Err(404)=>);
    let mut res = Response::from_file(file);
    header::set_file_header(p, &last_modified, &mut res);
    Ok(res)

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
