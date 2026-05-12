use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CobbledDeepslateSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CobbledDeepslateSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 27808; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 27807; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 27805; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 27809; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 27810; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 27806; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27808 {
            return Some(CobbledDeepslateSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 27807 {
            return Some(CobbledDeepslateSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 27805 {
            return Some(CobbledDeepslateSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 27809 {
            return Some(CobbledDeepslateSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 27810 {
            return Some(CobbledDeepslateSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 27806 {
            return Some(CobbledDeepslateSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

