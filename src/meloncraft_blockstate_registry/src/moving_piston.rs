use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovingPiston {
    pub r#type: Type,
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Normal,
    Sticky,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for MovingPiston {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::Down && self.r#type == Type::Sticky {
            return 2120;
        }
        if self.r#type == Type::Normal && self.r#facing == Facing::West {
            return 2115;
        }
        if self.r#facing == Facing::North && self.r#type == Type::Normal {
            return 2109;
        }
        if self.r#type == Type::Normal && self.r#facing == Facing::East {
            return 2111;
        }
        if self.r#type == Type::Sticky && self.r#facing == Facing::North {
            return 2110;
        }
        if self.r#type == Type::Sticky && self.r#facing == Facing::Up {
            return 2118;
        }
        if self.r#facing == Facing::South && self.r#type == Type::Normal {
            return 2113;
        }
        if self.r#facing == Facing::Up && self.r#type == Type::Normal {
            return 2117;
        }
        if self.r#type == Type::Sticky && self.r#facing == Facing::West {
            return 2116;
        }
        if self.r#facing == Facing::East && self.r#type == Type::Sticky {
            return 2112;
        }
        if self.r#facing == Facing::South && self.r#type == Type::Sticky {
            return 2114;
        }
        if self.r#type == Type::Normal && self.r#facing == Facing::Down {
            return 2119;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2120 {
            return Some(MovingPiston {
                r#facing: Facing::Down,
                r#type: Type::Sticky,
            });
        }
        if state_id == 2115 {
            return Some(MovingPiston {
                r#type: Type::Normal,
                r#facing: Facing::West,
            });
        }
        if state_id == 2109 {
            return Some(MovingPiston {
                r#facing: Facing::North,
                r#type: Type::Normal,
            });
        }
        if state_id == 2111 {
            return Some(MovingPiston {
                r#type: Type::Normal,
                r#facing: Facing::East,
            });
        }
        if state_id == 2110 {
            return Some(MovingPiston {
                r#type: Type::Sticky,
                r#facing: Facing::North,
            });
        }
        if state_id == 2118 {
            return Some(MovingPiston {
                r#type: Type::Sticky,
                r#facing: Facing::Up,
            });
        }
        if state_id == 2113 {
            return Some(MovingPiston {
                r#facing: Facing::South,
                r#type: Type::Normal,
            });
        }
        if state_id == 2117 {
            return Some(MovingPiston {
                r#facing: Facing::Up,
                r#type: Type::Normal,
            });
        }
        if state_id == 2116 {
            return Some(MovingPiston {
                r#type: Type::Sticky,
                r#facing: Facing::West,
            });
        }
        if state_id == 2112 {
            return Some(MovingPiston {
                r#facing: Facing::East,
                r#type: Type::Sticky,
            });
        }
        if state_id == 2114 {
            return Some(MovingPiston {
                r#facing: Facing::South,
                r#type: Type::Sticky,
            });
        }
        if state_id == 2119 {
            return Some(MovingPiston {
                r#type: Type::Normal,
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}
