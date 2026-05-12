use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StickyPiston {
    pub extended: bool,
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

impl BlockState for StickyPiston {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::Down && block_state.r#extended == true { return 2040; }
        if block_state.r#extended == true && block_state.r#facing == Facing::North { return 2035; }
        if block_state.r#extended == false && block_state.r#facing == Facing::South { return 2043; }
        if block_state.r#extended == true && block_state.r#facing == Facing::East { return 2036; }
        if block_state.r#extended == true && block_state.r#facing == Facing::South { return 2037; }
        if block_state.r#facing == Facing::West && block_state.r#extended == false { return 2044; }
        if block_state.r#facing == Facing::North && block_state.r#extended == false { return 2041; }
        if block_state.r#facing == Facing::Up && block_state.r#extended == false { return 2045; }
        if block_state.r#facing == Facing::West && block_state.r#extended == true { return 2038; }
        if block_state.r#extended == false && block_state.r#facing == Facing::East { return 2042; }
        if block_state.r#extended == false && block_state.r#facing == Facing::Down { return 2046; }
        if block_state.r#extended == true && block_state.r#facing == Facing::Up { return 2039; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2040 {
            return Some(StickyPiston {
                r#facing: Facing::Down,
                r#extended: true,
            });
        }
        if state_id == 2035 {
            return Some(StickyPiston {
                r#extended: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2043 {
            return Some(StickyPiston {
                r#extended: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2036 {
            return Some(StickyPiston {
                r#extended: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2037 {
            return Some(StickyPiston {
                r#extended: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2044 {
            return Some(StickyPiston {
                r#facing: Facing::West,
                r#extended: false,
            });
        }
        if state_id == 2041 {
            return Some(StickyPiston {
                r#facing: Facing::North,
                r#extended: false,
            });
        }
        if state_id == 2045 {
            return Some(StickyPiston {
                r#facing: Facing::Up,
                r#extended: false,
            });
        }
        if state_id == 2038 {
            return Some(StickyPiston {
                r#facing: Facing::West,
                r#extended: true,
            });
        }
        if state_id == 2042 {
            return Some(StickyPiston {
                r#extended: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2046 {
            return Some(StickyPiston {
                r#extended: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 2039 {
            return Some(StickyPiston {
                r#extended: true,
                r#facing: Facing::Up,
            });
        }
        return None;
    }
}

