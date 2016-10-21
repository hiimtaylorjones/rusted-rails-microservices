extern crate rustweb;
extern crate diesel;

use self::diesel::prelude::*;
use self::rustweb::*;
use std::env::args;

fn main() {
    use rustweb::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = posts.filter(user_id.eq(pattern))
        .execute(&connection)
        .expect("Error finding posts for user");
}
