use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCutCopperSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WaxedCutCopperSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 25820; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 25816; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 25815; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 25818; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 25817; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 25819; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25820 {
            return Some(WaxedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 25816 {
            return Some(WaxedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25815 {
            return Some(WaxedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 25818 {
            return Some(WaxedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25817 {
            return Some(WaxedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25819 {
            return Some(WaxedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

