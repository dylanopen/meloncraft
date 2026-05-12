use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandBlock {
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

impl BlockState for CommandBlock {
    fn to_id(self) -> i32 {
        if block_state.r#conditional == true && block_state.r#facing == Facing::West { return 9770; }
        if block_state.r#facing == Facing::Down && block_state.r#conditional == true { return 9772; }
        if block_state.r#facing == Facing::North && block_state.r#conditional == false { return 9773; }
        if block_state.r#conditional == false && block_state.r#facing == Facing::West { return 9776; }
        if block_state.r#conditional == true && block_state.r#facing == Facing::South { return 9769; }
        if block_state.r#facing == Facing::South && block_state.r#conditional == false { return 9775; }
        if block_state.r#conditional == true && block_state.r#facing == Facing::Up { return 9771; }
        if block_state.r#facing == Facing::North && block_state.r#conditional == true { return 9767; }
        if block_state.r#conditional == false && block_state.r#facing == Facing::Down { return 9778; }
        if block_state.r#conditional == true && block_state.r#facing == Facing::East { return 9768; }
        if block_state.r#facing == Facing::East && block_state.r#conditional == false { return 9774; }
        if block_state.r#facing == Facing::Up && block_state.r#conditional == false { return 9777; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9770 {
            return Some(CommandBlock {
                r#conditional: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 9772 {
            return Some(CommandBlock {
                r#facing: Facing::Down,
                r#conditional: true,
            });
        }
        if state_id == 9773 {
            return Some(CommandBlock {
                r#facing: Facing::North,
                r#conditional: false,
            });
        }
        if state_id == 9776 {
            return Some(CommandBlock {
                r#conditional: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 9769 {
            return Some(CommandBlock {
                r#conditional: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9775 {
            return Some(CommandBlock {
                r#facing: Facing::South,
                r#conditional: false,
            });
        }
        if state_id == 9771 {
            return Some(CommandBlock {
                r#conditional: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 9767 {
            return Some(CommandBlock {
                r#facing: Facing::North,
                r#conditional: true,
            });
        }
        if state_id == 9778 {
            return Some(CommandBlock {
                r#conditional: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 9768 {
            return Some(CommandBlock {
                r#conditional: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 9774 {
            return Some(CommandBlock {
                r#facing: Facing::East,
                r#conditional: false,
            });
        }
        if state_id == 9777 {
            return Some(CommandBlock {
                r#facing: Facing::Up,
                r#conditional: false,
            });
        }
        return None;
    }
}

