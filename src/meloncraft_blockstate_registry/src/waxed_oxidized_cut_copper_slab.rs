use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCutCopperSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for WaxedOxidizedCutCopperSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == true { return 25801; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 25799; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 25802; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 25798; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 25797; }
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 25800; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25801 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 25799 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25802 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 25798 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25797 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 25800 {
            return Some(WaxedOxidizedCutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

