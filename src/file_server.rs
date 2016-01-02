use std::path::{Path, PathBuf};
use std::fs::File;
use tiny_http::Response;

pub fn serve(p: &Path) -> Option<Response<File>> {
    // let last_modified = util::get_last_modified(p).unwrap();
    // if !check_modifed(&req, &last_modified) {
    //     error_end(req, 304);
    //     return;
    // }
    match File::open(p) {
        Ok(v) => {

            let mut res = Response::from_file(v);
            // res = set_file_header(p, res);
            Some(res)
        }
        Err(_) =>  None,
    }

}
