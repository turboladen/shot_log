use uuid::Uuid;

#[derive(Serialize)]
pub struct DropDown {
    pub id: Uuid,
    pub label: String,
}
