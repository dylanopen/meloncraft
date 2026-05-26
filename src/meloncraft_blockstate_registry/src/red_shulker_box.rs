use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedShulkerBox {
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

impl BlockState for RedShulkerBox {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West {
            return 14755;
        }
        if self.r#facing == Facing::Down {
            return 14757;
        }
        if self.r#facing == Facing::Up {
            return 14756;
        }
        if self.r#facing == Facing::North {
            return 14752;
        }
        if self.r#facing == Facing::East {
            return 14753;
        }
        if self.r#facing == Facing::South {
            return 14754;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14755 {
            return Some(RedShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14757 {
            return Some(RedShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14756 {
            return Some(RedShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14752 {
            return Some(RedShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14753 {
            return Some(RedShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14754 {
            return Some(RedShulkerBox {
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
