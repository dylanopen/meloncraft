use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedBlackstoneBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Double { return 22049; }
        if self.r#waterlogged == true && self.r#type == Type::Double { return 22048; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 22047; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 22046; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 22044; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 22045; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22049 {
            return Some(PolishedBlackstoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 22048 {
            return Some(PolishedBlackstoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 22047 {
            return Some(PolishedBlackstoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 22046 {
            return Some(PolishedBlackstoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 22044 {
            return Some(PolishedBlackstoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 22045 {
            return Some(PolishedBlackstoneBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

