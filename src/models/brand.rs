use uuid::Uuid;
use schema::brands;

#[derive(Identifiable, Queryable, Serialize, Associations)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
}

