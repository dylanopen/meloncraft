use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HornCoralWallFan {
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

impl BlockState for HornCoralWallFan {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 15061;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == false {
            return 15058;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == false {
            return 15064;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::North {
            return 15057;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::West {
            return 15062;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true {
            return 15063;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == true {
            return 15059;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::South {
            return 15060;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15061 {
            return Some(HornCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15058 {
            return Some(HornCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15064 {
            return Some(HornCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15057 {
            return Some(HornCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15062 {
            return Some(HornCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15063 {
            return Some(HornCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15059 {
            return Some(HornCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15060 {
            return Some(HornCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
