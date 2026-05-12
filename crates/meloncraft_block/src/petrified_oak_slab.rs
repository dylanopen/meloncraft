use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetrifiedOakSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PetrifiedOakSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 13218; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13221; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 13222; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 13223; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 13219; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13220; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13218 {
            return Some(PetrifiedOakSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13221 {
            return Some(PetrifiedOakSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13222 {
            return Some(PetrifiedOakSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13223 {
            return Some(PetrifiedOakSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13219 {
            return Some(PetrifiedOakSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13220 {
            return Some(PetrifiedOakSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

