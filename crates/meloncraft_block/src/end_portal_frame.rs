use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndPortalFrame {
    pub r#facing: Facing,
    pub eye: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for EndPortalFrame {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#eye == false { return 9275; }
        if block_state.r#facing == Facing::North && block_state.r#eye == false { return 9272; }
        if block_state.r#facing == Facing::South && block_state.r#eye == true { return 9269; }
        if block_state.r#facing == Facing::West && block_state.r#eye == true { return 9270; }
        if block_state.r#facing == Facing::West && block_state.r#eye == false { return 9274; }
        if block_state.r#eye == true && block_state.r#facing == Facing::East { return 9271; }
        if block_state.r#eye == false && block_state.r#facing == Facing::South { return 9273; }
        if block_state.r#eye == true && block_state.r#facing == Facing::North { return 9268; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9275 {
            return Some(EndPortalFrame {
                r#facing: Facing::East,
                r#eye: false,
            });
        }
        if state_id == 9272 {
            return Some(EndPortalFrame {
                r#facing: Facing::North,
                r#eye: false,
            });
        }
        if state_id == 9269 {
            return Some(EndPortalFrame {
                r#facing: Facing::South,
                r#eye: true,
            });
        }
        if state_id == 9270 {
            return Some(EndPortalFrame {
                r#facing: Facing::West,
                r#eye: true,
            });
        }
        if state_id == 9274 {
            return Some(EndPortalFrame {
                r#facing: Facing::West,
                r#eye: false,
            });
        }
        if state_id == 9271 {
            return Some(EndPortalFrame {
                r#eye: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 9273 {
            return Some(EndPortalFrame {
                r#eye: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 9268 {
            return Some(EndPortalFrame {
                r#eye: true,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

