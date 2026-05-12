use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeShulkerBox {
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

impl BlockState for LimeShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West { return 14701; }
        if block_state.r#facing == Facing::Down { return 14703; }
        if block_state.r#facing == Facing::Up { return 14702; }
        if block_state.r#facing == Facing::North { return 14698; }
        if block_state.r#facing == Facing::East { return 14699; }
        if block_state.r#facing == Facing::South { return 14700; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14701 {
            return Some(LimeShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14703 {
            return Some(LimeShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14702 {
            return Some(LimeShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14698 {
            return Some(LimeShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14699 {
            return Some(LimeShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14700 {
            return Some(LimeShulkerBox {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

