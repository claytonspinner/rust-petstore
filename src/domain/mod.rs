extern crate rustc_serialize;

use std::str::FromStr;

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct Pet {
    pub id: u32,
    category: Category,
    name: String,
    photo_urls: Vec<String>,
    tags: Vec<Tag>,
    pub status: Status
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

#[derive(Clone, RustcEncodable, RustcDecodable, PartialEq)]
pub enum Status {
    Available,
    Pending,
    Sold
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Status, ()> {
        match s {
            "Available" => Ok(Status::Available),
            "Pending" => Ok(Status::Pending),
            "Sold" => Ok(Status::Sold),
            _ => Err(()),
        }
    }
}