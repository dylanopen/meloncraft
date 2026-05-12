use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for OakSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13130; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13128; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13132; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13131; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13129; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13133; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13130 {
            return Some(OakSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13128 {
            return Some(OakSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13132 {
            return Some(OakSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13131 {
            return Some(OakSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13129 {
            return Some(OakSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13133 {
            return Some(OakSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

