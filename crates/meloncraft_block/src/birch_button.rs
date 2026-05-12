use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchButton {
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

impl BlockState for BirchButton {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#powered == true && self.r#face == Face::Floor { return 10521; }
        if self.r#facing == Facing::South && self.r#face == Face::Ceiling && self.r#powered == false { return 10540; }
        if self.r#face == Face::Floor && self.r#powered == true && self.r#facing == Facing::South { return 10523; }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::North { return 10529; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::North && self.r#powered == false { return 10538; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Wall { return 10533; }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::South { return 10539; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#face == Face::Ceiling { return 10542; }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::East { return 10543; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Wall { return 10530; }
        if self.r#face == Face::Ceiling && self.r#powered == true && self.r#facing == Facing::North { return 10537; }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::East { return 10544; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#face == Face::Wall { return 10535; }
        if self.r#face == Face::Floor && self.r#powered == true && self.r#facing == Facing::West { return 10525; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Floor { return 10527; }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == false { return 10526; }
        if self.r#face == Face::Wall && self.r#facing == Facing::South && self.r#powered == false { return 10532; }
        if self.r#facing == Facing::West && self.r#face == Face::Ceiling && self.r#powered == true { return 10541; }
        if self.r#facing == Facing::North && self.r#face == Face::Floor && self.r#powered == false { return 10522; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Floor { return 10528; }
        if self.r#facing == Facing::East && self.r#face == Face::Wall && self.r#powered == false { return 10536; }
        if self.r#facing == Facing::South && self.r#face == Face::Wall && self.r#powered == true { return 10531; }
        if self.r#face == Face::Floor && self.r#facing == Facing::South && self.r#powered == false { return 10524; }
        if self.r#face == Face::Wall && self.r#powered == false && self.r#facing == Facing::West { return 10534; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10521 {
            return Some(BirchButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10540 {
            return Some(BirchButton {
                r#facing: Facing::South,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 10523 {
            return Some(BirchButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10529 {
            return Some(BirchButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10538 {
            return Some(BirchButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10533 {
            return Some(BirchButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 10539 {
            return Some(BirchButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 10542 {
            return Some(BirchButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10543 {
            return Some(BirchButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 10530 {
            return Some(BirchButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10537 {
            return Some(BirchButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10544 {
            return Some(BirchButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 10535 {
            return Some(BirchButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 10525 {
            return Some(BirchButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10527 {
            return Some(BirchButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Floor,
            });
        }
        if state_id == 10526 {
            return Some(BirchButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10532 {
            return Some(BirchButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10541 {
            return Some(BirchButton {
                r#facing: Facing::West,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 10522 {
            return Some(BirchButton {
                r#facing: Facing::North,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 10528 {
            return Some(BirchButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10536 {
            return Some(BirchButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10531 {
            return Some(BirchButton {
                r#facing: Facing::South,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10524 {
            return Some(BirchButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10534 {
            return Some(BirchButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

