extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate petstore;
#[macro_use] extern crate log;
extern crate env_logger;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use petstore::domain::*;
use petstore::persistence::PetsInMemory;
use petstore::persistence::PersistsPets;
use std::fmt::Debug;
use router::Params;
use std::any::Any;

use log::LogLevel::Info;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {
    env_logger::init().unwrap();

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

    {
        let pets_in_memory_clone = pets_in_memory.clone();
        router.delete("/pets/:pet_id", move |r: &mut Request| delete_pet(r, &mut pets_in_memory_clone.lock().unwrap()), "delete_pet");
    }

    fn get_pets(request: &mut Request, pets_in_memory: &PetsInMemory) -> IronResult<Response> {
        debug!("get_pets");


        let payload = json::encode(&pets_in_memory.pets).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    fn get_pet(request: &mut Request, pets_in_memory: &PetsInMemory) -> IronResult<Response> {
        debug!("get_pet");
        let pet_id: u32 = (*request.extensions.get::<Router>().unwrap().find("pet_id").unwrap()).parse::<u32>().unwrap();

        let payload = json::encode(&pets_in_memory.get(&pet_id)).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    fn set_pets(request: &mut Request, pets_in_memory: &mut PetsInMemory) -> IronResult<Response> {
        debug!("set_pets");
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        debug!("{}", &*payload);
        let pets_id = pets_in_memory.create(&json::decode(&*payload).unwrap());
        Ok(Response::with((status::Ok, json::encode(&pets_id).unwrap())))
    }

    fn delete_pet(request: &mut Request, pets_in_memory: &mut PetsInMemory) -> IronResult<Response> {
        debug!("delete_pet");
        let pet_id: u32 = (*request.extensions.get::<Router>().unwrap().find("pet_id").unwrap()).parse::<u32>().unwrap();

        // TODO Actually handle failure here
        let pet = (*pets_in_memory).delete(&pet_id).unwrap();
        let payload = json::encode(&pet).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    fn something(request: &mut Request, pets_in_memory: &PetsInMemory) -> IronResult<Response> {
        Ok(Response::with((status::Ok)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}