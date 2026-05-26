use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PistonHead {
    pub r#type: Type,
    pub r#facing: Facing,
    pub short: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Normal,
    Sticky,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for PistonHead {
    fn to_id(&self) -> i32 {
        if self.r#short == false && self.r#type == Type::Normal && self.r#facing == Facing::Down {
            return 2091;
        }
        if self.r#short == true && self.r#type == Type::Sticky && self.r#facing == Facing::Down {
            return 2090;
        }
        if self.r#type == Type::Sticky && self.r#short == false && self.r#facing == Facing::East {
            return 2076;
        }
        if self.r#type == Type::Sticky && self.r#facing == Facing::West && self.r#short == true {
            return 2082;
        }
        if self.r#facing == Facing::West && self.r#type == Type::Sticky && self.r#short == false {
            return 2084;
        }
        if self.r#type == Type::Normal && self.r#facing == Facing::Up && self.r#short == true {
            return 2085;
        }
        if self.r#short == false && self.r#type == Type::Sticky && self.r#facing == Facing::Up {
            return 2088;
        }
        if self.r#facing == Facing::North && self.r#type == Type::Sticky && self.r#short == false {
            return 2072;
        }
        if self.r#facing == Facing::South && self.r#short == true && self.r#type == Type::Normal {
            return 2077;
        }
        if self.r#facing == Facing::South && self.r#type == Type::Normal && self.r#short == false {
            return 2079;
        }
        if self.r#facing == Facing::Up && self.r#short == false && self.r#type == Type::Normal {
            return 2087;
        }
        if self.r#short == true && self.r#type == Type::Sticky && self.r#facing == Facing::Up {
            return 2086;
        }
        if self.r#type == Type::Normal && self.r#facing == Facing::North && self.r#short == false {
            return 2071;
        }
        if self.r#short == false && self.r#facing == Facing::East && self.r#type == Type::Normal {
            return 2075;
        }
        if self.r#facing == Facing::North && self.r#short == true && self.r#type == Type::Sticky {
            return 2070;
        }
        if self.r#short == true && self.r#facing == Facing::West && self.r#type == Type::Normal {
            return 2081;
        }
        if self.r#short == true && self.r#type == Type::Normal && self.r#facing == Facing::Down {
            return 2089;
        }
        if self.r#facing == Facing::East && self.r#short == true && self.r#type == Type::Normal {
            return 2073;
        }
        if self.r#type == Type::Sticky && self.r#facing == Facing::Down && self.r#short == false {
            return 2092;
        }
        if self.r#facing == Facing::North && self.r#type == Type::Normal && self.r#short == true {
            return 2069;
        }
        if self.r#short == false && self.r#type == Type::Sticky && self.r#facing == Facing::South {
            return 2080;
        }
        if self.r#facing == Facing::East && self.r#type == Type::Sticky && self.r#short == true {
            return 2074;
        }
        if self.r#type == Type::Normal && self.r#facing == Facing::West && self.r#short == false {
            return 2083;
        }
        if self.r#type == Type::Sticky && self.r#short == true && self.r#facing == Facing::South {
            return 2078;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2091 {
            return Some(PistonHead {
                r#short: false,
                r#type: Type::Normal,
                r#facing: Facing::Down,
            });
        }
        if state_id == 2090 {
            return Some(PistonHead {
                r#short: true,
                r#type: Type::Sticky,
                r#facing: Facing::Down,
            });
        }
        if state_id == 2076 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#short: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2082 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#facing: Facing::West,
                r#short: true,
            });
        }
        if state_id == 2084 {
            return Some(PistonHead {
                r#facing: Facing::West,
                r#type: Type::Sticky,
                r#short: false,
            });
        }
        if state_id == 2085 {
            return Some(PistonHead {
                r#type: Type::Normal,
                r#facing: Facing::Up,
                r#short: true,
            });
        }
        if state_id == 2088 {
            return Some(PistonHead {
                r#short: false,
                r#type: Type::Sticky,
                r#facing: Facing::Up,
            });
        }
        if state_id == 2072 {
            return Some(PistonHead {
                r#facing: Facing::North,
                r#type: Type::Sticky,
                r#short: false,
            });
        }
        if state_id == 2077 {
            return Some(PistonHead {
                r#facing: Facing::South,
                r#short: true,
                r#type: Type::Normal,
            });
        }
        if state_id == 2079 {
            return Some(PistonHead {
                r#facing: Facing::South,
                r#type: Type::Normal,
                r#short: false,
            });
        }
        if state_id == 2087 {
            return Some(PistonHead {
                r#facing: Facing::Up,
                r#short: false,
                r#type: Type::Normal,
            });
        }
        if state_id == 2086 {
            return Some(PistonHead {
                r#short: true,
                r#type: Type::Sticky,
                r#facing: Facing::Up,
            });
        }
        if state_id == 2071 {
            return Some(PistonHead {
                r#type: Type::Normal,
                r#facing: Facing::North,
                r#short: false,
            });
        }
        if state_id == 2075 {
            return Some(PistonHead {
                r#short: false,
                r#facing: Facing::East,
                r#type: Type::Normal,
            });
        }
        if state_id == 2070 {
            return Some(PistonHead {
                r#facing: Facing::North,
                r#short: true,
                r#type: Type::Sticky,
            });
        }
        if state_id == 2081 {
            return Some(PistonHead {
                r#short: true,
                r#facing: Facing::West,
                r#type: Type::Normal,
            });
        }
        if state_id == 2089 {
            return Some(PistonHead {
                r#short: true,
                r#type: Type::Normal,
                r#facing: Facing::Down,
            });
        }
        if state_id == 2073 {
            return Some(PistonHead {
                r#facing: Facing::East,
                r#short: true,
                r#type: Type::Normal,
            });
        }
        if state_id == 2092 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#facing: Facing::Down,
                r#short: false,
            });
        }
        if state_id == 2069 {
            return Some(PistonHead {
                r#facing: Facing::North,
                r#type: Type::Normal,
                r#short: true,
            });
        }
        if state_id == 2080 {
            return Some(PistonHead {
                r#short: false,
                r#type: Type::Sticky,
                r#facing: Facing::South,
            });
        }
        if state_id == 2074 {
            return Some(PistonHead {
                r#facing: Facing::East,
                r#type: Type::Sticky,
                r#short: true,
            });
        }
        if state_id == 2083 {
            return Some(PistonHead {
                r#type: Type::Normal,
                r#facing: Facing::West,
                r#short: false,
            });
        }
        if state_id == 2078 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#short: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
