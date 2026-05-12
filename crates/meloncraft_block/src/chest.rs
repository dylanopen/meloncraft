use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chest {
    pub waterlogged: bool,
    pub r#type: Type,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Single,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Chest {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#type == Type::Right { return 3796; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 3808; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#type == Type::Right { return 3790; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#type == Type::Single { return 3799; }
        if block_state.r#type == Type::Single && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 3793; }
        if block_state.r#facing == Facing::West && block_state.r#type == Type::Single && block_state.r#waterlogged == true { return 3798; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Single && block_state.r#waterlogged == true { return 3786; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 3806; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 3791; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 3803; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 3802; }
        if block_state.r#type == Type::Single && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 3792; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 3800; }
        if block_state.r#type == Type::Right && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 3797; }
        if block_state.r#facing == Facing::South && block_state.r#type == Type::Left && block_state.r#waterlogged == false { return 3795; }
        if block_state.r#facing == Facing::South && block_state.r#type == Type::Left && block_state.r#waterlogged == true { return 3794; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 3807; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#type == Type::Left { return 3801; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 3809; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Single && block_state.r#waterlogged == false { return 3787; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#type == Type::Left { return 3789; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 3804; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Single && block_state.r#facing == Facing::East { return 3805; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 3788; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3796 {
            return Some(Chest {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#type: Type::Right,
            });
        }
        if state_id == 3808 {
            return Some(Chest {
                r#type: Type::Right,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 3790 {
            return Some(Chest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Right,
            });
        }
        if state_id == 3799 {
            return Some(Chest {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#type: Type::Single,
            });
        }
        if state_id == 3793 {
            return Some(Chest {
                r#type: Type::Single,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 3798 {
            return Some(Chest {
                r#facing: Facing::West,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 3786 {
            return Some(Chest {
                r#facing: Facing::North,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 3806 {
            return Some(Chest {
                r#type: Type::Left,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 3791 {
            return Some(Chest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 3803 {
            return Some(Chest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 3802 {
            return Some(Chest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 3792 {
            return Some(Chest {
                r#type: Type::Single,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 3800 {
            return Some(Chest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 3797 {
            return Some(Chest {
                r#type: Type::Right,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 3795 {
            return Some(Chest {
                r#facing: Facing::South,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 3794 {
            return Some(Chest {
                r#facing: Facing::South,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 3807 {
            return Some(Chest {
                r#type: Type::Left,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 3801 {
            return Some(Chest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 3809 {
            return Some(Chest {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 3787 {
            return Some(Chest {
                r#facing: Facing::North,
                r#type: Type::Single,
                r#waterlogged: false,
            });
        }
        if state_id == 3789 {
            return Some(Chest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 3804 {
            return Some(Chest {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 3805 {
            return Some(Chest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::East,
            });
        }
        if state_id == 3788 {
            return Some(Chest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

