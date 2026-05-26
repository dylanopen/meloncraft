use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TubeCoralWallFan {
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

impl BlockState for TubeCoralWallFan {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::North {
            return 15026;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true {
            return 15025;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::West {
            return 15030;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South {
            return 15027;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::South {
            return 15028;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::East {
            return 15032;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::West {
            return 15029;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true {
            return 15031;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15026 {
            return Some(TubeCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15025 {
            return Some(TubeCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15030 {
            return Some(TubeCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15027 {
            return Some(TubeCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15028 {
            return Some(TubeCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15032 {
            return Some(TubeCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15029 {
            return Some(TubeCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15031 {
            return Some(TubeCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
