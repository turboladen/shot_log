use chrono::offset::Utc;
use chrono::DateTime;
use models::brands::Brand;
use schema::cameras;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(Brand)]
pub struct Camera {
    pub id: Uuid,
    pub model: String,
    pub brand_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct SerializableCamera {
    pub camera: Camera,
    pub brand: Brand,
}
