use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraniteSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for GraniteSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 16262; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 16265; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 16266; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 16267; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 16264; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 16263; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16262 {
            return Some(GraniteSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16265 {
            return Some(GraniteSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16266 {
            return Some(GraniteSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 16267 {
            return Some(GraniteSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 16264 {
            return Some(GraniteSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16263 {
            return Some(GraniteSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

