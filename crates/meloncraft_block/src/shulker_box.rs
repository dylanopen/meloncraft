use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShulkerBox {
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

impl BlockState for ShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West { return 14665; }
        if block_state.r#facing == Facing::Up { return 14666; }
        if block_state.r#facing == Facing::Down { return 14667; }
        if block_state.r#facing == Facing::East { return 14663; }
        if block_state.r#facing == Facing::North { return 14662; }
        if block_state.r#facing == Facing::South { return 14664; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14665 {
            return Some(ShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14666 {
            return Some(ShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14667 {
            return Some(ShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14663 {
            return Some(ShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14662 {
            return Some(ShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14664 {
            return Some(ShulkerBox {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

