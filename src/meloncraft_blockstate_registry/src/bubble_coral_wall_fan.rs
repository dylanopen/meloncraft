use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BubbleCoralWallFan {
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

impl BlockState for BubbleCoralWallFan {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::South {
            return 15044;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == false {
            return 15046;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South {
            return 15043;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 15045;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true {
            return 15047;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::East {
            return 15048;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::North {
            return 15042;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::North {
            return 15041;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15044 {
            return Some(BubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15046 {
            return Some(BubbleCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15043 {
            return Some(BubbleCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15045 {
            return Some(BubbleCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15047 {
            return Some(BubbleCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15048 {
            return Some(BubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15042 {
            return Some(BubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15041 {
            return Some(BubbleCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
