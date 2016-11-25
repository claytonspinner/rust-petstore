extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {

    let mut pets: Arc<Mutex<HashMap<u32, &Pet>>> = Arc::new(Mutex::new(HashMap::new()));

    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello, World".to_string() }));
    let greeting_clone = greeting.clone();

    let pets = Arc::new(Mutex::new(HashMap::new()));
    let pets_clone = pets.clone();

    let mut router = Router::new();

    router.get("/get_greeting", move |r: &mut Request| hello_world(r, &greeting.lock().unwrap()), "get_greeting");
    router.post("/set_greeting", move |r: &mut Request| set_greeting(r, &mut greeting_clone.lock().unwrap()), "set_greeting");

    router.get("/pets", move |r: &mut Request| get_pets(r, &pets.lock().unwrap()), "get_pets");
    router.post("/pets", move |r: &mut Request| set_pets(r, &mut pets_clone.lock().unwrap()), "set_pets");


    /*router.get("/", move |r: &mut Request| get(r, &pets.lock().unwrap()), "get");
    router.post("/", move |r: &mut Request| set(r, &mut pets_clone.lock().unwrap()), "set");*/

    fn hello_world(_: &mut Request, greeting: &Greeting) -> IronResult<Response> {
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_greeting(request: &mut Request, greeting: &mut Greeting) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        *greeting = json::decode(&payload).unwrap();
        Ok(Response::with(status::Ok))
    }

    fn get_pets(request: &mut Request, pets: &HashMap<String, Pet>) -> IronResult<Response> {
        let payload = json::encode(&pets).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_pets(request: &mut Request, pets: &mut HashMap<String, Pet>) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let length = (pets.len()).to_string();
        pets.insert(length, json::decode(&payload).unwrap());
        Ok(Response::with(status::Ok))
    }

    fn test(_: &mut Request) -> IronResult<Response> {
        let pet = Pet {
            id: 0,
            category: Category {
                id: 0,
                name: "Dog".to_string()
            },
            name: "Muffins".to_string(),
            photo_urls: Vec::new(),
            tags: Vec::new(),
            status: Status::Available,
        };
        let payload = json::encode(&pet).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn add_pet(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let pet: Pet = json::decode(&payload).unwrap();
        //pets.insert(pet.id, pet);
        let payload = json::encode(&pet).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}

#[derive(RustcEncodable, RustcDecodable)]
struct Pet {
    id: u32,
    category: Category,
    name: String,
    photo_urls: Vec<String>,
    tags: Vec<Tag>,
    status: Status
}

#[derive(RustcEncodable, RustcDecodable)]
struct Category {
    id: u32,
    name: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct Tag {
    id: u32,
    name: String
}

#[derive(RustcEncodable, RustcDecodable)]
enum Status {
    Available,
    Pending,
    Sold
}