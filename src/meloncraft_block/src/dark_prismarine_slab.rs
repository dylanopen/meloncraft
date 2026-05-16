use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkPrismarineSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for DarkPrismarineSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Double && self.r#waterlogged == true { return 12688; }
        if self.r#type == Type::Double && self.r#waterlogged == false { return 12689; }
        if self.r#type == Type::Bottom && self.r#waterlogged == true { return 12686; }
        if self.r#waterlogged == false && self.r#type == Type::Top { return 12685; }
        if self.r#type == Type::Top && self.r#waterlogged == true { return 12684; }
        if self.r#type == Type::Bottom && self.r#waterlogged == false { return 12687; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12688 {
            return Some(DarkPrismarineSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 12689 {
            return Some(DarkPrismarineSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 12686 {
            return Some(DarkPrismarineSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12685 {
            return Some(DarkPrismarineSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 12684 {
            return Some(DarkPrismarineSlab {
                r#type: Type::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12687 {
            return Some(DarkPrismarineSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

