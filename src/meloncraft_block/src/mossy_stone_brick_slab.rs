use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyStoneBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for MossyStoneBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == true { return 16226; }
        if self.r#waterlogged == false && self.r#type == Type::Top { return 16227; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 16231; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 16228; }
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 16229; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 16230; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16226 {
            return Some(MossyStoneBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16227 {
            return Some(MossyStoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 16231 {
            return Some(MossyStoneBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16228 {
            return Some(MossyStoneBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16229 {
            return Some(MossyStoneBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16230 {
            return Some(MossyStoneBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

