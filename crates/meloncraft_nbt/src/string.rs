use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtString(pub String);

impl Deref for NbtString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for NbtString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for NbtString {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}
