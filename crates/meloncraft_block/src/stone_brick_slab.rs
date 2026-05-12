use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for StoneBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Top { return 13237; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 13236; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 13238; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 13241; }
        if self.r#waterlogged == true && self.r#type == Type::Double { return 13240; }
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 13239; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13237 {
            return Some(StoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13236 {
            return Some(StoneBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13238 {
            return Some(StoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13241 {
            return Some(StoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13240 {
            return Some(StoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13239 {
            return Some(StoneBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

