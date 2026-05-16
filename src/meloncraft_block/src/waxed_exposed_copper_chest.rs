use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperChest {
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

impl BlockState for WaxedExposedCopperChest {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#type == Type::Single && self.r#waterlogged == true { return 27013; }
        if self.r#waterlogged == false && self.r#type == Type::Left && self.r#facing == Facing::East { return 27034; }
        if self.r#facing == Facing::North && self.r#type == Type::Left && self.r#waterlogged == false { return 27016; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#type == Type::Right { return 27023; }
        if self.r#facing == Facing::South && self.r#type == Type::Single && self.r#waterlogged == true { return 27019; }
        if self.r#type == Type::Single && self.r#facing == Facing::West && self.r#waterlogged == true { return 27025; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#type == Type::Left { return 27022; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#type == Type::Right { return 27018; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#type == Type::Left { return 27028; }
        if self.r#type == Type::Single && self.r#facing == Facing::West && self.r#waterlogged == false { return 27026; }
        if self.r#type == Type::Right && self.r#waterlogged == true && self.r#facing == Facing::West { return 27029; }
        if self.r#facing == Facing::South && self.r#type == Type::Single && self.r#waterlogged == false { return 27020; }
        if self.r#type == Type::Right && self.r#facing == Facing::West && self.r#waterlogged == false { return 27030; }
        if self.r#type == Type::Right && self.r#facing == Facing::North && self.r#waterlogged == true { return 27017; }
        if self.r#type == Type::Single && self.r#facing == Facing::East && self.r#waterlogged == true { return 27031; }
        if self.r#type == Type::Single && self.r#facing == Facing::North && self.r#waterlogged == false { return 27014; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#type == Type::Left { return 27027; }
        if self.r#facing == Facing::East && self.r#type == Type::Single && self.r#waterlogged == false { return 27032; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#type == Type::Left { return 27021; }
        if self.r#waterlogged == true && self.r#type == Type::Right && self.r#facing == Facing::East { return 27035; }
        if self.r#type == Type::Right && self.r#facing == Facing::South && self.r#waterlogged == false { return 27024; }
        if self.r#type == Type::Left && self.r#facing == Facing::East && self.r#waterlogged == true { return 27033; }
        if self.r#type == Type::Right && self.r#waterlogged == false && self.r#facing == Facing::East { return 27036; }
        if self.r#waterlogged == true && self.r#type == Type::Left && self.r#facing == Facing::North { return 27015; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27013 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 27034 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 27016 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 27023 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#type: Type::Right,
            });
        }
        if state_id == 27019 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::South,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 27025 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27022 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 27018 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 27028 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#type: Type::Left,
            });
        }
        if state_id == 27026 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27029 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Right,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27020 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::South,
                r#type: Type::Single,
                r#waterlogged: false,
            });
        }
        if state_id == 27030 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27017 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27031 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27014 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Single,
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
        if state_id == 27032 {
            return Some(WaxedExposedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Single,
                r#waterlogged: false,
            });
        }
        if state_id == 27021 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#type: Type::Left,
            });
        }
        if state_id == 27035 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 27024 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27033 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27036 {
            return Some(WaxedExposedCopperChest {
                r#type: Type::Right,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27015 {
            return Some(WaxedExposedCopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

