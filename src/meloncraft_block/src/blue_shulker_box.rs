use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueShulkerBox {
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

impl BlockState for BlueShulkerBox {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::Up { return 14738; }
        if self.r#facing == Facing::South { return 14736; }
        if self.r#facing == Facing::East { return 14735; }
        if self.r#facing == Facing::North { return 14734; }
        if self.r#facing == Facing::West { return 14737; }
        if self.r#facing == Facing::Down { return 14739; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14738 {
            return Some(BlueShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14736 {
            return Some(BlueShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14735 {
            return Some(BlueShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14734 {
            return Some(BlueShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14737 {
            return Some(BlueShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14739 {
            return Some(BlueShulkerBox {
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}

