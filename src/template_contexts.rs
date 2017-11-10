use rocket::request::FlashMessage;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::hash::{Hash, Hasher};
use super::models::users::CurrentUser;

pub struct FlashContext {
    pub flash_message: FlashMessage,
}

impl FlashContext {
    pub fn new(flash_message: FlashMessage) -> Self {
        FlashContext { flash_message: flash_message }
    }

    fn css_class(&self) -> &str {
        match self.flash_message.name() {
            "success" => "success",
            "warning" => "warning",
            "error" => "danger",
            _ => "info"
        }
    }

    fn message(&self) -> &str {
        self.flash_message.msg()
    }
}

impl Hash for FlashContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.css_class().hash(state);
        self.message().hash(state);
    }
}

impl Serialize for FlashContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("FlashContext", 2)?;

        state.serialize_field("message", self.message())?;
        state.serialize_field("css_class", self.css_class())?;
        state.end()
    }
}

#[derive(Serialize, Hash)]
pub struct EmptyResourceContext {
    pub current_user: Option<CurrentUser>,
    pub flash: Option<FlashContext>,
}

#[derive(Serialize, Hash)]
pub struct ListResourcesContext<'a, T> {
    pub current_user: Option<CurrentUser>,
    pub flash: Option<FlashContext>,
    pub name: &'a str,
    pub resources: Vec<T>,
}

#[derive(Serialize, Hash)]
pub struct SingleResourceContext<'a, T> {
    pub current_user: Option<CurrentUser>,
    pub flash: Option<FlashContext>,
    pub name: &'a str,
    pub resource: T,
}
