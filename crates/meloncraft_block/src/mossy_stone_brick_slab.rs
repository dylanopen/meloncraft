use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyStoneBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for MossyStoneBrickSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 16227; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 16229; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 16231; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 16226; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 16230; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 16228; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16227 {
            return Some(MossyStoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 16229 {
            return Some(MossyStoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16231 {
            return Some(MossyStoneBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16226 {
            return Some(MossyStoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16230 {
            return Some(MossyStoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 16228 {
            return Some(MossyStoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

