use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlastFurnace {
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

impl BlockState for BlastFurnace {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#lit == false {
            return 20563;
        }
        if self.r#facing == Facing::North && self.r#lit == true {
            return 20560;
        }
        if self.r#lit == false && self.r#facing == Facing::East {
            return 20567;
        }
        if self.r#facing == Facing::North && self.r#lit == false {
            return 20561;
        }
        if self.r#facing == Facing::East && self.r#lit == true {
            return 20566;
        }
        if self.r#lit == false && self.r#facing == Facing::West {
            return 20565;
        }
        if self.r#lit == true && self.r#facing == Facing::South {
            return 20562;
        }
        if self.r#facing == Facing::West && self.r#lit == true {
            return 20564;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20563 {
            return Some(BlastFurnace {
                r#facing: Facing::South,
                r#lit: false,
            });
        }
        if state_id == 20560 {
            return Some(BlastFurnace {
                r#facing: Facing::North,
                r#lit: true,
            });
        }
        if state_id == 20567 {
            return Some(BlastFurnace {
                r#lit: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 20561 {
            return Some(BlastFurnace {
                r#facing: Facing::North,
                r#lit: false,
            });
        }
        if state_id == 20566 {
            return Some(BlastFurnace {
                r#facing: Facing::East,
                r#lit: true,
            });
        }
        if state_id == 20565 {
            return Some(BlastFurnace {
                r#lit: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 20562 {
            return Some(BlastFurnace {
                r#lit: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 20564 {
            return Some(BlastFurnace {
                r#facing: Facing::West,
                r#lit: true,
            });
        }
        return None;
    }
}
