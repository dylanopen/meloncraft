use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for TuffBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 24077; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 24078; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 24075; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 24074; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 24076; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 24079; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24077 {
            return Some(TuffBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 24078 {
            return Some(TuffBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 24075 {
            return Some(TuffBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 24074 {
            return Some(TuffBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 24076 {
            return Some(TuffBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 24079 {
            return Some(TuffBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

