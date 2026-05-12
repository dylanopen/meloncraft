use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCutCopperSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WeatheredCutCopperSlab {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 25454; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 25455; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 25456; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 25451; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 25452; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 25453; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25454 {
            return Some(WeatheredCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25455 {
            return Some(WeatheredCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 25456 {
            return Some(WeatheredCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 25451 {
            return Some(WeatheredCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 25452 {
            return Some(WeatheredCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25453 {
            return Some(WeatheredCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

