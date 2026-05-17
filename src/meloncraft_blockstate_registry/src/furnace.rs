use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Furnace {
    pub r#facing: Facing,
    pub lit: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Furnace {
    fn to_id(&self) -> i32 {
        if self.r#lit == true && self.r#facing == Facing::West { return 5130; }
        if self.r#facing == Facing::North && self.r#lit == true { return 5126; }
        if self.r#lit == false && self.r#facing == Facing::North { return 5127; }
        if self.r#lit == false && self.r#facing == Facing::West { return 5131; }
        if self.r#lit == false && self.r#facing == Facing::East { return 5133; }
        if self.r#lit == false && self.r#facing == Facing::South { return 5129; }
        if self.r#lit == true && self.r#facing == Facing::East { return 5132; }
        if self.r#facing == Facing::South && self.r#lit == true { return 5128; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5130 {
            return Some(Furnace {
                r#lit: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 5126 {
            return Some(Furnace {
                r#facing: Facing::North,
                r#lit: true,
            });
        }
        if state_id == 5127 {
            return Some(Furnace {
                r#lit: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 5131 {
            return Some(Furnace {
                r#lit: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 5133 {
            return Some(Furnace {
                r#lit: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5129 {
            return Some(Furnace {
                r#lit: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5132 {
            return Some(Furnace {
                r#lit: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 5128 {
            return Some(Furnace {
                r#facing: Facing::South,
                r#lit: true,
            });
        }
        return None;
    }
}

