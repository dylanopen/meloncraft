use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuartzSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for QuartzSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13256; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 13257; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13258; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13259; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 13254; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 13255; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13256 {
            return Some(QuartzSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13257 {
            return Some(QuartzSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13258 {
            return Some(QuartzSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13259 {
            return Some(QuartzSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13254 {
            return Some(QuartzSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13255 {
            return Some(QuartzSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

