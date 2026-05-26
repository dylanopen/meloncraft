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
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == true {
            return 25463;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == false {
            return 25466;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 25465;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 25467;
        }
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 25468;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 25464;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25463 {
            return Some(CutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25466 {
            return Some(CutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25465 {
            return Some(CutCopperSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25467 {
            return Some(CutCopperSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 25468 {
            return Some(CutCopperSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 25464 {
            return Some(CutCopperSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
