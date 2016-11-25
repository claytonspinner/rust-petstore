pub mod pets {
    use domain::Pet;

    trait PersistsPets {
        fn create(pet: &Pet);
        fn update(pet: &Pet);
    }
}