use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for BrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 13232; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13233; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13234; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13235; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13230; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13231; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13232 {
            return Some(BrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13233 {
            return Some(BrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13234 {
            return Some(BrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13235 {
            return Some(BrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13230 {
            return Some(BrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13231 {
            return Some(BrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

