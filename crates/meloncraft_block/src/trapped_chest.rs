use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrappedChest {
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#type: Type,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Single,
    Left,
    Right,
}

impl BlockState for TrappedChest {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#type == Type::Right { return 11016; }
        if self.r#facing == Facing::East && self.r#type == Type::Single && self.r#waterlogged == true { return 11023; }
        if self.r#type == Type::Right && self.r#facing == Facing::East && self.r#waterlogged == false { return 11028; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#type == Type::Left { return 11026; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#type == Type::Right { return 11010; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#type == Type::Right { return 11022; }
        if self.r#type == Type::Single && self.r#facing == Facing::East && self.r#waterlogged == false { return 11024; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#type == Type::Left { return 11020; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#type == Type::Right { return 11021; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#type == Type::Left { return 11007; }
        if self.r#waterlogged == true && self.r#type == Type::Right && self.r#facing == Facing::South { return 11015; }
        if self.r#type == Type::Left && self.r#waterlogged == true && self.r#facing == Facing::South { return 11013; }
        if self.r#type == Type::Left && self.r#facing == Facing::South && self.r#waterlogged == false { return 11014; }
        if self.r#type == Type::Left && self.r#waterlogged == false && self.r#facing == Facing::North { return 11008; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#type == Type::Single { return 11005; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#type == Type::Left { return 11019; }
        if self.r#waterlogged == false && self.r#type == Type::Single && self.r#facing == Facing::South { return 11012; }
        if self.r#type == Type::Right && self.r#facing == Facing::North && self.r#waterlogged == true { return 11009; }
        if self.r#waterlogged == true && self.r#type == Type::Left && self.r#facing == Facing::East { return 11025; }
        if self.r#waterlogged == true && self.r#type == Type::Single && self.r#facing == Facing::South { return 11011; }
        if self.r#type == Type::Right && self.r#facing == Facing::East && self.r#waterlogged == true { return 11027; }
        if self.r#facing == Facing::West && self.r#type == Type::Single && self.r#waterlogged == false { return 11018; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#type == Type::Single { return 11017; }
        if self.r#type == Type::Single && self.r#facing == Facing::North && self.r#waterlogged == false { return 11006; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11016 {
            return Some(TrappedChest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 11023 {
            return Some(TrappedChest {
                r#facing: Facing::East,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 11028 {
            return Some(TrappedChest {
                r#type: Type::Right,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 11026 {
            return Some(TrappedChest {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#type: Type::Left,
            });
        }
        if state_id == 11010 {
            return Some(TrappedChest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 11022 {
            return Some(TrappedChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 11024 {
            return Some(TrappedChest {
                r#type: Type::Single,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 11020 {
            return Some(TrappedChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 11021 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#type: Type::Right,
            });
        }
        if state_id == 11007 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Left,
            });
        }
        if state_id == 11015 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::South,
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
                r#type: Type::Left,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 11008 {
            return Some(TrappedChest {
                r#type: Type::Left,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 11005 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Single,
            });
        }
        if state_id == 11019 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#type: Type::Left,
            });
        }
        if state_id == 11012 {
            return Some(TrappedChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 11009 {
            return Some(TrappedChest {
                r#type: Type::Right,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 11025 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 11011 {
            return Some(TrappedChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 11027 {
            return Some(TrappedChest {
                r#type: Type::Right,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 11018 {
            return Some(TrappedChest {
                r#facing: Facing::West,
                r#type: Type::Single,
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
        if state_id == 11006 {
            return Some(TrappedChest {
                r#type: Type::Single,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

