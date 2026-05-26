use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedWallSign {
    pub waterlogged: bool,
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WarpedWallSign {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#waterlogged == false {
            return 21519;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::East {
            return 21518;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true {
            return 21512;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == false {
            return 21515;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == false {
            return 21513;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == false {
            return 21517;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 21516;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South {
            return 21514;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21519 {
            return Some(WarpedWallSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 21518 {
            return Some(WarpedWallSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 21512 {
            return Some(WarpedWallSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 21515 {
            return Some(WarpedWallSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 21513 {
            return Some(WarpedWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 21517 {
            return Some(WarpedWallSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 21516 {
            return Some(WarpedWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 21514 {
            return Some(WarpedWallSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
