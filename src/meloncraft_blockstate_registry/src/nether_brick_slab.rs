use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherBrickSlab {
    pub waterlogged: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Top,
    Bottom,
    Double,
}

impl BlockState for NetherBrickSlab {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#type == Type::Top {
            return 13248;
        }
        if self.r#type == Type::Double && self.r#waterlogged == false {
            return 13253;
        }
        if self.r#type == Type::Top && self.r#waterlogged == false {
            return 13249;
        }
        if self.r#waterlogged == false && self.r#type == Type::Bottom {
            return 13251;
        }
        if self.r#type == Type::Bottom && self.r#waterlogged == true {
            return 13250;
        }
        if self.r#waterlogged == true && self.r#type == Type::Double {
            return 13252;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13248 {
            return Some(NetherBrickSlab {
                r#waterlogged: true,
                r#type: Type::Top,
            });
        }
        if state_id == 13253 {
            return Some(NetherBrickSlab {
                r#type: Type::Double,
                r#waterlogged: false,
            });
        }
        if state_id == 13249 {
            return Some(NetherBrickSlab {
                r#type: Type::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 13251 {
            return Some(NetherBrickSlab {
                r#waterlogged: false,
                r#type: Type::Bottom,
            });
        }
        if state_id == 13250 {
            return Some(NetherBrickSlab {
                r#type: Type::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 13252 {
            return Some(NetherBrickSlab {
                r#waterlogged: true,
                r#type: Type::Double,
            });
        }
        return None;
    }
}
