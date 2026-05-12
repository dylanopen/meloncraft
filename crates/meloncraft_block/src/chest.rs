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
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#type == Type::Right { return 3791; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#type == Type::Left { return 3788; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#type == Type::Left { return 3794; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#type == Type::Left { return 3789; }
        if self.r#type == Type::Single && self.r#facing == Facing::West && self.r#waterlogged == false { return 3799; }
        if self.r#type == Type::Single && self.r#waterlogged == true && self.r#facing == Facing::South { return 3792; }
        if self.r#waterlogged == true && self.r#type == Type::Right && self.r#facing == Facing::East { return 3808; }
        if self.r#facing == Facing::West && self.r#type == Type::Single && self.r#waterlogged == true { return 3798; }
        if self.r#type == Type::Left && self.r#facing == Facing::East && self.r#waterlogged == true { return 3806; }
        if self.r#type == Type::Right && self.r#facing == Facing::South && self.r#waterlogged == true { return 3796; }
        if self.r#facing == Facing::South && self.r#type == Type::Right && self.r#waterlogged == false { return 3797; }
        if self.r#type == Type::Right && self.r#facing == Facing::West && self.r#waterlogged == true { return 3802; }
        if self.r#facing == Facing::West && self.r#type == Type::Right && self.r#waterlogged == false { return 3803; }
        if self.r#type == Type::Single && self.r#waterlogged == false && self.r#facing == Facing::East { return 3805; }
        if self.r#waterlogged == false && self.r#type == Type::Right && self.r#facing == Facing::East { return 3809; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#type == Type::Single { return 3793; }
        if self.r#type == Type::Single && self.r#facing == Facing::East && self.r#waterlogged == true { return 3804; }
        if self.r#waterlogged == false && self.r#type == Type::Left && self.r#facing == Facing::East { return 3807; }
        if self.r#type == Type::Right && self.r#waterlogged == true && self.r#facing == Facing::North { return 3790; }
        if self.r#waterlogged == false && self.r#type == Type::Left && self.r#facing == Facing::South { return 3795; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#type == Type::Single { return 3787; }
        if self.r#facing == Facing::West && self.r#type == Type::Left && self.r#waterlogged == false { return 3801; }
        if self.r#type == Type::Single && self.r#facing == Facing::North && self.r#waterlogged == true { return 3786; }
        if self.r#type == Type::Left && self.r#waterlogged == true && self.r#facing == Facing::West { return 3800; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3791 {
            return Some(Chest {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#type: Type::Right,
            });
        }
        if state_id == 3788 {
            return Some(Chest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Left,
            });
        }
        if state_id == 3794 {
            return Some(Chest {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 3789 {
            return Some(Chest {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#type: Type::Left,
            });
        }
        if state_id == 3799 {
            return Some(Chest {
                r#type: Type::Single,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 3792 {
            return Some(Chest {
                r#type: Type::Single,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 3808 {
            return Some(Chest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 3798 {
            return Some(Chest {
                r#facing: Facing::West,
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
        if state_id == 3796 {
            return Some(Chest {
                r#type: Type::Right,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 3797 {
            return Some(Chest {
                r#facing: Facing::South,
                r#type: Type::Right,
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
        if state_id == 3803 {
            return Some(Chest {
                r#facing: Facing::West,
                r#type: Type::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 3805 {
            return Some(Chest {
                r#type: Type::Single,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 3809 {
            return Some(Chest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 3793 {
            return Some(Chest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 3804 {
            return Some(Chest {
                r#type: Type::Single,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 3807 {
            return Some(Chest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 3790 {
            return Some(Chest {
                r#type: Type::Right,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 3795 {
            return Some(Chest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 3787 {
            return Some(Chest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 3801 {
            return Some(Chest {
                r#facing: Facing::West,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 3786 {
            return Some(Chest {
                r#type: Type::Single,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 3800 {
            return Some(Chest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

