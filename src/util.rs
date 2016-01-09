use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use time::{self, Timespec};
use tiny_http::{Request, Response, StatusCode};

#[inline]
pub fn end_with_code(rep: Request, status_code: u16) {
    let res = Response::new_empty(StatusCode(status_code));
    let _ = rep.respond(res);
}

// pub fn get_short_date() -> String {
//     time::now().strftime("%Y-%m-%d").unwrap().to_string()
// }
#[inline]
pub fn get_last_modified(p: &Path) -> Option<String> {
    match fs::metadata(p) {
        Ok(ref v) => {
            let tm = &time::at(Timespec::new(v.mtime() as i64, v.mtime_nsec() as i32));
            Some(tm.to_utc().rfc822().to_string())
        }
        Err(_) => None,
    }
}

// pub fn get_file_size(file: &File) -> Option<usize> {
//     file.metadata().ok().map(|v| v.len() as usize)
// }

// pub fn str_split(s:&str, sep:&char){
//    let vec:Vec<char>= s.chars().collect();
//    let l=vec.len();
//     for variable in 0..l {
//         if &vec[variable]==sep{
//         if variable+1<l{
//         unsafe{
//         s.slice_unchecked(variable,l)

//         }
//         }
//         break;

//         }
//     }

// }
