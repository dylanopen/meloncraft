use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedTuffSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for PolishedTuffSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 23665; }
        if self.r#type == Type::Double && self.r#waterlogged == true { return 23666; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 23662; }
        if self.r#waterlogged == true && self.r#type == Type::Bottom { return 23664; }
        if self.r#waterlogged == false && self.r#type == Type::Top { return 23663; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 23667; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23665 {
            return Some(PolishedTuffSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 23666 {
            return Some(PolishedTuffSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 23662 {
            return Some(PolishedTuffSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 23664 {
            return Some(PolishedTuffSlab {
                r#waterlogged: true,
                r#type: Type::Bottom,
            });
        }
        if state_id == 23663 {
            return Some(PolishedTuffSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 23667 {
            return Some(PolishedTuffSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

