use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperChest {
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

impl BlockState for CopperChest {
    fn to_id(&self) -> i32 {
        if self.r#type == Type::Right
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 26916;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#type == Type::Left
        {
            return 26896;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#type == Type::Left
        {
            return 26901;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::West && self.r#waterlogged == false
        {
            return 26908;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::East && self.r#waterlogged == true
        {
            return 26913;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#type == Type::Right
        {
            return 26915;
        }
        if self.r#type == Type::Single
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 26912;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::North && self.r#waterlogged == true
        {
            return 26895;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#type == Type::Single
        {
            return 26900;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#type == Type::Right
        {
            return 26904;
        }
        if self.r#type == Type::Right
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 26903;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#type == Type::Single
        {
            return 26906;
        }
        if self.r#waterlogged == true
            && self.r#type == Type::Single
            && self.r#facing == Facing::East
        {
            return 26911;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#type == Type::Right
        {
            return 26897;
        }
        if self.r#type == Type::Single
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 26899;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#type == Type::Right
        {
            return 26898;
        }
        if self.r#waterlogged == true && self.r#type == Type::Left && self.r#facing == Facing::West
        {
            return 26907;
        }
        if self.r#type == Type::Left && self.r#facing == Facing::East && self.r#waterlogged == false
        {
            return 26914;
        }
        if self.r#type == Type::Single
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 26905;
        }
        if self.r#waterlogged == true
            && self.r#type == Type::Single
            && self.r#facing == Facing::North
        {
            return 26893;
        }
        if self.r#type == Type::Right && self.r#facing == Facing::West && self.r#waterlogged == true
        {
            return 26909;
        }
        if self.r#waterlogged == false
            && self.r#type == Type::Right
            && self.r#facing == Facing::West
        {
            return 26910;
        }
        if self.r#type == Type::Single
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 26894;
        }
        if self.r#facing == Facing::South
            && self.r#type == Type::Left
            && self.r#waterlogged == false
        {
            return 26902;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26916 {
            return Some(CopperChest {
                r#type: Type::Right,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26896 {
            return Some(CopperChest {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#type: Type::Left,
            });
        }
        if state_id == 26901 {
            return Some(CopperChest {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#type: Type::Left,
            });
        }
        if state_id == 26908 {
            return Some(CopperChest {
                r#type: Type::Left,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 26913 {
            return Some(CopperChest {
                r#type: Type::Left,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 26915 {
            return Some(CopperChest {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        if state_id == 26912 {
            return Some(CopperChest {
                r#type: Type::Single,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26895 {
            return Some(CopperChest {
                r#type: Type::Left,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 26900 {
            return Some(CopperChest {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#type: Type::Single,
            });
        }
        if state_id == 26904 {
            return Some(CopperChest {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#type: Type::Right,
            });
        }
        if state_id == 26903 {
            return Some(CopperChest {
                r#type: Type::Right,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26906 {
            return Some(CopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Single,
            });
        }
        if state_id == 26911 {
            return Some(CopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::East,
            });
        }
        if state_id == 26897 {
            return Some(CopperChest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Right,
            });
        }
        if state_id == 26899 {
            return Some(CopperChest {
                r#type: Type::Single,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26898 {
            return Some(CopperChest {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 26907 {
            return Some(CopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 26914 {
            return Some(CopperChest {
                r#type: Type::Left,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26905 {
            return Some(CopperChest {
                r#type: Type::Single,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 26893 {
            return Some(CopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::North,
            });
        }
        if state_id == 26909 {
            return Some(CopperChest {
                r#type: Type::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 26910 {
            return Some(CopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 26894 {
            return Some(CopperChest {
                r#type: Type::Single,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26902 {
            return Some(CopperChest {
                r#facing: Facing::South,
                r#type: Type::Left,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
