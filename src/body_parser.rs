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
                match  content.parse::<Json>(){
                    Ok(v)=>{
                        Some(v)
                    }
                    Err(_)=>{
                       None
                    }
                }
            }else{
                None
            }
        }
        None => {
            None
        }
    }

}
// let mut request = server.recv().unwrap();

// if get_content_type(&request) == "application/json" {
//     let mut content = String::new();
//     request.as_reader().read_to_string(&mut content).unwrap();
//     let json: Json = content.parse().unwrap();
// }
