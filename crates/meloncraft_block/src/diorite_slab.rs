use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DioriteSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for DioriteSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 16286; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 16288; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 16287; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 16291; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 16290; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 16289; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16286 {
            return Some(DioriteSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16288 {
            return Some(DioriteSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16287 {
            return Some(DioriteSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 16291 {
            return Some(DioriteSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16290 {
            return Some(DioriteSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16289 {
            return Some(DioriteSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

