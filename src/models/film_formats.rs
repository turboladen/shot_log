use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use schema::film_formats;

#[derive(Associations, Identifiable, Queryable, Serialize)]
pub struct FilmFormat {
    pub id: Uuid,
    pub designation: String,
    pub stock_size_value: Option<f64>,
    pub stock_size_unit: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
