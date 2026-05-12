use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for TuffSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 23254; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 23255; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 23256; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 23252; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 23251; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 23253; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23254 {
            return Some(TuffSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 23255 {
            return Some(TuffSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 23256 {
            return Some(TuffSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 23252 {
            return Some(TuffSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 23251 {
            return Some(TuffSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 23253 {
            return Some(TuffSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

