#[macro_export]
macro_rules! error_send{
	($a:expr,$c:expr) => ({
		  let rep = Response::new_empty(StatusCode($c));
    	  let _ = $a.respond(rep);
	});
}

