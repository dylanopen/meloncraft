use crate::NbtTag;

#[derive(Debug, Clone)]
pub struct NbtCompound(pub Vec<NbtTag>);
