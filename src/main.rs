extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Mutex, Arc};

#[derive(RustcEncodable, RustcDecodable)]
struct Data {
    msg: String
}

fn main() {
    let mut router = Router::new();

    let data = Arc::new(Mutex::new(Data { msg: "Default Message".into() }));
    let data_clone = data.clone();

    router.get("/", move |r: &mut Request|
        hello_world(r, &data.lock().unwrap()));
    router.post("/set", move |r: &mut Request|
        set_data(r, &mut data_clone.lock().unwrap()));

    fn hello_world(_: &mut Request, data: &Data) -> IronResult<Response> {
        let payload = json::encode(&data).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_data(request: &mut Request, data: &mut Data) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload);
        *data = json::decode(&payload).unwrap();
        let payload = json::encode(data).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
