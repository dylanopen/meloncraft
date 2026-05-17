use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooWallHangingSign {
    pub r#facing: Facing,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BambooWallHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#facing == Facing::South { return 6564; }
        if self.r#waterlogged == false && self.r#facing == Facing::South { return 6565; }
        if self.r#facing == Facing::West && self.r#waterlogged == false { return 6567; }
        if self.r#facing == Facing::East && self.r#waterlogged == true { return 6568; }
        if self.r#waterlogged == false && self.r#facing == Facing::East { return 6569; }
        if self.r#waterlogged == false && self.r#facing == Facing::North { return 6563; }
        if self.r#facing == Facing::North && self.r#waterlogged == true { return 6562; }
        if self.r#waterlogged == true && self.r#facing == Facing::West { return 6566; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6564 {
            return Some(BambooWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6565 {
            return Some(BambooWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6567 {
            return Some(BambooWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6568 {
            return Some(BambooWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 6569 {
            return Some(BambooWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6563 {
            return Some(BambooWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6562 {
            return Some(BambooWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 6566 {
            return Some(BambooWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

