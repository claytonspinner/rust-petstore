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

use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::net::IpAddr;
use std::str::FromStr;

use petstore::controller::pet_controller::PetControllerBuilder;
use petstore::persistence::PetsInMemory;

const HOST: &'static str = "localhost";
const PORT: &'static str = "3000";

fn main() {
    let drain = slog_term::streamer().build().fuse();
    let root_logger = slog::Logger::root(drain, o!("version" => "0.5"));
    info!(root_logger, "Application starting");

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
        router.post("/pets", move |r: &mut Request| pet_controller_clone.lock().unwrap().set_pet(r), "set_pet");
    }

    {
        let pet_controller_clone = pet_controller.clone();
        router.put("/pets", move |r: &mut Request| pet_controller_clone.lock().unwrap().update_pet(r), "update_pet");
    }

    {
        let pet_controller_clone = pet_controller.clone();
        router.post("/pets/:pet_id", move |r: &mut Request| pet_controller_clone.lock().unwrap().update_pet_with_id(r), "update_pet_with_id");
    }

    {
        let pet_controller_clone = pet_controller.clone();
        router.delete("/pets/:pet_id", move |r: &mut Request| pet_controller_clone.lock().unwrap().delete_pet(r), "delete_pet");
    }

    let server = format!("{}:{}", HOST, PORT);
    Iron::new(router).http(&*server).unwrap();
}