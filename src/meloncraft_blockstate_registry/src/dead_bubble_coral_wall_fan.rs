use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadBubbleCoralWallFan {
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

impl BlockState for DeadBubbleCoralWallFan {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::East {
            return 15008;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::West {
            return 15006;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true {
            return 15001;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::North {
            return 15002;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == true {
            return 15003;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == false {
            return 15004;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::East {
            return 15007;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 15005;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15008 {
            return Some(DeadBubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15006 {
            return Some(DeadBubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15001 {
            return Some(DeadBubbleCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15002 {
            return Some(DeadBubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15003 {
            return Some(DeadBubbleCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15004 {
            return Some(DeadBubbleCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15007 {
            return Some(DeadBubbleCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15005 {
            return Some(DeadBubbleCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
