use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayShulkerBox {
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

impl BlockState for GrayShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West { return 14713; }
        if block_state.r#facing == Facing::Up { return 14714; }
        if block_state.r#facing == Facing::Down { return 14715; }
        if block_state.r#facing == Facing::North { return 14710; }
        if block_state.r#facing == Facing::East { return 14711; }
        if block_state.r#facing == Facing::South { return 14712; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14713 {
            return Some(GrayShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14714 {
            return Some(GrayShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14715 {
            return Some(GrayShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14710 {
            return Some(GrayShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14711 {
            return Some(GrayShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14712 {
            return Some(GrayShulkerBox {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

