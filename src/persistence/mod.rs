use domain::Pet;

use std::collections::HashMap;

pub trait PersistsPets: 'static {
    // CRUD operations
    fn create(&mut self, &pet: &Pet) -> u32;
    fn get_all(&self) -> &HashMap<u32, Pet>;
    fn get(&self, pet_id: &u32) -> Option<&Pet>;
    fn update(&mut self, pet: &Pet) -> Option<&Pet>;
    fn delete(&mut self, pet_id: &u32) -> Option<Pet>;

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

    fn get_all(&self) -> &HashMap<u32, Pet> {
        &(*self).pets
    }

    fn get(&self, pet_id: &u32) -> Option<&Pet> {
        (*self).pets.get(pet_id).clone()
    }

    fn update(&mut self, pet: &Pet) -> Option<&Pet> {
        unimplemented!()
    }

    fn delete(&mut self, pet_id: &u32) -> Option<Pet> {
        (*self).pets.remove(pet_id)
    }

    // TODO For now just return all pets
    fn find_by_status(&self) {
    }

    // TODO For now just return all pets
    fn find_by_tags(&self) {
        unimplemented!()
    }
}