use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenShulkerBox {
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

impl BlockState for GreenShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South { return 14748; }
        if block_state.r#facing == Facing::East { return 14747; }
        if block_state.r#facing == Facing::Up { return 14750; }
        if block_state.r#facing == Facing::West { return 14749; }
        if block_state.r#facing == Facing::North { return 14746; }
        if block_state.r#facing == Facing::Down { return 14751; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14748 {
            return Some(GreenShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14747 {
            return Some(GreenShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14750 {
            return Some(GreenShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14749 {
            return Some(GreenShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14746 {
            return Some(GreenShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14751 {
            return Some(GreenShulkerBox {
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}

