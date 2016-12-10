extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate petstore;

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::*;

use iron::prelude::*;
use router::Router;
use std::sync::{Arc, Mutex};

use petstore::controller::pet_controller::PetControllerBuilder;
use petstore::persistence::PetsInMemory;


fn main() {
    let drain = slog_term::streamer().build().fuse();
    let root_logger = slog::Logger::root(drain, o!("version" => "0.5"));
    info!(root_logger, "Application started");

    let pets_in_memory: Arc<Mutex<PetsInMemory>> = Arc::new(Mutex::new(PetsInMemory::new()));

    let pet_controller = Arc::new(Mutex::new(PetControllerBuilder::new()
        .logger(Some(root_logger))
        .pet_persistence(Some(pets_in_memory.clone()))
        .finalize()));

    let mut router = Router::new();

    {
        let pet_controller_clone = pet_controller.clone();
        router.get("/pets", move |r: &mut Request| pet_controller_clone.lock().unwrap().get_pets(), "get_pets");
    }

    {
        let pet_controller_clone = pet_controller.clone();
        router.get("/pets/:pet_id", move |r: &mut Request| pet_controller_clone.lock().unwrap().get_pet(r), "get_pet");
    }

    {
        let pet_controller_clone = pet_controller.clone();
        router.post("/pets", move |r: &mut Request| pet_controller_clone.lock().unwrap().set_pet(r), "set_pets");
    }

    {
        let pet_controller_clone = pet_controller.clone();
        router.delete("/pets/:pet_id", move |r: &mut Request| pet_controller_clone.lock().unwrap().delete_pet(r), "delete_pet");
    }

    Iron::new(router).http("localhost:3000").unwrap();
}