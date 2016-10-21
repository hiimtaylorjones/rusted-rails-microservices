extern crate rustweb;
extern crate diesel;
extern crate hyper;
extern crate rustc_serialize;

use self::rustweb::*;
//use self::rustweb::models::*;
//use self::diesel::prelude::*;

use hyper::{Client};
use std::io::Read;
use rustc_serialize::json;

// Automatically generates traits to the struct
#[derive(RustcDecodable, RustcEncodable)]
pub struct PostSerializer {
    id: u8,
    title: String,
    body: String,
}

fn main() {
    for x in 1..5 {
        let url = format!("http://localhost:3000/api/v1/posts/{}", x);
        let response = get_content(&url).unwrap();
        // println!("{:?}", response);
        let decoded: PostSerializer = json::decode(&response).unwrap();
        // println!("{:?}", decoded.title);
        create_post_from_object(&decoded);
    }
}

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buffer = String::new();
    try!(response.read_to_string(&mut buffer));
    Ok(buffer)
}

fn create_post_from_object(post: &PostSerializer) {
    let connection = establish_connection();
    println!("==========================================================");
    println!("Title: {}", post.title);
    println!("==========================================================\n");
    println!("{}\n", post.body);
    create_post(&connection, &post.title, &post.body);
}
