use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutCopperSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CutCopperSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 25464; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == false { return 25468; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == false { return 25466; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 25463; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Double { return 25467; }
        if block_state.r#type == Type::Bottom && block_state.r#waterlogged == true { return 25465; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25464 {
            return Some(CutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 25468 {
            return Some(CutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 25466 {
            return Some(CutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25463 {
            return Some(CutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25467 {
            return Some(CutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 25465 {
            return Some(CutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

