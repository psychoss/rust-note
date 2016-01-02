use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use time::{self, Timespec};
use std::slice;

use std::fs::File;


pub fn get_last_modified(p: &Path) -> Option<String> {
    match fs::metadata(p) {
        Ok(ref v) => {
            let tm = &time::at(Timespec::new(v.mtime() as i64, v.mtime_nsec() as i32));
            Some(tm.to_utc().rfc822().to_string())
        }
        Err(err) => None,
    }
}
pub fn get_file_size(file:&File)->Option<usize>{
    file.metadata().ok().map(|v| v.len() as usize)
}
//unsafe { url.slice_unchecked(1, url.len()) };

pub fn truncate_after(s: &str, mid: usize) -> &str {
    let (_, after) = s.split_at(mid);
           println!("a{:?}",after);
   after
   
}
pub fn truncate_after_by(s: &str, c: char) -> &str {
   match  s.find(c){
       Some(v)=>{
              truncate_after(s,v)
       }
       None=>{
         s
           
       }
   }
  
  
}

pub fn truncate_before(s: &str, mid: usize) -> &str {
    let (before, _) = s.split_at(mid);
    before
}
pub fn truncate_before_by(s: &str, c: char) -> &str {
   match  s.find(c){
       Some(v)=>{
              truncate_before(s,v)
       }
       None=>{
         s
           
       }
   }
  
  
}
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
