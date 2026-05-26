use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleShulkerBox {
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

impl BlockState for PurpleShulkerBox {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West {
            return 14731;
        }
        if self.r#facing == Facing::North {
            return 14728;
        }
        if self.r#facing == Facing::Up {
            return 14732;
        }
        if self.r#facing == Facing::East {
            return 14729;
        }
        if self.r#facing == Facing::South {
            return 14730;
        }
        if self.r#facing == Facing::Down {
            return 14733;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14731 {
            return Some(PurpleShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14728 {
            return Some(PurpleShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14732 {
            return Some(PurpleShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14729 {
            return Some(PurpleShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14730 {
            return Some(PurpleShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14733 {
            return Some(PurpleShulkerBox {
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}
