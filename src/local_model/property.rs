#[derive(Queryable)]
pub struct Property {
    pub id: i32,
    pub general_location: String,
    pub price: i32,
    pub published: String,
    pub uri: String
}