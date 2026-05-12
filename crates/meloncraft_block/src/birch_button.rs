use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchButton {
    pub r#face: Face,
    pub r#facing: Facing,
    pub powered: bool,
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

impl BlockState for BirchButton {
    fn to_id(self) -> i32 {
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == true { return 10527; }
        if block_state.r#powered == true && block_state.r#face == Face::Wall && block_state.r#facing == Facing::West { return 10533; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South { return 10540; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#face == Face::Ceiling { return 10543; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::West { return 10526; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::West && block_state.r#powered == true { return 10525; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::South && block_state.r#powered == true { return 10531; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::West { return 10534; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#face == Face::Floor { return 10521; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#face == Face::Ceiling { return 10544; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::South { return 10532; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10538; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::South { return 10524; }
        if block_state.r#facing == Facing::East && block_state.r#face == Face::Wall && block_state.r#powered == true { return 10535; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#face == Face::Floor { return 10523; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::East { return 10536; }
        if block_state.r#powered == true && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North { return 10537; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#face == Face::Ceiling { return 10539; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#face == Face::Ceiling { return 10541; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#face == Face::Wall { return 10529; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#face == Face::Ceiling { return 10542; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#face == Face::Floor { return 10528; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#face == Face::Wall { return 10530; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#face == Face::Floor { return 10522; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10527 {
            return Some(BirchButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10533 {
            return Some(BirchButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::West,
            });
        }
        if state_id == 10540 {
            return Some(BirchButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 10543 {
            return Some(BirchButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10526 {
            return Some(BirchButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::West,
            });
        }
        if state_id == 10525 {
            return Some(BirchButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10531 {
            return Some(BirchButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10534 {
            return Some(BirchButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10521 {
            return Some(BirchButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10544 {
            return Some(BirchButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10532 {
            return Some(BirchButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10538 {
            return Some(BirchButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10524 {
            return Some(BirchButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::South,
            });
        }
        if state_id == 10535 {
            return Some(BirchButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10523 {
            return Some(BirchButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 10536 {
            return Some(BirchButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::East,
            });
        }
        if state_id == 10537 {
            return Some(BirchButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 10539 {
            return Some(BirchButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10541 {
            return Some(BirchButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10529 {
            return Some(BirchButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Wall,
            });
        }
        if state_id == 10542 {
            return Some(BirchButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10528 {
            return Some(BirchButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10530 {
            return Some(BirchButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Wall,
            });
        }
        if state_id == 10522 {
            return Some(BirchButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Floor,
            });
        }
        return None;
    }
}

