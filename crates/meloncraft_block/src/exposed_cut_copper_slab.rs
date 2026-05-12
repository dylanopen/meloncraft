use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCutCopperSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for ExposedCutCopperSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 25457; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 25459; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 25458; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 25460; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 25461; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 25462; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25457 {
            return Some(ExposedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 25459 {
            return Some(ExposedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25458 {
            return Some(ExposedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25460 {
            return Some(ExposedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25461 {
            return Some(ExposedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 25462 {
            return Some(ExposedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

