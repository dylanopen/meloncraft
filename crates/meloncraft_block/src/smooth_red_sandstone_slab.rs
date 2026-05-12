use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothRedSandstoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for SmoothRedSandstoneSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 16220; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 16225; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 16223; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 16224; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 16222; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 16221; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16220 {
            return Some(SmoothRedSandstoneSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16225 {
            return Some(SmoothRedSandstoneSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16223 {
            return Some(SmoothRedSandstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16224 {
            return Some(SmoothRedSandstoneSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 16222 {
            return Some(SmoothRedSandstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16221 {
            return Some(SmoothRedSandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

