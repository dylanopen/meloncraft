use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadFireCoralWallFan {
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

impl BlockState for DeadFireCoralWallFan {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#waterlogged == true { return 15015; }
        if self.r#facing == Facing::West && self.r#waterlogged == true { return 15013; }
        if self.r#facing == Facing::North && self.r#waterlogged == false { return 15010; }
        if self.r#waterlogged == true && self.r#facing == Facing::North { return 15009; }
        if self.r#waterlogged == false && self.r#facing == Facing::South { return 15012; }
        if self.r#waterlogged == false && self.r#facing == Facing::East { return 15016; }
        if self.r#facing == Facing::West && self.r#waterlogged == false { return 15014; }
        if self.r#waterlogged == true && self.r#facing == Facing::South { return 15011; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15015 {
            return Some(DeadFireCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15013 {
            return Some(DeadFireCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15010 {
            return Some(DeadFireCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15009 {
            return Some(DeadFireCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15012 {
            return Some(DeadFireCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15016 {
            return Some(DeadFireCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15014 {
            return Some(DeadFireCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15011 {
            return Some(DeadFireCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

