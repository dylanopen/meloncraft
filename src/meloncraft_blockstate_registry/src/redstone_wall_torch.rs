use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedstoneWallTorch {
    pub r#facing: Facing,
    pub lit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for RedstoneWallTorch {
    fn to_id(&self) -> i32 {
        if self.r#lit == false && self.r#facing == Facing::North {
            return 6687;
        }
        if self.r#facing == Facing::East && self.r#lit == true {
            return 6692;
        }
        if self.r#facing == Facing::South && self.r#lit == false {
            return 6689;
        }
        if self.r#facing == Facing::South && self.r#lit == true {
            return 6688;
        }
        if self.r#lit == true && self.r#facing == Facing::North {
            return 6686;
        }
        if self.r#facing == Facing::West && self.r#lit == true {
            return 6690;
        }
        if self.r#facing == Facing::West && self.r#lit == false {
            return 6691;
        }
        if self.r#lit == false && self.r#facing == Facing::East {
            return 6693;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6687 {
            return Some(RedstoneWallTorch {
                r#lit: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6692 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::East,
                r#lit: true,
            });
        }
        if state_id == 6689 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::South,
                r#lit: false,
            });
        }
        if state_id == 6688 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::South,
                r#lit: true,
            });
        }
        if state_id == 6686 {
            return Some(RedstoneWallTorch {
                r#lit: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6690 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::West,
                r#lit: true,
            });
        }
        if state_id == 6691 {
            return Some(RedstoneWallTorch {
                r#facing: Facing::West,
                r#lit: false,
            });
        }
        if state_id == 6693 {
            return Some(RedstoneWallTorch {
                r#lit: false,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
