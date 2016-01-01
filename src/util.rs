use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use time::{self, Timespec};

pub fn get_last_modified(p: &Path) -> Option<String> {
    match fs::metadata(p) {
        Ok(ref v) => {
            let tm = &time::at(Timespec::new(v.mtime() as i64, v.mtime_nsec() as i32));
            Some(tm.to_utc().rfc822().to_string())
        }
        Err(err) => None,
    }
}
