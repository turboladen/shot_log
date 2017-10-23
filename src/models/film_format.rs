use uuid::Uuid;
use schema::film_formats;

#[derive(Identifiable, Queryable, Serialize, Associations)]
pub struct FilmFormat {
    pub id: Uuid,
    pub designation: String,
    pub stock_size_value: Option<f64>,
    pub stock_size_unit: Option<String>,
}
