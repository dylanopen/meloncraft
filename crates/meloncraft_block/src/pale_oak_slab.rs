use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PaleOakSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13171; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 13172; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13173; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13175; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13170; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13174; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13171 {
            return Some(PaleOakSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13172 {
            return Some(PaleOakSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13173 {
            return Some(PaleOakSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13175 {
            return Some(PaleOakSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13170 {
            return Some(PaleOakSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13174 {
            return Some(PaleOakSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        return None;
    }
}

