use super::models::users::CurrentUser;
use rocket::request::FlashMessage;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::hash::{Hash, Hasher};

pub struct FlashContext<'a, 'r> {
    pub flash_message: FlashMessage<'a, 'r>,
}

impl<'a, 'r> FlashContext<'a, 'r> {
    pub fn new(flash_message: FlashMessage<'a, 'r>) -> Self {
        FlashContext {
            flash_message: flash_message,
        }
    }

    fn css_class(&self) -> &str {
        match self.flash_message.name() {
            "success" => "success",
            "warning" => "warning",
            "error" => "danger",
            _ => "info",
        }
    }

    fn message(&self) -> &str {
        self.flash_message.msg()
    }
}

impl<'a, 'r> Hash for FlashContext<'a, 'r> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.css_class().hash(state);
        self.message().hash(state);
    }
}

impl<'a, 'r> Serialize for FlashContext<'a, 'r> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FlashContext", 2)?;

        state.serialize_field("message", self.message())?;
        state.serialize_field("css_class", self.css_class())?;
        state.end()
    }
}

#[derive(Serialize, Hash)]
pub struct EmptyResourceContext<'a, 'r> {
    pub current_user: Option<CurrentUser>,
    pub flash: Option<FlashContext<'a, 'r>>,
}

#[derive(Serialize, Hash)]
pub struct ListResourcesContext<'a, 'b, 'r, T> {
    pub current_user: Option<CurrentUser>,
    pub flash: Option<FlashContext<'a, 'r>>,
    pub name: &'b str,
    pub resources: Vec<T>,
}

#[derive(Serialize, Hash)]
pub struct SingleResourceContext<'a, 'b, 'r, T> {
    pub current_user: Option<CurrentUser>,
    pub flash: Option<FlashContext<'a, 'r>>,
    pub name: &'b str,
    pub resource: T,
}
