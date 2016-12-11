extern crate router;
extern crate iron;

use slog;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use ::persistence::PersistsPets;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex};

use domain::Pet;

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

    pub fn get_pets(&self) -> IronResult<Response> {
        debug!(self.logger, "get_pets");

        // understand how this returns a reference to a HashMap but you still get the object out
        let payload = json::encode(&self.pet_persistence.lock().unwrap().get_all()).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    pub fn get_pet(&self, request: &mut Request) -> IronResult<Response> {
        debug!(self.logger, "get_pet");
        let pet_id: u32 = (*request.extensions.get::<Router>().unwrap().find("pet_id").unwrap()).parse::<u32>().unwrap();

        let payload = json::encode(&self.pet_persistence.lock().unwrap().get(&pet_id)).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    pub fn set_pet(&self, request: &mut Request) -> IronResult<Response> {
        debug!(self.logger, "set_pet");
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let pets_id = &self.pet_persistence.lock().unwrap().create(&json::decode(&*payload).unwrap());
        Ok(Response::with((status::Ok, json::encode(&pets_id).unwrap())))
    }

    pub fn update_pet(&self, request: &mut Request) -> IronResult<Response> {
        debug!(self.logger, "update_pet");
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let pet = self.pet_persistence.lock().unwrap().update(json::decode(&*payload).unwrap());
        Ok(Response::with((status::Ok, json::encode(&pet).unwrap())))
    }

    pub fn update_pet_with_id(&self, request: &mut Request) -> IronResult<Response> {
        debug!(self.logger, "update_pet_with_id");
        let pet_id: u32 = (*request.extensions.get::<Router>().unwrap().find("pet_id").unwrap()).parse::<u32>().unwrap();

        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let mut pet: Pet = json::decode(&*payload).unwrap();
        pet.id = pet_id;
        let pet = self.pet_persistence.lock().unwrap().update(pet);
        Ok(Response::with((status::Ok, json::encode(&pet).unwrap())))
    }

    pub fn delete_pet(&self, request: &mut Request) -> IronResult<Response> {
        debug!(self.logger, "delete_pet");
        let pet_id: u32 = (*request.extensions.get::<Router>().unwrap().find("pet_id").unwrap()).parse::<u32>().unwrap();

        // TODO Actually handle failure here
        let pet = &self.pet_persistence.lock().unwrap().delete(&pet_id).unwrap();
        let payload = json::encode(&pet).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }
}
