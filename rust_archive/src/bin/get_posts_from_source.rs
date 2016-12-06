extern crate rustweb;
extern crate diesel;
extern crate hyper;
extern crate rustc_serialize;

use self::rustweb::*;

use hyper::{Client};
use std::io::Read;
use rustc_serialize::json;
use std::env::args;

// Automatically generates traits to the struct
#[derive(RustcDecodable, RustcEncodable)]
pub struct PostSerializer {
    id: u8,
    title: String,
    body: String,
}

fn main() {
    let start_s = args().nth(1).expect("Please provide a min id");
    let start : i32 = match start_s.parse() {
        Ok(n) => {
           n
       },
       Err(_) => {
           println!("error: first argument not an integer");
           return;
       },
   };
    let stop_s = args().nth(2).expect("Please provide a max id");
    let stop : i32 = match stop_s.parse() {
        Ok(n) => {
           n
       },
       Err(_) => {
           println!("error: second argument not an integer");
           return;
       },
   };
    for x in start..stop {
        let url = format!("http://localhost:3000/api/v1/posts/{}", x);
        let response = get_content(&url).unwrap();
        let decoded: PostSerializer = json::decode(&response).unwrap();
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
