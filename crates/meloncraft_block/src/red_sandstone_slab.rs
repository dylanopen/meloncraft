use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedSandstoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for RedSandstoneSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13263; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 13262; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13264; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13261; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13265; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13260; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13263 {
            return Some(RedSandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13262 {
            return Some(RedSandstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13264 {
            return Some(RedSandstoneSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13261 {
            return Some(RedSandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13265 {
            return Some(RedSandstoneSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13260 {
            return Some(RedSandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

