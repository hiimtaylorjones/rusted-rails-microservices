extern crate rustweb;
extern crate diesel;

use self::diesel::prelude::*;
use self::rustweb::*;
use std::env::args;

fn main() {
    use rustweb::schema::posts::dsl::*;

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts)
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
