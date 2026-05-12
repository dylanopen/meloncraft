use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovingPiston {
    pub r#facing: Facing,
    pub r#type: Type,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Normal,
    Sticky,
}

impl BlockState for MovingPiston {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Normal && block_state.r#facing == Facing::South { return 2113; }
        if block_state.r#type == Type::Normal && block_state.r#facing == Facing::Up { return 2117; }
        if block_state.r#facing == Facing::Down && block_state.r#type == Type::Sticky { return 2120; }
        if block_state.r#facing == Facing::East && block_state.r#type == Type::Normal { return 2111; }
        if block_state.r#type == Type::Sticky && block_state.r#facing == Facing::West { return 2116; }
        if block_state.r#facing == Facing::West && block_state.r#type == Type::Normal { return 2115; }
        if block_state.r#type == Type::Sticky && block_state.r#facing == Facing::East { return 2112; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Normal { return 2109; }
        if block_state.r#type == Type::Sticky && block_state.r#facing == Facing::North { return 2110; }
        if block_state.r#type == Type::Sticky && block_state.r#facing == Facing::Up { return 2118; }
        if block_state.r#facing == Facing::Down && block_state.r#type == Type::Normal { return 2119; }
        if block_state.r#type == Type::Sticky && block_state.r#facing == Facing::South { return 2114; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2113 {
            return Some(MovingPiston {
                r#type: Type::Normal,
                r#facing: Facing::South,
            });
        }
        if state_id == 2117 {
            return Some(MovingPiston {
                r#type: Type::Normal,
                r#facing: Facing::Up,
            });
        }
        if state_id == 2120 {
            return Some(MovingPiston {
                r#facing: Facing::Down,
                r#type: Type::Sticky,
            });
        }
        if state_id == 2111 {
            return Some(MovingPiston {
                r#facing: Facing::East,
                r#type: Type::Normal,
            });
        }
        if state_id == 2116 {
            return Some(MovingPiston {
                r#type: Type::Sticky,
                r#facing: Facing::West,
            });
        }
        if state_id == 2115 {
            return Some(MovingPiston {
                r#facing: Facing::West,
                r#type: Type::Normal,
            });
        }
        if state_id == 2112 {
            return Some(MovingPiston {
                r#type: Type::Sticky,
                r#facing: Facing::East,
            });
        }
        if state_id == 2109 {
            return Some(MovingPiston {
                r#facing: Facing::North,
                r#type: Type::Normal,
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
        if state_id == 2119 {
            return Some(MovingPiston {
                r#facing: Facing::Down,
                r#type: Type::Normal,
            });
        }
        if state_id == 2114 {
            return Some(MovingPiston {
                r#type: Type::Sticky,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

