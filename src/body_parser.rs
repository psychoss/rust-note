use rustc_serialize::json::Json;
use header;
use tiny_http::Request;

pub fn parse(req: &mut Request) ->Option<Json>{
    let mimetype = {
        let headers = req.headers();
        header::get_content_type(&headers)
    };
    match mimetype {
        Some(ref v) => {
            if v == "application/json" {
                let mut content = String::new();
                req.as_reader().read_to_string(&mut content).unwrap();
                let json=take_or!(content.parse::<Json>(),None=>);
                Some(json)
            }else{
                None
            }
        }
        None => {
            None
        }
    }

}
