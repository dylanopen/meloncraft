use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleWallSign {
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

impl BlockState for JungleWallSign {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#waterlogged == false {
            return 5669;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true {
            return 5666;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::West {
            return 5671;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == false {
            return 5673;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 5670;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true {
            return 5672;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == true {
            return 5668;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == false {
            return 5667;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5669 {
            return Some(JungleWallSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5666 {
            return Some(JungleWallSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 5671 {
            return Some(JungleWallSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 5673 {
            return Some(JungleWallSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 5670 {
            return Some(JungleWallSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 5672 {
            return Some(JungleWallSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5668 {
            return Some(JungleWallSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 5667 {
            return Some(JungleWallSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
