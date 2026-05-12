use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackShulkerBox {
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

impl BlockState for BlackShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::Up { return 14762; }
        if block_state.r#facing == Facing::East { return 14759; }
        if block_state.r#facing == Facing::South { return 14760; }
        if block_state.r#facing == Facing::West { return 14761; }
        if block_state.r#facing == Facing::Down { return 14763; }
        if block_state.r#facing == Facing::North { return 14758; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14762 {
            return Some(BlackShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14759 {
            return Some(BlackShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14760 {
            return Some(BlackShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14761 {
            return Some(BlackShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14763 {
            return Some(BlackShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14758 {
            return Some(BlackShulkerBox {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

