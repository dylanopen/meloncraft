use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowShulkerBox {
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

impl BlockState for YellowShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North { return 14692; }
        if block_state.r#facing == Facing::South { return 14694; }
        if block_state.r#facing == Facing::West { return 14695; }
        if block_state.r#facing == Facing::East { return 14693; }
        if block_state.r#facing == Facing::Down { return 14697; }
        if block_state.r#facing == Facing::Up { return 14696; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14692 {
            return Some(YellowShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14694 {
            return Some(YellowShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14695 {
            return Some(YellowShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14693 {
            return Some(YellowShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14697 {
            return Some(YellowShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14696 {
            return Some(YellowShulkerBox {
                r#facing: Facing::Up,
            });
        }
        return None;
    }
}

