use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperChest {
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

impl BlockState for WaxedExposedCopperChest {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27018; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 27027; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Left && block_state.r#facing == Facing::North { return 27016; }
        if block_state.r#type == Type::Single && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27019; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#type == Type::Left { return 27028; }
        if block_state.r#type == Type::Single && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27031; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 27013; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 27015; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#type == Type::Single { return 27026; }
        if block_state.r#type == Type::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27035; }
        if block_state.r#facing == Facing::East && block_state.r#type == Type::Left && block_state.r#waterlogged == true { return 27033; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Left && block_state.r#facing == Facing::South { return 27021; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27034; }
        if block_state.r#facing == Facing::East && block_state.r#type == Type::Right && block_state.r#waterlogged == false { return 27036; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Right && block_state.r#facing == Facing::West { return 27030; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Single && block_state.r#facing == Facing::South { return 27020; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27022; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Right { return 27029; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#type == Type::Single { return 27032; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27024; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Single && block_state.r#facing == Facing::North { return 27014; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Single { return 27025; }
        if block_state.r#type == Type::Right && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27017; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#type == Type::Right { return 27023; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27018 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27027 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 27016 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 27019 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Single,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27028 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#type: Type::Left,
            });
        }
        if state_id == 27031 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Single,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27013 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 27015 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 27026 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#type: Type::Single,
            });
        }
        if state_id == 27035 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27033 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 27021 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 27034 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27036 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 27030 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 27020 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 27022 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27029 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        if state_id == 27032 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#type: Type::Single,
            });
        }
        if state_id == 27024 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27014 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::North,
            });
        }
        if state_id == 27025 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 27017 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27023 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        return None;
    }
}

