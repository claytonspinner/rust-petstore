extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate petstore;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use petstore::domain::*;
//use petstore::service;
//use petstore::persistence;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {

    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello, World".to_string() }));
    let greeting_clone = greeting.clone();

    let pets = Arc::new(Mutex::new(HashMap::new()));
    let pets_clone = pets.clone();

    let mut router = Router::new();

    router.get("/pets", move |r: &mut Request| get_pets(r, &pets.lock().unwrap()), "get_pets");
    router.post("/pets", move |r: &mut Request| set_pets(r, &mut pets_clone.lock().unwrap()), "set_pets");


    fn get_pets(request: &mut Request, pets: &HashMap<String, Pet>) -> IronResult<Response> {
        let payload = json::encode(&pets).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_pets(request: &mut Request, pets: &mut HashMap<String, Pet>) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let length = pets.len().to_string();
        pets.insert(length, json::decode(&payload).unwrap());
        Ok(Response::with(status::Ok))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}