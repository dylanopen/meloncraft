use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaShulkerBox {
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

impl BlockState for MagentaShulkerBox {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North { return 14680; }
        if self.r#facing == Facing::South { return 14682; }
        if self.r#facing == Facing::East { return 14681; }
        if self.r#facing == Facing::Up { return 14684; }
        if self.r#facing == Facing::West { return 14683; }
        if self.r#facing == Facing::Down { return 14685; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14680 {
            return Some(MagentaShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14682 {
            return Some(MagentaShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14681 {
            return Some(MagentaShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14684 {
            return Some(MagentaShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14683 {
            return Some(MagentaShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14685 {
            return Some(MagentaShulkerBox {
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}

