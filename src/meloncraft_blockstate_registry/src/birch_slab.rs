use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for BirchSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 13143; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 13145; }
        if self.r#type == Type::Top && self.r#waterlogged == false { return 13141; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 13140; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 13144; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 13142; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13143 {
            return Some(BirchSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13145 {
            return Some(BirchSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13141 {
            return Some(BirchSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13140 {
            return Some(BirchSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 13144 {
            return Some(BirchSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 13142 {
            return Some(BirchSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

