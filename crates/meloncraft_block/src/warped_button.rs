use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedButton {
    pub powered: bool,
    pub r#face: Face,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Floor,
    Wall,
    Ceiling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WarpedButton {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East { return 21310; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#face == Face::Ceiling { return 21309; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South && block_state.r#powered == false { return 21307; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#face == Face::Floor { return 21292; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::West { return 21301; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#face == Face::Wall { return 21300; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#face == Face::Wall { return 21299; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == true { return 21294; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North && block_state.r#powered == true { return 21304; }
        if block_state.r#powered == true && block_state.r#face == Face::Floor && block_state.r#facing == Facing::South { return 21290; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::North && block_state.r#powered == true { return 21296; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::East && block_state.r#powered == false { return 21303; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#face == Face::Wall { return 21297; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == false { return 21295; }
        if block_state.r#face == Face::Wall && block_state.r#powered == true && block_state.r#facing == Facing::South { return 21298; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Ceiling && block_state.r#powered == false { return 21305; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#face == Face::Ceiling { return 21311; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#face == Face::Floor { return 21293; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#face == Face::Floor { return 21289; }
        if block_state.r#face == Face::Wall && block_state.r#powered == true && block_state.r#facing == Facing::East { return 21302; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South && block_state.r#powered == true { return 21306; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Floor && block_state.r#powered == false { return 21291; }
        if block_state.r#powered == true && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West { return 21308; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Floor && block_state.r#powered == true { return 21288; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21310 {
            return Some(WarpedButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 21309 {
            return Some(WarpedButton {
                r#powered: false,
                r#facing: Facing::West,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 21307 {
            return Some(WarpedButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 21292 {
            return Some(WarpedButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 21301 {
            return Some(WarpedButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21300 {
            return Some(WarpedButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Wall,
            });
        }
        if state_id == 21299 {
            return Some(WarpedButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 21294 {
            return Some(WarpedButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 21304 {
            return Some(WarpedButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 21290 {
            return Some(WarpedButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::South,
            });
        }
        if state_id == 21296 {
            return Some(WarpedButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 21303 {
            return Some(WarpedButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 21297 {
            return Some(WarpedButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Wall,
            });
        }
        if state_id == 21295 {
            return Some(WarpedButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 21298 {
            return Some(WarpedButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 21305 {
            return Some(WarpedButton {
                r#facing: Facing::North,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 21311 {
            return Some(WarpedButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 21293 {
            return Some(WarpedButton {
                r#powered: false,
                r#facing: Facing::West,
                r#face: Face::Floor,
            });
        }
        if state_id == 21289 {
            return Some(WarpedButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 21302 {
            return Some(WarpedButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 21306 {
            return Some(WarpedButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 21291 {
            return Some(WarpedButton {
                r#facing: Facing::South,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 21308 {
            return Some(WarpedButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 21288 {
            return Some(WarpedButton {
                r#facing: Facing::North,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        return None;
    }
}

