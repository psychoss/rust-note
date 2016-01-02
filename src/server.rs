use std::net::ToSocketAddrs;
use tiny_http::{Server,Request};
use std::sync::Arc;

use context::Context;
use request::Req;


pub fn new_server<A: ToSocketAddrs>(addr: A,context:Context) {

    let sock_addr = addr.to_socket_addrs()
                        .ok()
                        .and_then(|mut addrs| addrs.next())
                        .expect("Could not parse socket address.");

    let server = Server::http(sock_addr).unwrap();
    let arc_context=Arc::new(context);
    
    for r in server.incoming_requests() {
        Req::new(arc_context.clone()).dispatch(r);
    }
}

