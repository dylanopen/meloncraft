use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothStoneSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for SmoothStoneSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13202; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13203; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13205; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13204; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13200; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13201; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13202 {
            return Some(SmoothStoneSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13203 {
            return Some(SmoothStoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13205 {
            return Some(SmoothStoneSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13204 {
            return Some(SmoothStoneSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13200 {
            return Some(SmoothStoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13201 {
            return Some(SmoothStoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

