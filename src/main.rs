extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::headers::{Headers, ContentType};
use iron::mime::{Mime, TopLevel, SubLevel};

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let mut response = Response::with((status::Ok, "{\"status\":\"ok\"}"));
      
        let mut headers = Headers::new();
        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]))
        );
        
        response.headers = headers;
        Ok(response)
    }


    let _server = Iron::new(hello_world).http("localhost:8080").unwrap();
    println!("Server on http://localhost:8080");
}
