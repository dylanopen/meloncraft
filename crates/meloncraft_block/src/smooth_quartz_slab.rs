use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothQuartzSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for SmoothQuartzSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 16261; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 16258; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 16256; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 16259; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 16257; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 16260; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16261 {
            return Some(SmoothQuartzSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 16258 {
            return Some(SmoothQuartzSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16256 {
            return Some(SmoothQuartzSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16259 {
            return Some(SmoothQuartzSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16257 {
            return Some(SmoothQuartzSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16260 {
            return Some(SmoothQuartzSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        return None;
    }
}

