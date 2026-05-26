use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueShulkerBox {
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

impl BlockState for LightBlueShulkerBox {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South {
            return 14688;
        }
        if self.r#facing == Facing::East {
            return 14687;
        }
        if self.r#facing == Facing::Up {
            return 14690;
        }
        if self.r#facing == Facing::North {
            return 14686;
        }
        if self.r#facing == Facing::West {
            return 14689;
        }
        if self.r#facing == Facing::Down {
            return 14691;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14688 {
            return Some(LightBlueShulkerBox {
                r#facing: Facing::South,
            });
        }
        if state_id == 14687 {
            return Some(LightBlueShulkerBox {
                r#facing: Facing::East,
            });
        }
        if state_id == 14690 {
            return Some(LightBlueShulkerBox {
                r#facing: Facing::Up,
            });
        }
        if state_id == 14686 {
            return Some(LightBlueShulkerBox {
                r#facing: Facing::North,
            });
        }
        if state_id == 14689 {
            return Some(LightBlueShulkerBox {
                r#facing: Facing::West,
            });
        }
        if state_id == 14691 {
            return Some(LightBlueShulkerBox {
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}
