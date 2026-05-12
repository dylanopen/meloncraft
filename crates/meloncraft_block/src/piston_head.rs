use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PistonHead {
    pub r#type: Type,
    pub short: bool,
    pub r#facing: Facing,
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
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#type == Type::Sticky && block_state.r#short == true { return 2082; }
        if block_state.r#facing == Facing::North && block_state.r#type == Type::Normal && block_state.r#short == true { return 2069; }
        if block_state.r#type == Type::Sticky && block_state.r#short == false && block_state.r#facing == Facing::East { return 2076; }
        if block_state.r#short == false && block_state.r#type == Type::Normal && block_state.r#facing == Facing::East { return 2075; }
        if block_state.r#type == Type::Sticky && block_state.r#facing == Facing::East && block_state.r#short == true { return 2074; }
        if block_state.r#type == Type::Normal && block_state.r#short == false && block_state.r#facing == Facing::West { return 2083; }
        if block_state.r#facing == Facing::Up && block_state.r#short == true && block_state.r#type == Type::Sticky { return 2086; }
        if block_state.r#facing == Facing::North && block_state.r#short == false && block_state.r#type == Type::Sticky { return 2072; }
        if block_state.r#short == true && block_state.r#type == Type::Normal && block_state.r#facing == Facing::South { return 2077; }
        if block_state.r#short == false && block_state.r#type == Type::Normal && block_state.r#facing == Facing::North { return 2071; }
        if block_state.r#short == true && block_state.r#type == Type::Sticky && block_state.r#facing == Facing::South { return 2078; }
        if block_state.r#facing == Facing::West && block_state.r#short == false && block_state.r#type == Type::Sticky { return 2084; }
        if block_state.r#type == Type::Normal && block_state.r#facing == Facing::Up && block_state.r#short == true { return 2085; }
        if block_state.r#short == false && block_state.r#facing == Facing::Up && block_state.r#type == Type::Normal { return 2087; }
        if block_state.r#type == Type::Normal && block_state.r#facing == Facing::South && block_state.r#short == false { return 2079; }
        if block_state.r#type == Type::Sticky && block_state.r#facing == Facing::North && block_state.r#short == true { return 2070; }
        if block_state.r#facing == Facing::Down && block_state.r#short == true && block_state.r#type == Type::Normal { return 2089; }
        if block_state.r#type == Type::Sticky && block_state.r#short == true && block_state.r#facing == Facing::Down { return 2090; }
        if block_state.r#facing == Facing::Down && block_state.r#short == false && block_state.r#type == Type::Normal { return 2091; }
        if block_state.r#short == false && block_state.r#type == Type::Sticky && block_state.r#facing == Facing::Down { return 2092; }
        if block_state.r#facing == Facing::East && block_state.r#short == true && block_state.r#type == Type::Normal { return 2073; }
        if block_state.r#type == Type::Sticky && block_state.r#facing == Facing::Up && block_state.r#short == false { return 2088; }
        if block_state.r#type == Type::Sticky && block_state.r#short == false && block_state.r#facing == Facing::South { return 2080; }
        if block_state.r#short == true && block_state.r#type == Type::Normal && block_state.r#facing == Facing::West { return 2081; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2082 {
            return Some(PistonHead {
                r#facing: Facing::West,
                r#type: Type::Sticky,
                r#short: true,
            });
        }
        if state_id == 2069 {
            return Some(PistonHead {
                r#facing: Facing::North,
                r#type: Type::Normal,
                r#short: true,
            });
        }
        if state_id == 2076 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#short: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2075 {
            return Some(PistonHead {
                r#short: false,
                r#type: Type::Normal,
                r#facing: Facing::East,
            });
        }
        if state_id == 2074 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#facing: Facing::East,
                r#short: true,
            });
        }
        if state_id == 2083 {
            return Some(PistonHead {
                r#type: Type::Normal,
                r#short: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2086 {
            return Some(PistonHead {
                r#facing: Facing::Up,
                r#short: true,
                r#type: Type::Sticky,
            });
        }
        if state_id == 2072 {
            return Some(PistonHead {
                r#facing: Facing::North,
                r#short: false,
                r#type: Type::Sticky,
            });
        }
        if state_id == 2077 {
            return Some(PistonHead {
                r#short: true,
                r#type: Type::Normal,
                r#facing: Facing::South,
            });
        }
        if state_id == 2071 {
            return Some(PistonHead {
                r#short: false,
                r#type: Type::Normal,
                r#facing: Facing::North,
            });
        }
        if state_id == 2078 {
            return Some(PistonHead {
                r#short: true,
                r#type: Type::Sticky,
                r#facing: Facing::South,
            });
        }
        if state_id == 2084 {
            return Some(PistonHead {
                r#facing: Facing::West,
                r#short: false,
                r#type: Type::Sticky,
            });
        }
        if state_id == 2085 {
            return Some(PistonHead {
                r#type: Type::Normal,
                r#facing: Facing::Up,
                r#short: true,
            });
        }
        if state_id == 2087 {
            return Some(PistonHead {
                r#short: false,
                r#facing: Facing::Up,
                r#type: Type::Normal,
            });
        }
        if state_id == 2079 {
            return Some(PistonHead {
                r#type: Type::Normal,
                r#facing: Facing::South,
                r#short: false,
            });
        }
        if state_id == 2070 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#facing: Facing::North,
                r#short: true,
            });
        }
        if state_id == 2089 {
            return Some(PistonHead {
                r#facing: Facing::Down,
                r#short: true,
                r#type: Type::Normal,
            });
        }
        if state_id == 2090 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#short: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 2091 {
            return Some(PistonHead {
                r#facing: Facing::Down,
                r#short: false,
                r#type: Type::Normal,
            });
        }
        if state_id == 2092 {
            return Some(PistonHead {
                r#short: false,
                r#type: Type::Sticky,
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
        if state_id == 2088 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#facing: Facing::Up,
                r#short: false,
            });
        }
        if state_id == 2080 {
            return Some(PistonHead {
                r#type: Type::Sticky,
                r#short: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2081 {
            return Some(PistonHead {
                r#short: true,
                r#type: Type::Normal,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

