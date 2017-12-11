use chrono::DateTime;
use chrono::offset::Utc;
use models::cameras::Camera;
use models::users::{CurrentUser, User};
use rocket_contrib::UUID;
use schema::user_cameras;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(Camera)]
#[belongs_to(User)]
#[belongs_to(CurrentUser, foreign_key = "user_id")]
pub struct UserCamera {
    pub id: Uuid,
    pub user_id: Uuid,
    pub camera_id: Uuid,
    pub roll_prefix: String,
    pub serial_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromForm)]
pub struct UserCameraForm {
    pub camera_id: UUID,
    pub roll_prefix: String,
    pub serial_number: Option<String>,
}

#[derive(Insertable)]
#[table_name = "user_cameras"]
pub struct NewUserCamera {
    pub user_id: Uuid,
    pub camera_id: Uuid,
    pub roll_prefix: String,
    pub serial_number: Option<String>,
}
