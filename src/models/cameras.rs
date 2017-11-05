use chrono::DateTime;
use chrono::offset::Utc;
use models::brands::Brand;
use schema::cameras;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[belongs_to(Brand)]
pub struct Camera {
    pub id: Uuid,
    pub model: String,
    pub brand_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
