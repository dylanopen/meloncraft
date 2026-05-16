use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedButton {
    pub r#face: Face,
    pub powered: bool,
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
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#face == Face::Floor && self.r#powered == false { return 21295; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#face == Face::Wall { return 21298; }
        if self.r#facing == Facing::North && self.r#face == Face::Ceiling && self.r#powered == true { return 21304; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::South && self.r#powered == true { return 21306; }
        if self.r#facing == Facing::North && self.r#face == Face::Wall && self.r#powered == false { return 21297; }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::South { return 21307; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Ceiling { return 21311; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#face == Face::Wall { return 21299; }
        if self.r#facing == Facing::East && self.r#face == Face::Wall && self.r#powered == true { return 21302; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::West && self.r#powered == false { return 21309; }
        if self.r#facing == Facing::West && self.r#face == Face::Ceiling && self.r#powered == true { return 21308; }
        if self.r#powered == false && self.r#face == Face::Floor && self.r#facing == Facing::North { return 21289; }
        if self.r#powered == true && self.r#face == Face::Floor && self.r#facing == Facing::South { return 21290; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::North && self.r#powered == false { return 21305; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Ceiling { return 21310; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Wall { return 21303; }
        if self.r#facing == Facing::West && self.r#face == Face::Wall && self.r#powered == true { return 21300; }
        if self.r#face == Face::Wall && self.r#facing == Facing::West && self.r#powered == false { return 21301; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#face == Face::Floor { return 21288; }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == false { return 21293; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#face == Face::Floor { return 21291; }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#face == Face::Wall { return 21296; }
        if self.r#powered == true && self.r#face == Face::Floor && self.r#facing == Facing::East { return 21294; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Floor { return 21292; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21295 {
            return Some(WarpedButton {
                r#facing: Facing::East,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 21298 {
            return Some(WarpedButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 21304 {
            return Some(WarpedButton {
                r#facing: Facing::North,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 21306 {
            return Some(WarpedButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 21297 {
            return Some(WarpedButton {
                r#facing: Facing::North,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 21307 {
            return Some(WarpedButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 21311 {
            return Some(WarpedButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 21299 {
            return Some(WarpedButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 21302 {
            return Some(WarpedButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 21309 {
            return Some(WarpedButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 21308 {
            return Some(WarpedButton {
                r#facing: Facing::West,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 21289 {
            return Some(WarpedButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 21290 {
            return Some(WarpedButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::South,
            });
        }
        if state_id == 21305 {
            return Some(WarpedButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 21310 {
            return Some(WarpedButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 21303 {
            return Some(WarpedButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 21300 {
            return Some(WarpedButton {
                r#facing: Facing::West,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 21301 {
            return Some(WarpedButton {
                r#face: Face::Wall,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 21288 {
            return Some(WarpedButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 21293 {
            return Some(WarpedButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 21291 {
            return Some(WarpedButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 21296 {
            return Some(WarpedButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Wall,
            });
        }
        if state_id == 21294 {
            return Some(WarpedButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::East,
            });
        }
        if state_id == 21292 {
            return Some(WarpedButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        return None;
    }
}

