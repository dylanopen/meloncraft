use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCutCopperSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WaxedOxidizedCutCopperSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Top { return 25797; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 25801; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 25800; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 25802; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == false { return 25798; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 25799; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25797 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 25801 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 25800 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25802 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 25798 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25799 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

