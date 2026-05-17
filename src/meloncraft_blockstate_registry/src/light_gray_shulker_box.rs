use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayShulkerBox {
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

impl BlockState for LightGrayShulkerBox {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East { return 14717; }
        if self.r#facing == Facing::Up { return 14720; }
        if self.r#facing == Facing::Down { return 14721; }
        if self.r#facing == Facing::West { return 14719; }
        if self.r#facing == Facing::South { return 14718; }
        if self.r#facing == Facing::North { return 14716; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14717 {
            return Some(LightGrayShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14720 {
            return Some(LightGrayShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14721 {
            return Some(LightGrayShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14719 {
            return Some(LightGrayShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14718 {
            return Some(LightGrayShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14716 {
            return Some(LightGrayShulkerBox {
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

