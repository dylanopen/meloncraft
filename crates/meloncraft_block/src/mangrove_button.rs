use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveButton {
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

impl BlockState for MangroveButton {
    fn to_id(&self) -> i32 {
        if self.r#face == Face::Ceiling && self.r#facing == Facing::North && self.r#powered == true { return 10681; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::North && self.r#powered == false { return 10682; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Floor { return 10666; }
        if self.r#facing == Facing::South && self.r#face == Face::Floor && self.r#powered == true { return 10667; }
        if self.r#face == Face::Floor && self.r#facing == Facing::East && self.r#powered == true { return 10671; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Floor { return 10672; }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::South { return 10676; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::South && self.r#powered == false { return 10684; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Ceiling { return 10687; }
        if self.r#facing == Facing::West && self.r#face == Face::Wall && self.r#powered == false { return 10678; }
        if self.r#facing == Facing::East && self.r#face == Face::Wall && self.r#powered == true { return 10679; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::South && self.r#powered == true { return 10683; }
        if self.r#facing == Facing::West && self.r#face == Face::Ceiling && self.r#powered == true { return 10685; }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == true { return 10669; }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::North { return 10673; }
        if self.r#face == Face::Wall && self.r#facing == Facing::West && self.r#powered == true { return 10677; }
        if self.r#facing == Facing::East && self.r#face == Face::Wall && self.r#powered == false { return 10680; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::West && self.r#powered == false { return 10686; }
        if self.r#face == Face::Wall && self.r#facing == Facing::South && self.r#powered == true { return 10675; }
        if self.r#powered == true && self.r#face == Face::Floor && self.r#facing == Facing::North { return 10665; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::East && self.r#powered == false { return 10688; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#face == Face::Floor { return 10670; }
        if self.r#facing == Facing::North && self.r#face == Face::Wall && self.r#powered == false { return 10674; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#face == Face::Floor { return 10668; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10681 {
            return Some(MangroveButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10682 {
            return Some(MangroveButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10666 {
            return Some(MangroveButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10667 {
            return Some(MangroveButton {
                r#facing: Facing::South,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 10671 {
            return Some(MangroveButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10672 {
            return Some(MangroveButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10676 {
            return Some(MangroveButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 10684 {
            return Some(MangroveButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10687 {
            return Some(MangroveButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10678 {
            return Some(MangroveButton {
                r#facing: Facing::West,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10679 {
            return Some(MangroveButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10683 {
            return Some(MangroveButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10685 {
            return Some(MangroveButton {
                r#facing: Facing::West,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 10669 {
            return Some(MangroveButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10673 {
            return Some(MangroveButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10677 {
            return Some(MangroveButton {
                r#face: Face::Wall,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10680 {
            return Some(MangroveButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10686 {
            return Some(MangroveButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10675 {
            return Some(MangroveButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10665 {
            return Some(MangroveButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 10688 {
            return Some(MangroveButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10670 {
            return Some(MangroveButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10674 {
            return Some(MangroveButton {
                r#facing: Facing::North,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10668 {
            return Some(MangroveButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        return None;
    }
}

