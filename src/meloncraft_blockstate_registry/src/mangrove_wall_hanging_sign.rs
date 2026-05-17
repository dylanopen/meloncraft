use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveWallHangingSign {
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

impl BlockState for MangroveWallHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#waterlogged == true { return 6544; }
        if self.r#facing == Facing::South && self.r#waterlogged == true { return 6540; }
        if self.r#facing == Facing::West && self.r#waterlogged == false { return 6543; }
        if self.r#facing == Facing::North && self.r#waterlogged == false { return 6539; }
        if self.r#facing == Facing::East && self.r#waterlogged == false { return 6545; }
        if self.r#facing == Facing::West && self.r#waterlogged == true { return 6542; }
        if self.r#facing == Facing::South && self.r#waterlogged == false { return 6541; }
        if self.r#facing == Facing::North && self.r#waterlogged == true { return 6538; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6544 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 6540 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6543 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6539 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 6545 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 6542 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 6541 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 6538 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

