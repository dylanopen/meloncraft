use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NbtString(pub String);

impl Deref for NbtString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for NbtString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl From<String> for NbtString {
    fn from(value: String) -> Self {
        return Self(value);
    }
}

impl From<&str> for NbtString {
    fn from(value: &str) -> Self {
        return value.to_owned().into();
    }
}
