use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperChest {
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

impl BlockState for ExposedCopperChest {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#type == Type::Single
        {
            return 26924;
        }
        if self.r#facing == Facing::West
            && self.r#type == Type::Right
            && self.r#waterlogged == false
        {
            return 26934;
        }
        if self.r#type == Type::Single
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 26929;
        }
        if self.r#type == Type::Left && self.r#waterlogged == true && self.r#facing == Facing::East
        {
            return 26937;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#type == Type::Right
        {
            return 26940;
        }
        if self.r#waterlogged == true
            && self.r#type == Type::Right
            && self.r#facing == Facing::North
        {
            return 26921;
        }
        if self.r#type == Type::Right
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 26928;
        }
        if self.r#facing == Facing::North && self.r#type == Type::Left && self.r#waterlogged == true
        {
            return 26919;
        }
        if self.r#type == Type::Right
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 26927;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#type == Type::Single
        {
            return 26930;
        }
        if self.r#facing == Facing::North
            && self.r#type == Type::Single
            && self.r#waterlogged == true
        {
            return 26917;
        }
        if self.r#waterlogged == false
            && self.r#type == Type::Left
            && self.r#facing == Facing::South
        {
            return 26926;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#type == Type::Right
        {
            return 26933;
        }
        if self.r#type == Type::Left
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 26920;
        }
        if self.r#waterlogged == false
            && self.r#type == Type::Single
            && self.r#facing == Facing::North
        {
            return 26918;
        }
        if self.r#waterlogged == false && self.r#type == Type::Left && self.r#facing == Facing::East
        {
            return 26938;
        }
        if self.r#waterlogged == false && self.r#type == Type::Left && self.r#facing == Facing::West
        {
            return 26932;
        }
        if self.r#waterlogged == false
            && self.r#type == Type::Right
            && self.r#facing == Facing::North
        {
            return 26922;
        }
        if self.r#type == Type::Left && self.r#waterlogged == true && self.r#facing == Facing::West
        {
            return 26931;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::South && self.r#waterlogged == true
        {
            return 26925;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#type == Type::Single
        {
            return 26923;
        }
        if self.r#type == Type::Single
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 26935;
        }
        if self.r#facing == Facing::East
            && self.r#type == Type::Single
            && self.r#waterlogged == false
        {
            return 26936;
        }
        if self.r#facing == Facing::East && self.r#type == Type::Right && self.r#waterlogged == true
        {
            return 26939;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26924 {
            return Some(ExposedCopperChest {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#type: Type::Single,
            });
        }
        if state_id == 26934 {
            return Some(ExposedCopperChest {
                r#facing: Facing::West,
                r#type: Type::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 26929 {
            return Some(ExposedCopperChest {
                r#type: Type::Single,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 26937 {
            return Some(ExposedCopperChest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26940 {
            return Some(ExposedCopperChest {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 26921 {
            return Some(ExposedCopperChest {
                r#waterlogged: true,
                r#type: Type::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 26928 {
            return Some(ExposedCopperChest {
                r#type: Type::Right,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26919 {
            return Some(ExposedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 26927 {
            return Some(ExposedCopperChest {
                r#type: Type::Right,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26930 {
            return Some(ExposedCopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 26917 {
            return Some(ExposedCopperChest {
                r#facing: Facing::North,
                r#type: Type::Single,
                r#waterlogged: true,
            });
        }
        if state_id == 26926 {
            return Some(ExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 26933 {
            return Some(ExposedCopperChest {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#type: Type::Right,
            });
        }
        if state_id == 26920 {
            return Some(ExposedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26918 {
            return Some(ExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::North,
            });
        }
        if state_id == 26938 {
            return Some(ExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 26932 {
            return Some(ExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 26922 {
            return Some(ExposedCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 26931 {
            return Some(ExposedCopperChest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26925 {
            return Some(ExposedCopperChest {
                r#type: Type::Left,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26923 {
            return Some(ExposedCopperChest {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#type: Type::Single,
            });
        }
        if state_id == 26935 {
            return Some(ExposedCopperChest {
                r#type: Type::Single,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26936 {
            return Some(ExposedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Single,
                r#waterlogged: false,
            });
        }
        if state_id == 26939 {
            return Some(ExposedCopperChest {
                r#facing: Facing::East,
                r#type: Type::Right,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
