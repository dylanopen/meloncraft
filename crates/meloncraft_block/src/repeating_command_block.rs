use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatingCommandBlock {
    pub conditional: bool,
    pub r#facing: Facing,
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

impl BlockState for RepeatingCommandBlock {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#conditional == false { return 14623; }
        if block_state.r#conditional == true && block_state.r#facing == Facing::North { return 14615; }
        if block_state.r#conditional == true && block_state.r#facing == Facing::West { return 14618; }
        if block_state.r#facing == Facing::Up && block_state.r#conditional == true { return 14619; }
        if block_state.r#facing == Facing::North && block_state.r#conditional == false { return 14621; }
        if block_state.r#conditional == false && block_state.r#facing == Facing::Down { return 14626; }
        if block_state.r#conditional == false && block_state.r#facing == Facing::Up { return 14625; }
        if block_state.r#conditional == true && block_state.r#facing == Facing::South { return 14617; }
        if block_state.r#conditional == true && block_state.r#facing == Facing::East { return 14616; }
        if block_state.r#conditional == true && block_state.r#facing == Facing::Down { return 14620; }
        if block_state.r#conditional == false && block_state.r#facing == Facing::East { return 14622; }
        if block_state.r#facing == Facing::West && block_state.r#conditional == false { return 14624; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14623 {
            return Some(RepeatingCommandBlock {
                r#facing: Facing::South,
                r#conditional: false,
            });
        }
        if state_id == 14615 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14618 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14619 {
            return Some(RepeatingCommandBlock {
                r#facing: Facing::Up,
                r#conditional: true,
            });
        }
        if state_id == 14621 {
            return Some(RepeatingCommandBlock {
                r#facing: Facing::North,
                r#conditional: false,
            });
        }
        if state_id == 14626 {
            return Some(RepeatingCommandBlock {
                r#conditional: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 14625 {
            return Some(RepeatingCommandBlock {
                r#conditional: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 14617 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14616 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14620 {
            return Some(RepeatingCommandBlock {
                r#conditional: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 14622 {
            return Some(RepeatingCommandBlock {
                r#conditional: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14624 {
            return Some(RepeatingCommandBlock {
                r#facing: Facing::West,
                r#conditional: false,
            });
        }
        return None;
    }
}

