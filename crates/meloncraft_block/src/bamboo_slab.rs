use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for BambooSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13185; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13186; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13184; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 13182; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13183; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13187; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13185 {
            return Some(BambooSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13186 {
            return Some(BambooSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13184 {
            return Some(BambooSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13182 {
            return Some(BambooSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13183 {
            return Some(BambooSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13187 {
            return Some(BambooSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

