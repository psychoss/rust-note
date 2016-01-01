use std::fs::File;
use std::path::Path;
use tiny_http::{Header, Response};
use util::*;



// A simple way to set the response headers.
pub fn set_file_header(p: &Path, res: Response<File>) -> Response<File> {
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
            "jpg" | "jpeg" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..], &b"image/jpeg"[..])
                                    .unwrap())
            }
            "gif" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..], &b"image/gif"[..])
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
            "woff" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/font-woff"[..])
                                    .unwrap())
            }
            "woff2" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/font-woff2"[..])
                                    .unwrap())
            }
            "eot" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/vnd.ms-fontobject"[..])
                                    .unwrap())
            }
            "ttf" | "otf" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..],
                                                   &b"application/font-sfnt"[..])
                                    .unwrap())
            }
            "svg" => {
                res.with_header(Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..])
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
