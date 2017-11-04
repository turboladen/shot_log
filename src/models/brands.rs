use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use schema::brands;

#[derive(Identifiable, Queryable, Serialize, Associations)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
