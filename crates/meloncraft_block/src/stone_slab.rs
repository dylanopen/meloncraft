use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for StoneSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13197; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 13196; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13198; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 13199; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13194; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13195; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13197 {
            return Some(StoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13196 {
            return Some(StoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13198 {
            return Some(StoneSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13199 {
            return Some(StoneSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13194 {
            return Some(StoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13195 {
            return Some(StoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

