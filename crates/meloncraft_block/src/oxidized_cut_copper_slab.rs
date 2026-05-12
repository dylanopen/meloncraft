use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCutCopperSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for OxidizedCutCopperSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 25449; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 25446; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 25445; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 25450; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 25448; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 25447; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25449 {
            return Some(OxidizedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 25446 {
            return Some(OxidizedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25445 {
            return Some(OxidizedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25450 {
            return Some(OxidizedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 25448 {
            return Some(OxidizedCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25447 {
            return Some(OxidizedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

