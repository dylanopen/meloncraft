use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrismarineBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PrismarineBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Top && self.r#waterlogged == true {
            return 12678;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == false {
            return 12681;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 12682;
        }
        if self.r#waterlogged == false && self.r#type == Type::Double {
            return 12683;
        }
        if self.r#waterlogged == false && self.r#type == Type::Top {
            return 12679;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 12680;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12678 {
            return Some(PrismarineBrickSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12681 {
            return Some(PrismarineBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12682 {
            return Some(PrismarineBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 12683 {
            return Some(PrismarineBrickSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        if state_id == 12679 {
            return Some(PrismarineBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 12680 {
            return Some(PrismarineBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
