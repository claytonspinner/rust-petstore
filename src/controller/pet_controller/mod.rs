extern crate router;
extern crate iron;

use slog;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use ::persistence::PersistsPets;
use rustc_serialize::json;
use std::io::Read;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ::domain::*;
use std::fmt::Debug;
use self::router::Params;
use std::any::Any;
use std::marker::Send;


pub struct PetControllerBuilder<T: PersistsPets> {
    logger: Option<slog::Logger>,
    pet_persistence: Option<Arc<Mutex<T>>>
}

impl<T: PersistsPets> PetControllerBuilder<T> {
    pub fn new() -> PetControllerBuilder<T> {
        PetControllerBuilder {
            logger: None,
            pet_persistence: None
        }
    }

    pub fn logger(mut self, logger: Option<slog::Logger>) -> PetControllerBuilder<T> {
        self.logger = logger;
        self
    }

    pub fn pet_persistence(mut self, pet_persistence: Option<Arc<Mutex<T>>>) -> PetControllerBuilder<T> {
        self.pet_persistence = pet_persistence;
        self
    }

    pub fn finalize(self) -> PetController<T> {
        let thing = self;
        PetController {
            logger: thing.logger.unwrap(),
            pet_persistence: thing.pet_persistence.unwrap()
        }
    }

}

pub struct PetController<T: PersistsPets> {
    logger: slog::Logger,
    pet_persistence: Arc<Mutex<T>>
}

impl<T: PersistsPets> PetController<T> {

    pub fn get_pets(&self, request: &mut Request) -> IronResult<Response> {
        info!(self.logger, "get_pets");

        // understand how this returns a reference to a HashMap but you still get the object out
        let payload = json::encode(&self.pet_persistence.lock().unwrap().get_all()).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }
}


/*fn get_pet<T: PersistsPets>(request: &mut Request, pets_in_memory: &T) -> IronResult<Response> {
    //debug!("get_pet");
    let pet_id: u32 = (*request.extensions.get::<Router>().unwrap().find("pet_id").unwrap()).parse::<u32>().unwrap();

    let payload = json::encode(&pets_in_memory.get(&pet_id)).unwrap();

    Ok(Response::with((status::Ok, payload)))
}

fn set_pets<T: PersistsPets>(request: &mut Request, pets_in_memory: &mut T) -> IronResult<Response> {
    //debug!("set_pets");
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    //debug!("{}", &*payload);
    let pets_id = pets_in_memory.create(&json::decode(&*payload).unwrap());
    Ok(Response::with((status::Ok, json::encode(&pets_id).unwrap())))
}

fn delete_pet<T: PersistsPets>(request: &mut Request, pets_in_memory: &mut T) -> IronResult<Response> {
    //debug!("delete_pet");
    let pet_id: u32 = (*request.extensions.get::<Router>().unwrap().find("pet_id").unwrap()).parse::<u32>().unwrap();

    // TODO Actually handle failure here
    let pet = (*pets_in_memory).delete(&pet_id).unwrap();
    let payload = json::encode(&pet).unwrap();

    Ok(Response::with((status::Ok, payload)))
}*/


/*{
            let pets_in_memory_clone = pet_controller.pet_persistence.clone();
            router.get("/pets/:pet_id", move |r: &mut Request| get_pet(r, &pets_in_memory_clone.lock().unwrap() as &T), "get_pet");
        }

        {
            let pets_in_memory_clone = pet_controller.pet_persistence.clone();
            router.post("/pets", move |r: &mut Request| set_pets(r, &mut pets_in_memory_clone.lock().unwrap() as &mut T), "set_pets");
        }

        {
            let pets_in_memory_clone = pet_controller.pet_persistence.clone();
            router.delete("/pets/:pet_id", move |r: &mut Request| delete_pet(r, &mut pets_in_memory_clone.lock().unwrap() as &mut T), "delete_pet");
        }*/