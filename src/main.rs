extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate petstore;

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::*;

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
use petstore::controller::*;

use std::time;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {

    let drain = slog_term::streamer().build().fuse();
    let root_logger = slog::Logger::root(drain, o!("version" => "0.5"));
    info!(root_logger, "Application started");
    //info!(root_logger, root_logger);

    let pets_in_memory: Arc<Mutex<PetsInMemory>> = Arc::new(Mutex::new(PetsInMemory::new()));

    let pets_in_memory_clone = pets_in_memory.clone();

    let mut pet1 = pet_controller::PetControllerBuilder::new();
    let pet2 = pet1.logger(Some(root_logger));
    let pet3 = pet2.pet_persistence(Some(pets_in_memory_clone));
    let pet_controller = pet3.finalize();

    let pets_in_memory_clone1 = pets_in_memory.clone();


    let mut router = Router::new();

    router.get("/pets", move |r: &mut Request| pet_controller.get_pets(r), "get_pets");

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
        //debug!(root_logger, "get_pets");


        let payload = json::encode(&pets_in_memory.pets).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    fn get_pet(request: &mut Request, pets_in_memory: &PetsInMemory) -> IronResult<Response> {
        //debug!(root_logger, "get_pet");
        let pet_id: u32 = (*request.extensions.get::<Router>().unwrap().find("pet_id").unwrap()).parse::<u32>().unwrap();
        let payload = json::encode(&pets_in_memory.get(&pet_id)).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    fn set_pets(request: &mut Request, pets_in_memory: &mut PetsInMemory) -> IronResult<Response> {
        //debug!(root_logger, "set_pets");
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        //debug!("{}", &*payload);
        let pets_id = pets_in_memory.create(&json::decode(&*payload).unwrap());
        Ok(Response::with((status::Ok, json::encode(&pets_id).unwrap())))
    }

    fn delete_pet(request: &mut Request, pets_in_memory: &mut PetsInMemory) -> IronResult<Response> {
        //debug!(root_logger, "delete_pet");
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