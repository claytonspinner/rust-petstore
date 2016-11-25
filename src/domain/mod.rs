// extern crate rustc_serialize; <- this can't work?, investigate later


#[derive(RustcEncodable, RustcDecodable)]
pub struct Pet {
    pub id: u32,
    pub category: Category,
    pub name: String,
    pub photo_urls: Vec<String>,
    pub tags: Vec<Tag>,
    pub status: Status
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Category {
    pub id: u32,
    pub name: String
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Tag {
    pub id: u32,
    pub name: String
}

#[derive(RustcEncodable, RustcDecodable)]
pub enum Status {
    Available,
    Pending,
    Sold
}