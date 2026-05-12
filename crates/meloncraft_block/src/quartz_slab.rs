use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuartzSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for QuartzSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 13257; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 13256; }
        if self.r#waterlogged == true && self.r#type == Type::Top { return 13254; }
        if self.r#waterlogged == false && self.r#type == Type::Top { return 13255; }
        if self.r#waterlogged == true && self.r#type == Type::Double { return 13258; }
        if self.r#waterlogged == false && self.r#type == Type::Double { return 13259; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13257 {
            return Some(QuartzSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 13256 {
            return Some(QuartzSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13254 {
            return Some(QuartzSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13255 {
            return Some(QuartzSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 13258 {
            return Some(QuartzSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        if state_id == 13259 {
            return Some(QuartzSlab {
                r#waterlogged: false,
                r#type: Type::Double,
            });
        }
        return None;
    }
}

