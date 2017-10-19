use uuid::Uuid;
use schema::{brands, film_formats, film_stocks, users};

#[derive(Identifiable, Queryable, Serialize, Associations)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
pub struct FilmFormat {
    pub id: Uuid,
    pub designation: String,
    pub stock_size_value: Option<f64>,
    pub stock_size_unit: Option<String>,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[belongs_to(Brand)]
#[belongs_to(FilmFormat)]
pub struct FilmStock {
    pub id: Uuid,
    pub box_name: String,
    pub box_speed: Option<i32>,
    pub brand_id: Uuid,
    pub film_format_id: Uuid
}

#[derive(Identifiable, Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(FromForm)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct UserToSave {
    pub email: String,
    pub password_hash: String,
}
