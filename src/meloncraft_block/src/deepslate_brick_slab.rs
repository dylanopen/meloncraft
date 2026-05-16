use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for DeepslateBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == false { return 29043; }
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 29041; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 29038; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 29040; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 29039; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 29042; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29043 {
            return Some(DeepslateBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 29041 {
            return Some(DeepslateBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 29038 {
            return Some(DeepslateBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 29040 {
            return Some(DeepslateBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 29039 {
            return Some(DeepslateBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 29042 {
            return Some(DeepslateBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

