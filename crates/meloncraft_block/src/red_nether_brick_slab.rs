use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedNetherBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for RedNetherBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 16277; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 16278; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 16276; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 16274; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 16279; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 16275; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16277 {
            return Some(RedNetherBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16278 {
            return Some(RedNetherBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16276 {
            return Some(RedNetherBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16274 {
            return Some(RedNetherBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16279 {
            return Some(RedNetherBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16275 {
            return Some(RedNetherBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

