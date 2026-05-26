use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeShulkerBox {
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

impl BlockState for OrangeShulkerBox {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North {
            return 14674;
        }
        if self.r#facing == Facing::East {
            return 14675;
        }
        if self.r#facing == Facing::South {
            return 14676;
        }
        if self.r#facing == Facing::West {
            return 14677;
        }
        if self.r#facing == Facing::Down {
            return 14679;
        }
        if self.r#facing == Facing::Up {
            return 14678;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14674 {
            return Some(OrangeShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14675 {
            return Some(OrangeShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14676 {
            return Some(OrangeShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14677 {
            return Some(OrangeShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14679 {
            return Some(OrangeShulkerBox {
                r#facing: Facing::Down,
            });
        }
        if state_id == 14678 {
            return Some(OrangeShulkerBox {
                r#facing: Facing::Up,
            });
        }
        return None;
    }
}
