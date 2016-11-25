use domain::Pet;

use std::collections::HashMap;

pub trait PersistsPets {
    // CRUD operations
    fn create(&mut self, &pet: &Pet) -> u32;
    fn get(&self, pet_id: &u32) -> Pet;
    fn update(&mut self, pet: &Pet) -> Pet;
    fn delete(&mut self, pet_id: &u32) -> u32;

    // TODO complete the signatures for these
    fn find_by_status(&self);
    fn find_by_tags(&self);
}

pub struct PetsInMemory {
    pub pets: HashMap<u32, Pet>
}

impl PetsInMemory {
    pub fn new() -> PetsInMemory {
        PetsInMemory {
            pets: HashMap::new()
        }
    }
}

impl PersistsPets for PetsInMemory {
    fn create(&mut self, pet: &Pet) -> u32 {
        let id: u32 = (*self).pets.len() as u32;
        &self.pets.insert(id, (*pet).clone());

        id
    }

    fn get(&self, pet_id: &u32) -> Pet {
        match (*self).pets.get(&pet_id) {
            Some(pet) => (*pet).clone(),
            _ => {
                panic!("Failed to get Pet with ID {}", pet_id);
            }
        }
    }

    fn update(&mut self, pet: &Pet) -> Pet {
        unimplemented!()
    }

    fn delete(&mut self, pet_id: &u32) -> u32 {
        unimplemented!()
    }

    // TODO For now just return all pets
    fn find_by_status(&self) {
    }

    // TODO For now just return all pets
    fn find_by_tags(&self) {
        unimplemented!()
    }
}