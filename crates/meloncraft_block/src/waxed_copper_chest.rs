use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperChest {
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for WaxedCopperChest {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Right && block_state.r#facing == Facing::North { return 26993; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#type == Type::Single { return 26990; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27003; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 26997; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Single && block_state.r#facing == Facing::East { return 27008; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#type == Type::Left { return 26998; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Left && block_state.r#waterlogged == true { return 26991; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 26992; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::South { return 26995; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Right && block_state.r#facing == Facing::South { return 26999; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#type == Type::Single { return 27002; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Right && block_state.r#facing == Facing::South { return 27000; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#type == Type::Single { return 26996; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 27001; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#type == Type::Left { return 27004; }
        if block_state.r#type == Type::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27005; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 26994; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 27007; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#type == Type::Right { return 27012; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::North { return 26989; }
        if block_state.r#facing == Facing::East && block_state.r#type == Type::Left && block_state.r#waterlogged == true { return 27009; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Left && block_state.r#facing == Facing::East { return 27010; }
        if block_state.r#facing == Facing::West && block_state.r#type == Type::Right && block_state.r#waterlogged == false { return 27006; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Right && block_state.r#facing == Facing::East { return 27011; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26993 {
            return Some(WaxedCopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 26990 {
            return Some(WaxedCopperChest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 27003 {
            return Some(WaxedCopperChest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26997 {
            return Some(WaxedCopperChest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27008 {
            return Some(WaxedCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::East,
            });
        }
        if state_id == 26998 {
            return Some(WaxedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#type: Type::Left,
            });
        }
        if state_id == 26991 {
            return Some(WaxedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 26992 {
            return Some(WaxedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26995 {
            return Some(WaxedCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 26999 {
            return Some(WaxedCopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 27002 {
            return Some(WaxedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 27000 {
            return Some(WaxedCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 26996 {
            return Some(WaxedCopperChest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 27001 {
            return Some(WaxedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 27004 {
            return Some(WaxedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 27005 {
            return Some(WaxedCopperChest {
                r#type: Type::Right,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26994 {
            return Some(WaxedCopperChest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 27007 {
            return Some(WaxedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 27012 {
            return Some(WaxedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#type: Type::Right,
            });
        }
        if state_id == 26989 {
            return Some(WaxedCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::North,
            });
        }
        if state_id == 27009 {
            return Some(WaxedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 27010 {
            return Some(WaxedCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 27006 {
            return Some(WaxedCopperChest {
                r#facing: Facing::West,
                r#type: Type::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 27011 {
            return Some(WaxedCopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

