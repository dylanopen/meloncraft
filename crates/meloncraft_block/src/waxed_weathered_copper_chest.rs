use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperChest {
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

impl BlockState for WaxedWeatheredCopperChest {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 27045; }
        if block_state.r#facing == Facing::South && block_state.r#type == Type::Right && block_state.r#waterlogged == false { return 27048; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Right && block_state.r#facing == Facing::North { return 27042; }
        if block_state.r#facing == Facing::South && block_state.r#type == Type::Right && block_state.r#waterlogged == true { return 27047; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#type == Type::Left { return 27051; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#type == Type::Left { return 27052; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#type == Type::Single { return 27056; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27057; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 27058; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::East { return 27055; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Left && block_state.r#facing == Facing::South { return 27046; }
        if block_state.r#type == Type::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 27041; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#type == Type::Left { return 27039; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 27049; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Right { return 27053; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 27060; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27040; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Single && block_state.r#facing == Facing::North { return 27038; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#type == Type::Single { return 27037; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::South { return 27043; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Right && block_state.r#facing == Facing::West { return 27054; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#type == Type::Single { return 27044; }
        if block_state.r#type == Type::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27059; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#type == Type::Single { return 27050; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27045 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 27048 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::South,
                r#type: Type::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 27042 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 27047 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::South,
                r#type: Type::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 27051 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#type: Type::Left,
            });
        }
        if state_id == 27052 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 27056 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 27057 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27058 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Left,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27055 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::East,
            });
        }
        if state_id == 27046 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 27041 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Right,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27039 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Left,
            });
        }
        if state_id == 27049 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 27053 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        if state_id == 27060 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 27040 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27038 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::North,
            });
        }
        if state_id == 27037 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Single,
            });
        }
        if state_id == 27043 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 27054 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 27044 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 27059 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27050 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#type: Type::Single,
            });
        }
        return None;
    }
}

