use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediumAmethystBud {
    pub waterlogged: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for MediumAmethystBud {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#facing == Facing::Up { return 23234; }
        if self.r#facing == Facing::Up && self.r#waterlogged == false { return 23235; }
        if self.r#waterlogged == true && self.r#facing == Facing::East { return 23228; }
        if self.r#facing == Facing::East && self.r#waterlogged == false { return 23229; }
        if self.r#facing == Facing::West && self.r#waterlogged == false { return 23233; }
        if self.r#facing == Facing::Down && self.r#waterlogged == false { return 23237; }
        if self.r#facing == Facing::Down && self.r#waterlogged == true { return 23236; }
        if self.r#waterlogged == true && self.r#facing == Facing::South { return 23230; }
        if self.r#facing == Facing::North && self.r#waterlogged == true { return 23226; }
        if self.r#facing == Facing::South && self.r#waterlogged == false { return 23231; }
        if self.r#facing == Facing::West && self.r#waterlogged == true { return 23232; }
        if self.r#facing == Facing::North && self.r#waterlogged == false { return 23227; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23234 {
            return Some(MediumAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 23235 {
            return Some(MediumAmethystBud {
                r#facing: Facing::Up,
                r#waterlogged: false,
            });
        }
        if state_id == 23228 {
            return Some(MediumAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 23229 {
            return Some(MediumAmethystBud {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 23233 {
            return Some(MediumAmethystBud {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 23237 {
            return Some(MediumAmethystBud {
                r#facing: Facing::Down,
                r#waterlogged: false,
            });
        }
        if state_id == 23236 {
            return Some(MediumAmethystBud {
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 23230 {
            return Some(MediumAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 23226 {
            return Some(MediumAmethystBud {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 23231 {
            return Some(MediumAmethystBud {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 23232 {
            return Some(MediumAmethystBud {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 23227 {
            return Some(MediumAmethystBud {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

