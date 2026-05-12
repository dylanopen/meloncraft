use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutRedSandstoneSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CutRedSandstoneSlab {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#type == Type::Bottom { return 13269; }
        if block_state.r#type == Type::Double && block_state.r#waterlogged == true { return 13270; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Double { return 13271; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Top { return 13267; }
        if block_state.r#type == Type::Top && block_state.r#waterlogged == true { return 13266; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Bottom { return 13268; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13269 {
            return Some(CutRedSandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13270 {
            return Some(CutRedSandstoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13271 {
            return Some(CutRedSandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 13267 {
            return Some(CutRedSandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13266 {
            return Some(CutRedSandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13268 {
            return Some(CutRedSandstoneSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        return None;
    }
}

