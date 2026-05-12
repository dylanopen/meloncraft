use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakButton {
    pub powered: bool,
    pub r#facing: Facing,
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

impl BlockState for DarkOakButton {
    fn to_id(&self) -> i32 {
        if self.r#powered == true && self.r#facing == Facing::North && self.r#face == Face::Floor { return 10617; }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::East { return 10640; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#face == Face::Floor { return 10623; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Wall { return 10631; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Floor { return 10618; }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == true { return 10621; }
        if self.r#facing == Facing::North && self.r#face == Face::Wall && self.r#powered == true { return 10625; }
        if self.r#face == Face::Wall && self.r#powered == false && self.r#facing == Facing::South { return 10628; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Ceiling { return 10639; }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::South { return 10636; }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::North { return 10634; }
        if self.r#face == Face::Floor && self.r#facing == Facing::South && self.r#powered == false { return 10620; }
        if self.r#face == Face::Floor && self.r#powered == false && self.r#facing == Facing::East { return 10624; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#face == Face::Wall { return 10626; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#face == Face::Ceiling { return 10633; }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::West { return 10637; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Wall { return 10629; }
        if self.r#face == Face::Wall && self.r#facing == Facing::East && self.r#powered == false { return 10632; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#face == Face::Floor { return 10619; }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == false { return 10622; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#face == Face::Wall { return 10627; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::South && self.r#powered == true { return 10635; }
        if self.r#facing == Facing::West && self.r#face == Face::Ceiling && self.r#powered == false { return 10638; }
        if self.r#face == Face::Wall && self.r#facing == Facing::West && self.r#powered == false { return 10630; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10617 {
            return Some(DarkOakButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Floor,
            });
        }
        if state_id == 10640 {
            return Some(DarkOakButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 10623 {
            return Some(DarkOakButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10631 {
            return Some(DarkOakButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Wall,
            });
        }
        if state_id == 10618 {
            return Some(DarkOakButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10621 {
            return Some(DarkOakButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10625 {
            return Some(DarkOakButton {
                r#facing: Facing::North,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10628 {
            return Some(DarkOakButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10639 {
            return Some(DarkOakButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10636 {
            return Some(DarkOakButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 10634 {
            return Some(DarkOakButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 10620 {
            return Some(DarkOakButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10624 {
            return Some(DarkOakButton {
                r#face: Face::Floor,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 10626 {
            return Some(DarkOakButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Wall,
            });
        }
        if state_id == 10633 {
            return Some(DarkOakButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10637 {
            return Some(DarkOakButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 10629 {
            return Some(DarkOakButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 10632 {
            return Some(DarkOakButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10619 {
            return Some(DarkOakButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10622 {
            return Some(DarkOakButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10627 {
            return Some(DarkOakButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Wall,
            });
        }
        if state_id == 10635 {
            return Some(DarkOakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10638 {
            return Some(DarkOakButton {
                r#facing: Facing::West,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 10630 {
            return Some(DarkOakButton {
                r#face: Face::Wall,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        return None;
    }
}

