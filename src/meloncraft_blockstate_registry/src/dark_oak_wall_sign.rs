use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakWallSign {
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

impl BlockState for DarkOakWallSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#facing == Facing::North { return 5674; }
        if self.r#facing == Facing::East && self.r#waterlogged == false { return 5681; }
        if self.r#waterlogged == false && self.r#facing == Facing::South { return 5677; }
        if self.r#facing == Facing::West && self.r#waterlogged == true { return 5678; }
        if self.r#waterlogged == false && self.r#facing == Facing::West { return 5679; }
        if self.r#facing == Facing::South && self.r#waterlogged == true { return 5676; }
        if self.r#facing == Facing::East && self.r#waterlogged == true { return 5680; }
        if self.r#facing == Facing::North && self.r#waterlogged == false { return 5675; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5674 {
            return Some(DarkOakWallSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5681 {
            return Some(DarkOakWallSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 5677 {
            return Some(DarkOakWallSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5678 {
            return Some(DarkOakWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 5679 {
            return Some(DarkOakWallSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 5676 {
            return Some(DarkOakWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5680 {
            return Some(DarkOakWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5675 {
            return Some(DarkOakWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

