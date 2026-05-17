use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutRedSandstoneSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for CutRedSandstoneSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == false { return 13271; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 13268; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 13266; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 13269; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13267; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13270; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13271 {
            return Some(CutRedSandstoneSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13268 {
            return Some(CutRedSandstoneSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13266 {
            return Some(CutRedSandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13269 {
            return Some(CutRedSandstoneSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13267 {
            return Some(CutRedSandstoneSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13270 {
            return Some(CutRedSandstoneSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

