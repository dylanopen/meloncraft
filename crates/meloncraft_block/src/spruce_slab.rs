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
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 13139; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13137; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13138; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13134; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13136; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13135; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13139 {
            return Some(SpruceSlab {
                r#waterlogged: false,
                r#type: Type::Double,
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
                r#type: Type::Double,
                r#waterlogged: true,
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
        if state_id == 13135 {
            return Some(SpruceSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

