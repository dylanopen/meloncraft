use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperChest {
    pub r#facing: Facing,
    pub r#type: Type,
    pub waterlogged: bool,
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

impl BlockState for WaxedOxidizedCopperChest {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::West { return 27073; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Left && block_state.r#facing == Facing::West { return 27075; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27062; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#type == Type::Single { return 27080; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 27082; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Right && block_state.r#facing == Facing::North { return 27066; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Right && block_state.r#facing == Facing::West { return 27077; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#type == Type::Right { return 27078; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 27081; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#type == Type::Right { return 27083; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#type == Type::Left { return 27064; }
        if block_state.r#type == Type::Single && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27067; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 27063; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27068; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27069; }
        if block_state.r#facing == Facing::South && block_state.r#type == Type::Left && block_state.r#waterlogged == false { return 27070; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::North { return 27061; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#type == Type::Right { return 27072; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#type == Type::Left { return 27076; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 27079; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 27084; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27071; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 27074; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#type == Type::Right { return 27065; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27073 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::West,
            });
        }
        if state_id == 27075 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 27062 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27080 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 27082 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Left,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27066 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 27077 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 27078 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#type: Type::Right,
            });
        }
        if state_id == 27081 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 27083 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        if state_id == 27064 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#type: Type::Left,
            });
        }
        if state_id == 27067 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Single,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27063 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 27068 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27069 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27070 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::South,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 27061 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::North,
            });
        }
        if state_id == 27072 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#type: Type::Right,
            });
        }
        if state_id == 27076 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 27079 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 27084 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 27071 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27074 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27065 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Right,
            });
        }
        return None;
    }
}

