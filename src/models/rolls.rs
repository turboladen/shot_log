use chrono::offset::Utc;
use chrono::{DateTime, NaiveDate};
use form_values::PlainDate;
use models::film_stocks::FilmStock;
use models::user_cameras::UserCamera;
use uuid::Uuid;
use schema::rolls;

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(FilmStock)]
#[belongs_to(UserCamera)]
pub struct Roll {
    pub id: Uuid,
    pub film_stock_id: Uuid,
    pub user_camera_id: Uuid,
    pub display_id: String,
    pub loaded_at: NaiveDate,
    pub finished_at: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromForm)]
pub struct RollForm {
    pub film_stock_id: Uuid,
    pub user_camera_id: Uuid,
    pub display_id: String,
    pub loaded_at: PlainDate,
    pub finished_at: Option<PlainDate>,
}

#[derive(Insertable)]
#[table_name = "rolls"]
pub struct NewRoll {
    pub film_stock_id: Uuid,
    pub user_camera_id: Uuid,
    pub display_id: String,
    pub loaded_at: NaiveDate,
    pub finished_at: Option<NaiveDate>,
}
