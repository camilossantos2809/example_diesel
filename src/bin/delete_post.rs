extern crate example_diesel;
extern crate diesel;

use self::diesel::prelude::*;
use self::example_diesel::*;
use std::env::args;

fn main() {
    use example_diesel::schema::posts::dsl::*;
    let target = args().nth(1).expect("Expect a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");
    
    println!("Deleted {} posts", num_deleted);
}