use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainCommandBlock {
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

impl BlockState for ChainCommandBlock {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#conditional == true { return 14630; }
        if block_state.r#facing == Facing::North && block_state.r#conditional == true { return 14627; }
        if block_state.r#facing == Facing::East && block_state.r#conditional == true { return 14628; }
        if block_state.r#facing == Facing::Down && block_state.r#conditional == true { return 14632; }
        if block_state.r#conditional == false && block_state.r#facing == Facing::West { return 14636; }
        if block_state.r#facing == Facing::South && block_state.r#conditional == true { return 14629; }
        if block_state.r#conditional == false && block_state.r#facing == Facing::Up { return 14637; }
        if block_state.r#facing == Facing::East && block_state.r#conditional == false { return 14634; }
        if block_state.r#conditional == true && block_state.r#facing == Facing::Up { return 14631; }
        if block_state.r#conditional == false && block_state.r#facing == Facing::South { return 14635; }
        if block_state.r#conditional == false && block_state.r#facing == Facing::North { return 14633; }
        if block_state.r#facing == Facing::Down && block_state.r#conditional == false { return 14638; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14630 {
            return Some(ChainCommandBlock {
                r#facing: Facing::West,
                r#conditional: true,
            });
        }
        if state_id == 14627 {
            return Some(ChainCommandBlock {
                r#facing: Facing::North,
                r#conditional: true,
            });
        }
        if state_id == 14628 {
            return Some(ChainCommandBlock {
                r#facing: Facing::East,
                r#conditional: true,
            });
        }
        if state_id == 14632 {
            return Some(ChainCommandBlock {
                r#facing: Facing::Down,
                r#conditional: true,
            });
        }
        if state_id == 14636 {
            return Some(ChainCommandBlock {
                r#conditional: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 14629 {
            return Some(ChainCommandBlock {
                r#facing: Facing::South,
                r#conditional: true,
            });
        }
        if state_id == 14637 {
            return Some(ChainCommandBlock {
                r#conditional: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 14634 {
            return Some(ChainCommandBlock {
                r#facing: Facing::East,
                r#conditional: false,
            });
        }
        if state_id == 14631 {
            return Some(ChainCommandBlock {
                r#conditional: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 14635 {
            return Some(ChainCommandBlock {
                r#conditional: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14633 {
            return Some(ChainCommandBlock {
                r#conditional: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14638 {
            return Some(ChainCommandBlock {
                r#facing: Facing::Down,
                r#conditional: false,
            });
        }
        return None;
    }
}

