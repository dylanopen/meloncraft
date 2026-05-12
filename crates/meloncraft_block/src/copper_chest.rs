use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperChest {
    pub r#type: Type,
    pub waterlogged: bool,
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

impl BlockState for CopperChest {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Right && block_state.r#waterlogged == true { return 26897; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#type == Type::Left { return 26895; }
        if block_state.r#facing == Facing::South && block_state.r#type == Type::Right && block_state.r#waterlogged == true { return 26903; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 26898; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#type == Type::Right { return 26916; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 26893; }
        if block_state.r#facing == Facing::South && block_state.r#type == Type::Left && block_state.r#waterlogged == true { return 26901; }
        if block_state.r#facing == Facing::West && block_state.r#type == Type::Left && block_state.r#waterlogged == false { return 26908; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#type == Type::Single { return 26906; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Left && block_state.r#waterlogged == false { return 26896; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 26911; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Single && block_state.r#facing == Facing::East { return 26912; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 26894; }
        if block_state.r#facing == Facing::South && block_state.r#type == Type::Left && block_state.r#waterlogged == false { return 26902; }
        if block_state.r#type == Type::Single && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 26905; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 26913; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 26910; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Right && block_state.r#facing == Facing::East { return 26915; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 26899; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Right { return 26909; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 26904; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 26914; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Single && block_state.r#facing == Facing::South { return 26900; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Left && block_state.r#facing == Facing::West { return 26907; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26897 {
            return Some(CopperChest {
                r#facing: Facing::North,
                r#type: Type::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 26895 {
            return Some(CopperChest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Left,
            });
        }
        if state_id == 26903 {
            return Some(CopperChest {
                r#facing: Facing::South,
                r#type: Type::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 26898 {
            return Some(CopperChest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 26916 {
            return Some(CopperChest {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#type: Type::Right,
            });
        }
        if state_id == 26893 {
            return Some(CopperChest {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 26901 {
            return Some(CopperChest {
                r#facing: Facing::South,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 26908 {
            return Some(CopperChest {
                r#facing: Facing::West,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 26906 {
            return Some(CopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 26896 {
            return Some(CopperChest {
                r#facing: Facing::North,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 26911 {
            return Some(CopperChest {
                r#type: Type::Single,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 26912 {
            return Some(CopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::East,
            });
        }
        if state_id == 26894 {
            return Some(CopperChest {
                r#type: Type::Single,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26902 {
            return Some(CopperChest {
                r#facing: Facing::South,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 26905 {
            return Some(CopperChest {
                r#type: Type::Single,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26913 {
            return Some(CopperChest {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 26910 {
            return Some(CopperChest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 26915 {
            return Some(CopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 26899 {
            return Some(CopperChest {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 26909 {
            return Some(CopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        if state_id == 26904 {
            return Some(CopperChest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 26914 {
            return Some(CopperChest {
                r#type: Type::Left,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26900 {
            return Some(CopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 26907 {
            return Some(CopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

