use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCutCopperSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WaxedWeatheredCutCopperSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 25803; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 25805; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 25808; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 25807; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 25804; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 25806; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25803 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 25805 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25808 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 25807 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 25804 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25806 {
            return Some(WaxedWeatheredCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

