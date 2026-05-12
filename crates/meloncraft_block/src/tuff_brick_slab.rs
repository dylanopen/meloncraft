use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for TuffBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 24074; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 24079; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 24078; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 24077; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 24075; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 24076; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24074 {
            return Some(TuffBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 24079 {
            return Some(TuffBrickSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 24078 {
            return Some(TuffBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 24077 {
            return Some(TuffBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 24075 {
            return Some(TuffBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 24076 {
            return Some(TuffBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

