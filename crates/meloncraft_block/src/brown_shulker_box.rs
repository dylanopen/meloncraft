use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownShulkerBox {
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

impl BlockState for BrownShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South { return 14742; }
        if block_state.r#facing == Facing::North { return 14740; }
        if block_state.r#facing == Facing::East { return 14741; }
        if block_state.r#facing == Facing::West { return 14743; }
        if block_state.r#facing == Facing::Up { return 14744; }
        if block_state.r#facing == Facing::Down { return 14745; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14742 {
            return Some(BrownShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14740 {
            return Some(BrownShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14741 {
            return Some(BrownShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14743 {
            return Some(BrownShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14744 {
            return Some(BrownShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14745 {
            return Some(BrownShulkerBox {
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}

