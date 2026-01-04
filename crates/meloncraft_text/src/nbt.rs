use meloncraft_nbt::{NbtCompound, NbtString};

#[derive(Debug, Clone)]
pub enum NbtText {
    Plain(NbtString),
    Formatted(NbtCompound),
}
