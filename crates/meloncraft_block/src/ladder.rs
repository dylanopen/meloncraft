use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ladder {
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

impl BlockState for Ladder {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#facing == Facing::East { return 5524; }
        if self.r#waterlogged == true && self.r#facing == Facing::West { return 5522; }
        if self.r#facing == Facing::South && self.r#waterlogged == false { return 5521; }
        if self.r#waterlogged == false && self.r#facing == Facing::North { return 5519; }
        if self.r#waterlogged == true && self.r#facing == Facing::South { return 5520; }
        if self.r#facing == Facing::North && self.r#waterlogged == true { return 5518; }
        if self.r#waterlogged == false && self.r#facing == Facing::East { return 5525; }
        if self.r#facing == Facing::West && self.r#waterlogged == false { return 5523; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5524 {
            return Some(Ladder {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 5522 {
            return Some(Ladder {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 5521 {
            return Some(Ladder {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5519 {
            return Some(Ladder {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 5520 {
            return Some(Ladder {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 5518 {
            return Some(Ladder {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 5525 {
            return Some(Ladder {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5523 {
            return Some(Ladder {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

