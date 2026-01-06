use crate::{NbtTag, NbtValue};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct NbtCompound(pub Vec<NbtTag>);

impl Deref for NbtCompound {
    type Target = Vec<NbtTag>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NbtCompound {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<NbtTag>> for NbtCompound {
    fn from(value: Vec<NbtTag>) -> Self {
        Self(value)
    }
}

impl NbtCompound {
    pub fn get(&self, key: &str) -> Option<&NbtTag> {
        if !key.contains("/") {
            return self.0.iter().find(|tag| tag.key == key);
        }
        let mut tree: Vec<&str> = key.split("/").collect();
        let mut current_compound = self;
        while !tree.is_empty() {
            if tree.len() == 1 {
                return current_compound.get(tree[0]);
            }
            let NbtValue::Compound(compound) = current_compound.get_value(tree.remove(0))? else {
                return None;
            };
            current_compound = compound;
        };
        None
    }

    pub fn get_value(&self, key: &str) -> Option<&NbtValue> {
        self.get(key).map(|tag| &tag.value)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut NbtTag> {
        if !key.contains("/") {
            return self.0.iter_mut().find(|tag| tag.key == key);
        }
        let mut tree: Vec<&str> = key.split("/").collect();
        let mut current_compound = self;
        while !tree.is_empty() {
            if tree.len() == 1 {
                return current_compound.get_mut(tree[0]);
            }
            let NbtValue::Compound(compound) = current_compound.get_value_mut(tree.remove(0))? else {
                return None;
            };
            current_compound = compound;
        };
        None
    }

    pub fn get_value_mut(&mut self, key: &str) -> Option<&mut NbtValue> {
        self.get_mut(key).map(|tag| &mut tag.value)
    }

    pub fn insert(&mut self, tag: NbtTag) {
        if let Some(existing_tag) = self.get_mut(&tag.key) {
            *existing_tag = tag;
        } else {
            self.0.push(tag);
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<NbtTag> {
        if let Some(pos) = self.0.iter().position(|tag| tag.key == key) {
            Some(self.0.remove(pos))
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.iter().any(|tag| tag.key == key)
    }

    pub fn contains_value(&self, value: &NbtValue) -> bool {
        self.0.iter().any(|tag| &tag.value == value)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn keys(&self) -> Vec<&str> {
        self.0.iter().map(|tag| tag.key.as_str()).collect()
    }

    pub fn values(&self) -> Vec<&crate::NbtValue> {
        self.0.iter().map(|tag| &tag.value).collect()
    }

    pub fn values_mut(&mut self) -> Vec<&mut crate::NbtValue> {
        self.0.iter_mut().map(|tag| &mut tag.value).collect()
    }
}
