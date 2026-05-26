use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperChest {
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

impl BlockState for WaxedOxidizedCopperChest {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Right
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 27078;
        }
        if self.r#type == Type::Single
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 27074;
        }
        if self.r#type == Type::Left && self.r#waterlogged == true && self.r#facing == Facing::North
        {
            return 27063;
        }
        if self.r#type == Type::Single
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 27062;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#type == Type::Right
        {
            return 27071;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::West && self.r#waterlogged == true
        {
            return 27075;
        }
        if self.r#type == Type::Left
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 27064;
        }
        if self.r#waterlogged == false
            && self.r#type == Type::Single
            && self.r#facing == Facing::South
        {
            return 27068;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#type == Type::Left
        {
            return 27076;
        }
        if self.r#type == Type::Right && self.r#facing == Facing::West && self.r#waterlogged == true
        {
            return 27077;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#type == Type::Single
        {
            return 27079;
        }
        if self.r#waterlogged == false
            && self.r#type == Type::Single
            && self.r#facing == Facing::East
        {
            return 27080;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#type == Type::Right
        {
            return 27083;
        }
        if self.r#type == Type::Single
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 27067;
        }
        if self.r#type == Type::Right
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 27066;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#type == Type::Left
        {
            return 27081;
        }
        if self.r#waterlogged == true && self.r#type == Type::Left && self.r#facing == Facing::South
        {
            return 27069;
        }
        if self.r#waterlogged == false && self.r#type == Type::Left && self.r#facing == Facing::East
        {
            return 27082;
        }
        if self.r#type == Type::Right
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 27084;
        }
        if self.r#waterlogged == true
            && self.r#type == Type::Right
            && self.r#facing == Facing::North
        {
            return 27065;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#type == Type::Left
        {
            return 27070;
        }
        if self.r#type == Type::Single
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 27061;
        }
        if self.r#type == Type::Right
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 27072;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#type == Type::Single
        {
            return 27073;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27078 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27074 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27063 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27062 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27071 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        if state_id == 27075 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27064 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27068 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 27076 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#type: Type::Left,
            });
        }
        if state_id == 27077 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27079 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 27080 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::East,
            });
        }
        if state_id == 27083 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        if state_id == 27067 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27066 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Right,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27081 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#type: Type::Left,
            });
        }
        if state_id == 27069 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 27082 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 27084 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27065 {
            return Some(WaxedOxidizedCopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 27070 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 27061 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27072 {
            return Some(WaxedOxidizedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27073 {
            return Some(WaxedOxidizedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        return None;
    }
}
