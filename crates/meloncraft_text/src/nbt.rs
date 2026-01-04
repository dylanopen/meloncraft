use meloncraft_nbt::{NbtCompound, NbtString};

pub enum NbtText {
    Plain(NbtString),
    Formatted(NbtCompound),
}
