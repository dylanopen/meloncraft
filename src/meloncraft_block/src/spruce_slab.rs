use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for SpruceSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13135; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 13134; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 13136; }
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 13137; }
        if self.r#waterlogged == true && self.r#type == Type::Double { return 13138; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 13139; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13135 {
            return Some(SpruceSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13134 {
            return Some(SpruceSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13136 {
            return Some(SpruceSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13137 {
            return Some(SpruceSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13138 {
            return Some(SpruceSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13139 {
            return Some(SpruceSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

