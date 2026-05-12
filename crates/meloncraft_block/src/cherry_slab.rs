use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherrySlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CherrySlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 13159; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13161; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13160; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 13163; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13162; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13158; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13159 {
            return Some(CherrySlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13161 {
            return Some(CherrySlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13160 {
            return Some(CherrySlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13163 {
            return Some(CherrySlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13162 {
            return Some(CherrySlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13158 {
            return Some(CherrySlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

