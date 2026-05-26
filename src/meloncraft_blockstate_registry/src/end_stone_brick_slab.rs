use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndStoneBrickSlab {
    pub r#type: Type,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for EndStoneBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Bottom && self.r#waterlogged == false {
            return 16247;
        }
        if self.r#type == Type::Double && self.r#waterlogged == true {
            return 16248;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 16246;
        }
        if self.r#waterlogged == false && self.r#type == Type::Top {
            return 16245;
        }
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 16249;
        }
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 16244;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16247 {
            return Some(EndStoneBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16248 {
            return Some(EndStoneBrickSlab {
                r#type: Type::Double,
                r#waterlogged: true,
            });
        }
        if state_id == 16246 {
            return Some(EndStoneBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16245 {
            return Some(EndStoneBrickSlab {
                r#waterlogged: false,
                r#type: Type::Top,
            });
        }
        if state_id == 16249 {
            return Some(EndStoneBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 16244 {
            return Some(EndStoneBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        return None;
    }
}
