use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for DarkOakSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13167; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13164; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13168; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 13165; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13166; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 13169; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13167 {
            return Some(DarkOakSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13164 {
            return Some(DarkOakSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13168 {
            return Some(DarkOakSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13165 {
            return Some(DarkOakSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13166 {
            return Some(DarkOakSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13169 {
            return Some(DarkOakSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        return None;
    }
}

