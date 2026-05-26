use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchWallSign {
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

impl BlockState for BirchWallSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::West {
            return 5647;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South {
            return 5644;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true {
            return 5642;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == false {
            return 5649;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true {
            return 5648;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == false {
            return 5645;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == false {
            return 5643;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 5646;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5647 {
            return Some(BirchWallSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 5644 {
            return Some(BirchWallSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 5642 {
            return Some(BirchWallSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 5649 {
            return Some(BirchWallSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 5648 {
            return Some(BirchWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5645 {
            return Some(BirchWallSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5643 {
            return Some(BirchWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 5646 {
            return Some(BirchWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
