use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for StoneBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13240; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 13236; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 13238; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13237; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13239; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 13241; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13240 {
            return Some(StoneBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13236 {
            return Some(StoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13238 {
            return Some(StoneBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13237 {
            return Some(StoneBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13239 {
            return Some(StoneBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13241 {
            return Some(StoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        return None;
    }
}

