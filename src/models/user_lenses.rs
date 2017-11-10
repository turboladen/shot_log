use chrono::DateTime;
use chrono::offset::Utc;
use models::lenses::Lens;
use models::users::User;
use rocket_contrib::UUID;
use schema::user_lenses;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(Lens)]
#[belongs_to(User)]
#[table_name = "user_lenses"]
pub struct UserLens {
    pub id: Uuid,
    pub user_id: Uuid,
    pub lens_id: Uuid,
    pub serial_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromForm)]
pub struct UserLensForm {
    pub lens_id: UUID,
    pub serial_number: Option<String>,
}

#[derive(Insertable)]
#[table_name = "user_lenses"]
pub struct NewUserLens {
    pub user_id: Uuid,
    pub lens_id: Uuid,
    pub serial_number: Option<String>,
}
