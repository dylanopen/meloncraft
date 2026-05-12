use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrappedChest {
    pub r#type: Type,
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for TrappedChest {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#type == Type::Single { return 11023; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Left && block_state.r#facing == Facing::East { return 11025; }
        if block_state.r#type == Type::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 11027; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 11028; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 11019; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 11026; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#type == Type::Single { return 11017; }
        if block_state.r#facing == Facing::East && block_state.r#type == Type::Single && block_state.r#waterlogged == false { return 11024; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 11006; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 11018; }
        if block_state.r#type == Type::Right && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 11010; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 11005; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 11011; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 11021; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 11020; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Right && block_state.r#facing == Facing::North { return 11009; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#type == Type::Single { return 11012; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Right && block_state.r#facing == Facing::South { return 11015; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 11016; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 11022; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Left && block_state.r#waterlogged == true { return 11007; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Left && block_state.r#facing == Facing::North { return 11008; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 11013; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#type == Type::Left { return 11014; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11023 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#type: Type::Single,
            });
        }
        if state_id == 11025 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 11027 {
            return Some(TrappedChest {
                r#type: Type::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 11028 {
            return Some(TrappedChest {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 11019 {
            return Some(TrappedChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 11026 {
            return Some(TrappedChest {
                r#type: Type::Left,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 11017 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#type: Type::Single,
            });
        }
        if state_id == 11024 {
            return Some(TrappedChest {
                r#facing: Facing::East,
                r#type: Type::Single,
                r#waterlogged: false,
            });
        }
        if state_id == 11006 {
            return Some(TrappedChest {
                r#type: Type::Single,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 11018 {
            return Some(TrappedChest {
                r#type: Type::Single,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 11010 {
            return Some(TrappedChest {
                r#type: Type::Right,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 11005 {
            return Some(TrappedChest {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 11011 {
            return Some(TrappedChest {
                r#type: Type::Single,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 11021 {
            return Some(TrappedChest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 11020 {
            return Some(TrappedChest {
                r#type: Type::Left,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 11009 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 11012 {
            return Some(TrappedChest {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#type: Type::Single,
            });
        }
        if state_id == 11015 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 11016 {
            return Some(TrappedChest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 11022 {
            return Some(TrappedChest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 11007 {
            return Some(TrappedChest {
                r#facing: Facing::North,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 11008 {
            return Some(TrappedChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 11013 {
            return Some(TrappedChest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 11014 {
            return Some(TrappedChest {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#type: Type::Left,
            });
        }
        return None;
    }
}

