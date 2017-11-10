use chrono::DateTime;
use chrono::offset::Utc;
use models::brands::Brand;
use uuid::Uuid;
use schema::lenses;

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(Brand)]
#[table_name="lenses"]
pub struct Lens {
    pub id: Uuid,
    pub name: String,
    pub focal_length_min_value: f64,
    pub focal_length_min_unit: String,
    pub focal_length_max_value: Option<f64>,
    pub focal_length_max_unit: Option<String>,
    pub aperture_max: f64,
    pub aperture_min: Option<f64>,
    pub element_count: Option<i32>,
    pub group_count: Option<i32>,
    pub filter_thread_diameter_value: Option<f64>,
    pub filter_thread_diameter_unit: Option<String>,
    pub notes: Option<String>,
    pub brand_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
