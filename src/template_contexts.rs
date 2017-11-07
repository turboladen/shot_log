use rocket::request::FlashMessage;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use super::models::users::CurrentUser;

pub struct FlashContext {
    pub flash_message: FlashMessage,
}

impl FlashContext {
    pub fn new(flash_message: FlashMessage) -> Self {
        FlashContext { flash_message: flash_message }
    }
}

impl Serialize for FlashContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("FlashContext", 2)?;

        let css_class = match self.flash_message.name() {
            "success" => "success",
            "warning" => "warning",
            "error" => "danger",
            _ => "info"
        };

        state.serialize_field("message", self.flash_message.msg())?;
        state.serialize_field("css_class", css_class)?;
        state.end()
    }
}

#[derive(Serialize)]
pub struct EmptyResourceContext {
    pub current_user: Option<CurrentUser>,
    pub flash: Option<FlashContext>,
}

#[derive(Serialize)]
pub struct ListResourcesContext<'a, T> {
    pub current_user: Option<CurrentUser>,
    pub flash: Option<FlashContext>,
    pub name: &'a str,
    pub resources: Vec<T>,
}

#[derive(Serialize)]
pub struct SingleResourceContext<'a, T> {
    pub current_user: Option<CurrentUser>,
    pub flash: Option<FlashContext>,
    pub name: &'a str,
    pub resource: T,
}
