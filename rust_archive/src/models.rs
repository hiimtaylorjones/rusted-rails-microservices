#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author_id: i32,
}

#[derive(Author)]
pub struct Author {
    pub id: i32,
    pub title: String,
    pub body: String,
}

use super::schema::posts;
use super::schema::authors;

#[insertable_into(posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub author_id: i32,
}

#[insertable_into(authors)]
pub struct NewAuthor<'a> {
    pub name: &'a str,
    pub email: &'a str,
}
