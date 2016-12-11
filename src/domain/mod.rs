extern crate rustc_serialize;


#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct Pet {
    pub id: u32,
    category: Category,
    name: String,
    photo_urls: Vec<String>,
    tags: Vec<Tag>,
    status: Status
}

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct Category {
    id: u32,
    name: String
}

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct Tag {
    id: u32,
    name: String
}

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub enum Status {
    Available,
    Pending,
    Sold
}