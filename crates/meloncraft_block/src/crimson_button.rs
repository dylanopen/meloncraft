use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonButton {
    pub r#facing: Facing,
    pub powered: bool,
    pub r#face: Face,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Floor,
    Wall,
    Ceiling,
}

impl BlockState for CrimsonButton {
    fn to_id(self) -> i32 {
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == false { return 21271; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Floor && block_state.r#powered == false { return 21267; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::West { return 21277; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Ceiling && block_state.r#powered == true { return 21280; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == true { return 21286; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#face == Face::Floor { return 21266; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Wall && block_state.r#powered == false { return 21275; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::North && block_state.r#powered == true { return 21272; }
        if block_state.r#facing == Facing::East && block_state.r#face == Face::Wall && block_state.r#powered == false { return 21279; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::East { return 21270; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::North { return 21273; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#face == Face::Wall { return 21276; }
        if block_state.r#powered == true && block_state.r#face == Face::Wall && block_state.r#facing == Facing::East { return 21278; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::North { return 21264; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#face == Face::Floor { return 21265; }
        if block_state.r#face == Face::Floor && block_state.r#powered == false && block_state.r#facing == Facing::West { return 21269; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North { return 21281; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#face == Face::Ceiling { return 21282; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::West { return 21268; }
        if block_state.r#powered == true && block_state.r#face == Face::Wall && block_state.r#facing == Facing::South { return 21274; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South { return 21283; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == true && block_state.r#facing == Facing::West { return 21284; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#face == Face::Ceiling { return 21285; }
        if block_state.r#facing == Facing::East && block_state.r#face == Face::Ceiling && block_state.r#powered == false { return 21287; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21271 {
            return Some(CrimsonButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 21267 {
            return Some(CrimsonButton {
                r#facing: Facing::South,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 21277 {
            return Some(CrimsonButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::West,
            });
        }
        if state_id == 21280 {
            return Some(CrimsonButton {
                r#facing: Facing::North,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 21286 {
            return Some(CrimsonButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 21266 {
            return Some(CrimsonButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 21275 {
            return Some(CrimsonButton {
                r#facing: Facing::South,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 21272 {
            return Some(CrimsonButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 21279 {
            return Some(CrimsonButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 21270 {
            return Some(CrimsonButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 21273 {
            return Some(CrimsonButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 21276 {
            return Some(CrimsonButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 21278 {
            return Some(CrimsonButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::East,
            });
        }
        if state_id == 21264 {
            return Some(CrimsonButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 21265 {
            return Some(CrimsonButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Floor,
            });
        }
        if state_id == 21269 {
            return Some(CrimsonButton {
                r#face: Face::Floor,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21281 {
            return Some(CrimsonButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 21282 {
            return Some(CrimsonButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 21268 {
            return Some(CrimsonButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 21274 {
            return Some(CrimsonButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 21283 {
            return Some(CrimsonButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 21284 {
            return Some(CrimsonButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 21285 {
            return Some(CrimsonButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 21287 {
            return Some(CrimsonButton {
                r#facing: Facing::East,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        return None;
    }
}

