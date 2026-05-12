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
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#lit == false { return 20561; }
        if block_state.r#lit == false && block_state.r#facing == Facing::West { return 20565; }
        if block_state.r#facing == Facing::South && block_state.r#lit == true { return 20562; }
        if block_state.r#facing == Facing::West && block_state.r#lit == true { return 20564; }
        if block_state.r#facing == Facing::East && block_state.r#lit == false { return 20567; }
        if block_state.r#facing == Facing::East && block_state.r#lit == true { return 20566; }
        if block_state.r#lit == true && block_state.r#facing == Facing::North { return 20560; }
        if block_state.r#lit == false && block_state.r#facing == Facing::South { return 20563; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20561 {
            return Some(BlastFurnace {
                r#facing: Facing::North,
                r#lit: false,
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
                r#facing: Facing::South,
                r#lit: true,
            });
        }
        if state_id == 20564 {
            return Some(BlastFurnace {
                r#facing: Facing::West,
                r#lit: true,
            });
        }
        if state_id == 20567 {
            return Some(BlastFurnace {
                r#facing: Facing::East,
                r#lit: false,
            });
        }
        if state_id == 20566 {
            return Some(BlastFurnace {
                r#facing: Facing::East,
                r#lit: true,
            });
        }
        if state_id == 20560 {
            return Some(BlastFurnace {
                r#lit: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 20563 {
            return Some(BlastFurnace {
                r#lit: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

