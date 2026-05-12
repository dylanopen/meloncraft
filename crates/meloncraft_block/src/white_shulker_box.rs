use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteShulkerBox {
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

impl BlockState for WhiteShulkerBox {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East { return 14669; }
        if block_state.r#facing == Facing::North { return 14668; }
        if block_state.r#facing == Facing::West { return 14671; }
        if block_state.r#facing == Facing::Up { return 14672; }
        if block_state.r#facing == Facing::Down { return 14673; }
        if block_state.r#facing == Facing::South { return 14670; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14669 {
            return Some(WhiteShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14668 {
            return Some(WhiteShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14671 {
            return Some(WhiteShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14672 {
            return Some(WhiteShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14673 {
            return Some(WhiteShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14670 {
            return Some(WhiteShulkerBox {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

