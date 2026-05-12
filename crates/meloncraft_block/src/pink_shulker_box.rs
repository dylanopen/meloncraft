use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkShulkerBox {
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

impl BlockState for PinkShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North { return 14704; }
        if block_state.r#facing == Facing::Up { return 14708; }
        if block_state.r#facing == Facing::West { return 14707; }
        if block_state.r#facing == Facing::Down { return 14709; }
        if block_state.r#facing == Facing::East { return 14705; }
        if block_state.r#facing == Facing::South { return 14706; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14704 {
            return Some(PinkShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14708 {
            return Some(PinkShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14707 {
            return Some(PinkShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14709 {
            return Some(PinkShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14705 {
            return Some(PinkShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14706 {
            return Some(PinkShulkerBox {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

