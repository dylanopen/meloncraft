use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LargeAmethystBud {
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

impl BlockState for LargeAmethystBud {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::East { return 23217; }
        if self.r#facing == Facing::South && self.r#waterlogged == false { return 23219; }
        if self.r#waterlogged == true && self.r#facing == Facing::East { return 23216; }
        if self.r#facing == Facing::West && self.r#waterlogged == true { return 23220; }
        if self.r#waterlogged == false && self.r#facing == Facing::West { return 23221; }
        if self.r#facing == Facing::Up && self.r#waterlogged == true { return 23222; }
        if self.r#facing == Facing::North && self.r#waterlogged == true { return 23214; }
        if self.r#waterlogged == false && self.r#facing == Facing::Up { return 23223; }
        if self.r#facing == Facing::North && self.r#waterlogged == false { return 23215; }
        if self.r#facing == Facing::South && self.r#waterlogged == true { return 23218; }
        if self.r#waterlogged == true && self.r#facing == Facing::Down { return 23224; }
        if self.r#waterlogged == false && self.r#facing == Facing::Down { return 23225; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23217 {
            return Some(LargeAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 23219 {
            return Some(LargeAmethystBud {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 23216 {
            return Some(LargeAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 23220 {
            return Some(LargeAmethystBud {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 23221 {
            return Some(LargeAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 23222 {
            return Some(LargeAmethystBud {
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 23214 {
            return Some(LargeAmethystBud {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 23223 {
            return Some(LargeAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 23215 {
            return Some(LargeAmethystBud {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 23218 {
            return Some(LargeAmethystBud {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 23224 {
            return Some(LargeAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 23225 {
            return Some(LargeAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}

