use std::path::Path;
use tiny_http::{Header, Response};
use util::*;
use std::io::Read;

 const CONTENT_TYPE:&'static str ="Content-Type";
// const LAST_MODIFIED:&'static str ="Last-Modified";
// const IF_MODIFIED_SINCE:&'static str ="If-Modified-Since";
// const ACCEPT_ENCODING:&'static str="Accept-Encoding";


pub fn get_header_value( name:&'static str,header:&[Header])->Option<String>{
    for v in header {
        if v.field.equiv(&name) {
            return Some(v.value.as_str().to_string())
        }
    }
    None
}
pub fn get_content_type(header:&[Header])->Option<String>{
    get_header_value(CONTENT_TYPE,header)
    
}

// A simple way to set the response headers.
pub fn set_file_header<T:Read>(p: &Path, modified:&str,res: &mut Response<T>)  {
    let ext = match p.extension() {
        Some(v) => v.to_str().unwrap(),
        None => "",
    };
    if ext.len() > 0 {
       match ext {
            "css" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"text/css; charset=utf-8"[..])
                                    .unwrap());
            }
            "js" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/x-javascript"[..])
                                    .unwrap());
            }
            "jpg" | "jpeg" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..], &b"image/jpeg"[..])
                                    .unwrap());
            }
            "gif" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..], &b"image/gif"[..])
                                    .unwrap());
            }

            "html" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"text/html; charset=utf-8"[..])
                                    .unwrap());
            }
            "ico" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"image/vnd.microsoft.icon"[..])
                                    .unwrap());
            }
            "woff" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/font-woff"[..])
                                    .unwrap());
            }
            "woff2" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/font-woff2"[..])
                                    .unwrap());
            }
            "eot" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/vnd.ms-fontobject"[..])
                                    .unwrap());
            }
            "ttf" | "otf" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/font-sfnt"[..])
                                    .unwrap());
            }
            "svg" => {
                res.add_header(Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..])
                                    .unwrap());
            }
            _ => {},
        };
        // 31536000 seconds => One Year
        res.add_header(Header::from_bytes(&b"Cache-Control"[..],
                                             &b"public, max-age=31536000"[..])
                              .unwrap());
        let last_modified = get_last_modified(p).unwrap();
        res.add_header(Header::from_bytes(&b"Date"[..], &last_modified.as_bytes()[..]).unwrap());
        res.add_header(Header::from_bytes(&b"Last-Modified"[..], &modified.as_bytes()[..])
                              .unwrap());

    }

}
