use context::Context;
use std::path::PathBuf;
use std::sync::Arc;


pub struct Url {
    pub path: Option<PathBuf>,
}
impl Url {
    pub fn new(uri: &str, context: &Arc<Context>) -> Url {
        let mut root = context.root.clone();
        if uri == "/" {
            root.push("index.html");
            Url { path: Some(root) }
        } else {
            // Because the uri always start width "/"
            // have to trim it before push into PathBuf
            // let u = unsafe { uri.slice_unchecked(1, uri.len()) };
            parse(&uri[1..], root)

        }


    }
}


fn parse(uri: &str, mut root: PathBuf) -> Url {
    let v: Vec<char> = uri.chars().collect();
    let mut v_path: Vec<char> = Vec::new();
    let mut is_path = true;
    let mut has_dot = false;
    // for (i, x) in v.iter().enumerate()
    for x in v.iter() {
        if x == &'?' {
            is_path = false;
        }
        if has_dot == false && x == &'.' {
            has_dot = true;
        }
        if is_path {
            v_path.push(*x);
        }

    }
    if has_dot {
        // convert Vec<char> to String
        let s = v_path.iter().cloned().collect::<String>();
        root.push(&s);
        return Url { path: Some(root) };
    }
    Url { path: None }

}
