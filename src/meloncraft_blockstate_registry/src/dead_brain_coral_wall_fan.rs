use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadBrainCoralWallFan {
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

impl BlockState for DeadBrainCoralWallFan {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#waterlogged == false {
            return 14996;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South {
            return 14995;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 14997;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true {
            return 14999;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::East {
            return 15000;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == false {
            return 14998;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::North {
            return 14993;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == false {
            return 14994;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14996 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 14995 {
            return Some(DeadBrainCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14997 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 14999 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15000 {
            return Some(DeadBrainCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14998 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 14993 {
            return Some(DeadBrainCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14994 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
