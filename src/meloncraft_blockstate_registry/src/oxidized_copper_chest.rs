use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperChest {
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

impl BlockState for OxidizedCopperChest {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false
            && self.r#type == Type::Right
            && self.r#facing == Facing::North
        {
            return 26970;
        }
        if self.r#waterlogged == false
            && self.r#type == Type::Single
            && self.r#facing == Facing::North
        {
            return 26966;
        }
        if self.r#type == Type::Right && self.r#facing == Facing::East && self.r#waterlogged == true
        {
            return 26987;
        }
        if self.r#type == Type::Single
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 26971;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::North && self.r#waterlogged == true
        {
            return 26967;
        }
        if self.r#facing == Facing::North
            && self.r#type == Type::Left
            && self.r#waterlogged == false
        {
            return 26968;
        }
        if self.r#facing == Facing::North
            && self.r#type == Type::Right
            && self.r#waterlogged == true
        {
            return 26969;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#type == Type::Right
        {
            return 26975;
        }
        if self.r#facing == Facing::West
            && self.r#type == Type::Right
            && self.r#waterlogged == false
        {
            return 26982;
        }
        if self.r#facing == Facing::East
            && self.r#type == Type::Single
            && self.r#waterlogged == true
        {
            return 26983;
        }
        if self.r#waterlogged == false
            && self.r#type == Type::Single
            && self.r#facing == Facing::East
        {
            return 26984;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::East && self.r#waterlogged == true
        {
            return 26985;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::East && self.r#waterlogged == false
        {
            return 26986;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#type == Type::Left
        {
            return 26980;
        }
        if self.r#facing == Facing::West
            && self.r#type == Type::Single
            && self.r#waterlogged == true
        {
            return 26977;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#type == Type::Right
        {
            return 26988;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#type == Type::Left
        {
            return 26973;
        }
        if self.r#type == Type::Left
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 26974;
        }
        if self.r#type == Type::Right
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 26976;
        }
        if self.r#waterlogged == true && self.r#type == Type::Right && self.r#facing == Facing::West
        {
            return 26981;
        }
        if self.r#waterlogged == false
            && self.r#type == Type::Single
            && self.r#facing == Facing::West
        {
            return 26978;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#type == Type::Single
        {
            return 26972;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::West && self.r#waterlogged == true
        {
            return 26979;
        }
        if self.r#facing == Facing::North
            && self.r#type == Type::Single
            && self.r#waterlogged == true
        {
            return 26965;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26970 {
            return Some(OxidizedCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 26966 {
            return Some(OxidizedCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::North,
            });
        }
        if state_id == 26987 {
            return Some(OxidizedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 26971 {
            return Some(OxidizedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26967 {
            return Some(OxidizedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 26968 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 26969 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 26975 {
            return Some(OxidizedCopperChest {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#type: Type::Right,
            });
        }
        if state_id == 26982 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::West,
                r#type: Type::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 26983 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 26984 {
            return Some(OxidizedCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::East,
            });
        }
        if state_id == 26985 {
            return Some(OxidizedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 26986 {
            return Some(OxidizedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26980 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 26977 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::West,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 26988 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 26973 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 26974 {
            return Some(OxidizedCopperChest {
                r#type: Type::Left,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26976 {
            return Some(OxidizedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26981 {
            return Some(OxidizedCopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 26978 {
            return Some(OxidizedCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::West,
            });
        }
        if state_id == 26972 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 26979 {
            return Some(OxidizedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 26965 {
            return Some(OxidizedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
