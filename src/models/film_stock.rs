use chrono::DateTime;
use chrono::offset::Utc;
use models::brand::Brand;
use models::film_format::FilmFormat;
use schema::film_stocks;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Associations)]
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
