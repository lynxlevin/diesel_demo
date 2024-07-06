use std::env::args;

use diesel::prelude::*;
use diesel_demo::*;

use models::Post;

fn main() {
    use schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Not Found"),
        Err(_) => println!("An error occured while fetching post {}", post_id),
    }
}
