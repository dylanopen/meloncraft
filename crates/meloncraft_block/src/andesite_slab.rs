use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndesiteSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for AndesiteSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 16270; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 16269; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 16271; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 16272; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 16268; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 16273; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16270 {
            return Some(AndesiteSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16269 {
            return Some(AndesiteSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16271 {
            return Some(AndesiteSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16272 {
            return Some(AndesiteSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 16268 {
            return Some(AndesiteSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16273 {
            return Some(AndesiteSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

