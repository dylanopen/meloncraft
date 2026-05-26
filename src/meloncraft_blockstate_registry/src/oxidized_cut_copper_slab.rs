use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCutCopperSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for OxidizedCutCopperSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == true {
            return 25449;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 25446;
        }
        if self.r#waterlogged == true && self.r#type == Type::Bottom {
            return 25447;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 25448;
        }
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 25450;
        }
        if self.r#type == Type::Top && self.r#waterlogged == true {
            return 25445;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25449 {
            return Some(OxidizedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 25446 {
            return Some(OxidizedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25447 {
            return Some(OxidizedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25448 {
            return Some(OxidizedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 25450 {
            return Some(OxidizedCutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 25445 {
            return Some(OxidizedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
