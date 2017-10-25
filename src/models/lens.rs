use chrono::DateTime;
use chrono::offset::Utc;
use models::brand::Brand;
use uuid::Uuid;
use schema::lenses;

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[belongs_to(Brand)]
#[table_name="lenses"]
pub struct Lens {
    pub id: Uuid,
    pub name: String,
    pub focal_length_min_value: f32,
    pub focal_length_min_unit: String,
    pub focal_length_max_value: Option<f32>,
    pub focal_length_max_unit: Option<String>,
    pub aperture_max: f32,
    pub aperture_min: Option<f32>,
    pub element_count: Option<u32>,
    pub group_count: Option<u32>,
    pub filter_thread_diameter_value: Option<f32>,
    pub filter_thread_diameter_unit: Option<String>,
    pub notes: Option<String>,
    pub brand_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
