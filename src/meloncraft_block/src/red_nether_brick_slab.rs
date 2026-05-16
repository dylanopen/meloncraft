use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedNetherBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for RedNetherBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#type == Type::Bottom { return 16277; }
        if self.r#waterlogged == false && self.r#type == Type::Top { return 16275; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 16279; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 16274; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 16276; }
        if self.r#waterlogged == true && self.r#type == Type::Double { return 16278; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16277 {
            return Some(RedNetherBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16275 {
            return Some(RedNetherBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 16279 {
            return Some(RedNetherBrickSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 16274 {
            return Some(RedNetherBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 16276 {
            return Some(RedNetherBrickSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 16278 {
            return Some(RedNetherBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        return None;
    }
}

