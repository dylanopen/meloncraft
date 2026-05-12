use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperChest {
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

impl BlockState for ExposedCopperChest {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 26940; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#type == Type::Single { return 26930; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 26920; }
        if block_state.r#facing == Facing::West && block_state.r#type == Type::Single && block_state.r#waterlogged == true { return 26929; }
        if block_state.r#facing == Facing::East && block_state.r#type == Type::Single && block_state.r#waterlogged == false { return 26936; }
        if block_state.r#facing == Facing::West && block_state.r#type == Type::Right && block_state.r#waterlogged == false { return 26934; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#type == Type::Left { return 26926; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 26925; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 26932; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#type == Type::Right { return 26927; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 26933; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Right && block_state.r#waterlogged == false { return 26922; }
        if block_state.r#facing == Facing::East && block_state.r#type == Type::Single && block_state.r#waterlogged == true { return 26935; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::South { return 26923; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Left && block_state.r#waterlogged == true { return 26919; }
        if block_state.r#type == Type::Single && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 26924; }
        if block_state.r#facing == Facing::East && block_state.r#type == Type::Left && block_state.r#waterlogged == false { return 26938; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 26917; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Right && block_state.r#facing == Facing::South { return 26928; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 26931; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 26918; }
        if block_state.r#facing == Facing::East && block_state.r#type == Type::Left && block_state.r#waterlogged == true { return 26937; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Right && block_state.r#waterlogged == true { return 26921; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Right && block_state.r#facing == Facing::East { return 26939; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26940 {
            return Some(ExposedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26930 {
            return Some(ExposedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#type: Type::Single,
            });
        }
        if state_id == 26920 {
            return Some(ExposedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26929 {
            return Some(ExposedCopperChest {
                r#facing: Facing::West,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 26936 {
            return Some(ExposedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Single,
                r#waterlogged: false,
            });
        }
        if state_id == 26934 {
            return Some(ExposedCopperChest {
                r#facing: Facing::West,
                r#type: Type::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 26926 {
            return Some(ExposedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#type: Type::Left,
            });
        }
        if state_id == 26925 {
            return Some(ExposedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26932 {
            return Some(ExposedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 26927 {
            return Some(ExposedCopperChest {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        if state_id == 26933 {
            return Some(ExposedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 26922 {
            return Some(ExposedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 26935 {
            return Some(ExposedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 26923 {
            return Some(ExposedCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 26919 {
            return Some(ExposedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 26924 {
            return Some(ExposedCopperChest {
                r#type: Type::Single,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26938 {
            return Some(ExposedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 26917 {
            return Some(ExposedCopperChest {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 26928 {
            return Some(ExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 26931 {
            return Some(ExposedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 26918 {
            return Some(ExposedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26937 {
            return Some(ExposedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 26921 {
            return Some(ExposedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 26939 {
            return Some(ExposedCopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

