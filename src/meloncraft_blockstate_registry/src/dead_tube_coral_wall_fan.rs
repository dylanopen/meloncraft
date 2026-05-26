use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadTubeCoralWallFan {
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

impl BlockState for DeadTubeCoralWallFan {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#waterlogged == false {
            return 14988;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 14989;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true {
            return 14991;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::East {
            return 14992;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == false {
            return 14990;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == true {
            return 14987;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == false {
            return 14986;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true {
            return 14985;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14988 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 14989 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 14991 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 14992 {
            return Some(DeadTubeCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14990 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 14987 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 14986 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 14985 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
