use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperChest {
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

impl BlockState for WaxedWeatheredCopperChest {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#type == Type::Left { return 27051; }
        if self.r#type == Type::Right && self.r#waterlogged == false && self.r#facing == Facing::North { return 27042; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#type == Type::Right { return 27048; }
        if self.r#type == Type::Single && self.r#waterlogged == true && self.r#facing == Facing::West { return 27049; }
        if self.r#waterlogged == false && self.r#type == Type::Left && self.r#facing == Facing::North { return 27040; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#type == Type::Right { return 27054; }
        if self.r#type == Type::Single && self.r#facing == Facing::East && self.r#waterlogged == true { return 27055; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#type == Type::Left { return 27052; }
        if self.r#waterlogged == true && self.r#type == Type::Left && self.r#facing == Facing::East { return 27057; }
        if self.r#waterlogged == true && self.r#type == Type::Left && self.r#facing == Facing::North { return 27039; }
        if self.r#waterlogged == true && self.r#type == Type::Single && self.r#facing == Facing::South { return 27043; }
        if self.r#type == Type::Single && self.r#facing == Facing::East && self.r#waterlogged == false { return 27056; }
        if self.r#waterlogged == false && self.r#type == Type::Left && self.r#facing == Facing::East { return 27058; }
        if self.r#type == Type::Right && self.r#facing == Facing::West && self.r#waterlogged == true { return 27053; }
        if self.r#facing == Facing::South && self.r#type == Type::Left && self.r#waterlogged == true { return 27045; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#type == Type::Single { return 27044; }
        if self.r#type == Type::Single && self.r#facing == Facing::North && self.r#waterlogged == true { return 27037; }
        if self.r#waterlogged == false && self.r#type == Type::Right && self.r#facing == Facing::East { return 27060; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#type == Type::Right { return 27041; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#type == Type::Single { return 27050; }
        if self.r#type == Type::Right && self.r#waterlogged == true && self.r#facing == Facing::East { return 27059; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#type == Type::Right { return 27047; }
        if self.r#type == Type::Left && self.r#waterlogged == false && self.r#facing == Facing::South { return 27046; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#type == Type::Single { return 27038; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27051 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 27042 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Right,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27048 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 27049 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Single,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27040 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 27054 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 27055 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Single,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27052 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 27057 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 27039 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 27043 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 27056 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Single,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27058 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 27053 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27045 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::South,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 27044 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#type: Type::Single,
            });
        }
        if state_id == 27037 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Single,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27060 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 27041 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Right,
            });
        }
        if state_id == 27050 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::West,
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
        if state_id == 27047 {
            return Some(WaxedWeatheredCopperChest {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#type: Type::Right,
            });
        }
        if state_id == 27046 {
            return Some(WaxedWeatheredCopperChest {
                r#type: Type::Left,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27038 {
            return Some(WaxedWeatheredCopperChest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        return None;
    }
}

