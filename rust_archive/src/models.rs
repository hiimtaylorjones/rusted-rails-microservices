use schema::posts;
use schema::authors;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author_id: i32,
}

#[derive(Queryable)]
pub struct Author {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub author_id: i32,
}

#[derive(Insertable)]
#[table_name="authors"]
pub struct NewAuthor<'a> {
    pub name: &'a str,
    pub email: &'a str,
}
