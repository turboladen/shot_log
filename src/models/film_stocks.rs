use chrono::offset::Utc;
use chrono::DateTime;
use models::brands::Brand;
use models::film_formats::FilmFormat;
use schema::film_stocks;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(Brand)]
#[belongs_to(FilmFormat)]
pub struct FilmStock {
    pub id: Uuid,
    pub box_name: String,
    pub box_speed: Option<i32>,
    pub brand_id: Uuid,
    pub film_format_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct SerializableFilmStock {
    pub film_stock: FilmStock,
    pub brand: Brand,
    pub film_format: FilmFormat,
}
