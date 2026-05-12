use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkPetals {
    pub flower_amount: i32,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PinkPetals {
    fn to_id(self) -> i32 {
        if block_state.r#flower_amount == 3 && block_state.r#facing == Facing::West { return 27622; }
        if block_state.r#facing == Facing::West && block_state.r#flower_amount == 2 { return 27621; }
        if block_state.r#flower_amount == 3 && block_state.r#facing == Facing::South { return 27618; }
        if block_state.r#flower_amount == 4 && block_state.r#facing == Facing::North { return 27615; }
        if block_state.r#facing == Facing::East && block_state.r#flower_amount == 4 { return 27627; }
        if block_state.r#facing == Facing::East && block_state.r#flower_amount == 3 { return 27626; }
        if block_state.r#flower_amount == 1 && block_state.r#facing == Facing::North { return 27612; }
        if block_state.r#facing == Facing::South && block_state.r#flower_amount == 1 { return 27616; }
        if block_state.r#facing == Facing::North && block_state.r#flower_amount == 2 { return 27613; }
        if block_state.r#facing == Facing::West && block_state.r#flower_amount == 4 { return 27623; }
        if block_state.r#facing == Facing::East && block_state.r#flower_amount == 2 { return 27625; }
        if block_state.r#flower_amount == 3 && block_state.r#facing == Facing::North { return 27614; }
        if block_state.r#facing == Facing::East && block_state.r#flower_amount == 1 { return 27624; }
        if block_state.r#facing == Facing::West && block_state.r#flower_amount == 1 { return 27620; }
        if block_state.r#flower_amount == 4 && block_state.r#facing == Facing::South { return 27619; }
        if block_state.r#facing == Facing::South && block_state.r#flower_amount == 2 { return 27617; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27622 {
            return Some(PinkPetals {
                r#flower_amount: 3,
                r#facing: Facing::West,
            });
        }
        if state_id == 27621 {
            return Some(PinkPetals {
                r#facing: Facing::West,
                r#flower_amount: 2,
            });
        }
        if state_id == 27618 {
            return Some(PinkPetals {
                r#flower_amount: 3,
                r#facing: Facing::South,
            });
        }
        if state_id == 27615 {
            return Some(PinkPetals {
                r#flower_amount: 4,
                r#facing: Facing::North,
            });
        }
        if state_id == 27627 {
            return Some(PinkPetals {
                r#facing: Facing::East,
                r#flower_amount: 4,
            });
        }
        if state_id == 27626 {
            return Some(PinkPetals {
                r#facing: Facing::East,
                r#flower_amount: 3,
            });
        }
        if state_id == 27612 {
            return Some(PinkPetals {
                r#flower_amount: 1,
                r#facing: Facing::North,
            });
        }
        if state_id == 27616 {
            return Some(PinkPetals {
                r#facing: Facing::South,
                r#flower_amount: 1,
            });
        }
        if state_id == 27613 {
            return Some(PinkPetals {
                r#facing: Facing::North,
                r#flower_amount: 2,
            });
        }
        if state_id == 27623 {
            return Some(PinkPetals {
                r#facing: Facing::West,
                r#flower_amount: 4,
            });
        }
        if state_id == 27625 {
            return Some(PinkPetals {
                r#facing: Facing::East,
                r#flower_amount: 2,
            });
        }
        if state_id == 27614 {
            return Some(PinkPetals {
                r#flower_amount: 3,
                r#facing: Facing::North,
            });
        }
        if state_id == 27624 {
            return Some(PinkPetals {
                r#facing: Facing::East,
                r#flower_amount: 1,
            });
        }
        if state_id == 27620 {
            return Some(PinkPetals {
                r#facing: Facing::West,
                r#flower_amount: 1,
            });
        }
        if state_id == 27619 {
            return Some(PinkPetals {
                r#flower_amount: 4,
                r#facing: Facing::South,
            });
        }
        if state_id == 27617 {
            return Some(PinkPetals {
                r#facing: Facing::South,
                r#flower_amount: 2,
            });
        }
        return None;
    }
}

