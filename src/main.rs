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
use petstore::persistence::PetsInMemory;
use petstore::persistence::PersistsPets;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {

    let pets_in_memory = Arc::new(Mutex::new(PetsInMemory::new()));

    let mut router = Router::new();

    {
        let pets_in_memory_clone = pets_in_memory.clone();
        router.get("/pets", move |r: &mut Request| get_pets(r, &pets_in_memory_clone.lock().unwrap()), "get_pets");
    }

    {
        let pets_in_memory_clone = pets_in_memory.clone();
        router.get("/pets/:pet_id", move |r: &mut Request| get_pet(r, &pets_in_memory_clone.lock().unwrap()), "get_pet");
    }

    {
        let pets_in_memory_clone = pets_in_memory.clone();
        router.post("/pets", move |r: &mut Request| set_pets(r, &mut pets_in_memory_clone.lock().unwrap()), "set_pets");
    }


    fn get_pets(request: &mut Request, pets_in_memory: &PetsInMemory) -> IronResult<Response> {
        let payload = json::encode(&pets_in_memory.pets).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn get_pet(request: &mut Request, pets_in_memory: &PetsInMemory) -> IronResult<Response> {
        println!("In get_pet()");
        println!("{}", request.url);
        let payload = json::encode(&pets_in_memory.get(&0_u32)).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn something(request: &mut Request, pets_in_memory: &PetsInMemory) -> IronResult<Response> {
        Ok(Response::with((status::Ok)))
    }

    fn set_pets(request: &mut Request, pets_in_memory: &mut PetsInMemory) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let length = pets_in_memory.pets.len() as u32;
        pets_in_memory.pets.insert(length, json::decode(&payload).unwrap());
        Ok(Response::with(status::Ok))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}