use crate::schema::brands;
use chrono::offset::Utc;
use chrono::DateTime;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable, Serialize)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
