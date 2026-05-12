use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCutCopperSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WaxedExposedCutCopperSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 25812; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 25810; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 25811; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 25809; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 25813; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 25814; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25812 {
            return Some(WaxedExposedCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25810 {
            return Some(WaxedExposedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25811 {
            return Some(WaxedExposedCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25809 {
            return Some(WaxedExposedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 25813 {
            return Some(WaxedExposedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 25814 {
            return Some(WaxedExposedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

