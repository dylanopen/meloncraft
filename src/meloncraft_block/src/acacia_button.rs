use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaButton {
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

impl BlockState for AcaciaButton {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#powered == true && self.r#face == Face::Floor { return 10571; }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::North { return 10586; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#face == Face::Floor { return 10572; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::West && self.r#powered == true { return 10589; }
        if self.r#face == Face::Floor && self.r#powered == true && self.r#facing == Facing::East { return 10575; }
        if self.r#facing == Facing::North && self.r#face == Face::Floor && self.r#powered == true { return 10569; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#face == Face::Floor { return 10574; }
        if self.r#face == Face::Wall && self.r#facing == Facing::South && self.r#powered == true { return 10579; }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::North { return 10578; }
        if self.r#facing == Facing::South && self.r#face == Face::Wall && self.r#powered == false { return 10580; }
        if self.r#powered == true && self.r#face == Face::Wall && self.r#facing == Facing::North { return 10577; }
        if self.r#face == Face::Floor && self.r#facing == Facing::North && self.r#powered == false { return 10570; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#face == Face::Wall { return 10583; }
        if self.r#face == Face::Wall && self.r#facing == Facing::East && self.r#powered == false { return 10584; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::South && self.r#powered == true { return 10587; }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::North { return 10585; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#face == Face::Floor { return 10576; }
        if self.r#face == Face::Ceiling && self.r#powered == false && self.r#facing == Facing::East { return 10592; }
        if self.r#facing == Facing::West && self.r#face == Face::Wall && self.r#powered == true { return 10581; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::West && self.r#powered == false { return 10590; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#face == Face::Wall { return 10582; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#face == Face::Floor { return 10573; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#face == Face::Ceiling { return 10588; }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::East { return 10591; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10571 {
            return Some(AcaciaButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10586 {
            return Some(AcaciaButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 10572 {
            return Some(AcaciaButton {
                r#powered: false,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 10589 {
            return Some(AcaciaButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10575 {
            return Some(AcaciaButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10569 {
            return Some(AcaciaButton {
                r#facing: Facing::North,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 10574 {
            return Some(AcaciaButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10579 {
            return Some(AcaciaButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10578 {
            return Some(AcaciaButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 10580 {
            return Some(AcaciaButton {
                r#facing: Facing::South,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10577 {
            return Some(AcaciaButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 10570 {
            return Some(AcaciaButton {
                r#face: Face::Floor,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10583 {
            return Some(AcaciaButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 10584 {
            return Some(AcaciaButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10587 {
            return Some(AcaciaButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10585 {
            return Some(AcaciaButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 10576 {
            return Some(AcaciaButton {
                r#powered: false,
                r#facing: Facing::East,
                r#face: Face::Floor,
            });
        }
        if state_id == 10592 {
            return Some(AcaciaButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 10581 {
            return Some(AcaciaButton {
                r#facing: Facing::West,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10590 {
            return Some(AcaciaButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10582 {
            return Some(AcaciaButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10573 {
            return Some(AcaciaButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Floor,
            });
        }
        if state_id == 10588 {
            return Some(AcaciaButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10591 {
            return Some(AcaciaButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

