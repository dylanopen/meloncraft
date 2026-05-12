use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanShulkerBox {
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

impl BlockState for CyanShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::Up { return 14726; }
        if block_state.r#facing == Facing::North { return 14722; }
        if block_state.r#facing == Facing::Down { return 14727; }
        if block_state.r#facing == Facing::East { return 14723; }
        if block_state.r#facing == Facing::South { return 14724; }
        if block_state.r#facing == Facing::West { return 14725; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14726 {
            return Some(CyanShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14722 {
            return Some(CyanShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14727 {
            return Some(CyanShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14723 {
            return Some(CyanShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14724 {
            return Some(CyanShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14725 {
            return Some(CyanShulkerBox {
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

