#[macro_export]
macro_rules! error_send{
	($a:expr,$c:expr) => ({
		  let rep = Response::new_empty(StatusCode($c));
    	  let _ = $a.respond(rep);
	});
}

#[macro_export]
macro_rules!take_or{
    ($expr:expr,$function:expr) => (match $expr {
        ::std::option::Option::Some(val) => val,
        ::std::option::Option::None => return $function,
    });
    ($expr:expr,$function:expr=>) => (match $expr {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(_)=>  return $function,
    });
}

