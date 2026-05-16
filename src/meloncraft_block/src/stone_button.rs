use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneButton {
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

impl BlockState for StoneButton {
    fn to_id(&self) -> i32 {
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == false { return 6699; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Wall { return 6706; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#face == Face::Ceiling { return 6710; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::South && self.r#powered == true { return 6712; }
        if self.r#face == Face::Wall && self.r#facing == Facing::North && self.r#powered == true { return 6702; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::East && self.r#powered == true { return 6716; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#face == Face::Floor { return 6697; }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::South { return 6705; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Floor { return 6700; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Floor { return 6698; }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::North { return 6711; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Floor { return 6695; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#face == Face::Ceiling { return 6714; }
        if self.r#facing == Facing::North && self.r#face == Face::Wall && self.r#powered == false { return 6703; }
        if self.r#face == Face::Wall && self.r#facing == Facing::East && self.r#powered == true { return 6708; }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::West { return 6707; }
        if self.r#face == Face::Wall && self.r#powered == false && self.r#facing == Facing::East { return 6709; }
        if self.r#facing == Facing::East && self.r#face == Face::Floor && self.r#powered == false { return 6701; }
        if self.r#facing == Facing::East && self.r#face == Face::Ceiling && self.r#powered == false { return 6717; }
        if self.r#powered == true && self.r#face == Face::Floor && self.r#facing == Facing::North { return 6694; }
        if self.r#powered == true && self.r#face == Face::Floor && self.r#facing == Facing::South { return 6696; }
        if self.r#face == Face::Wall && self.r#facing == Facing::South && self.r#powered == true { return 6704; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#face == Face::Ceiling { return 6713; }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::West { return 6715; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6699 {
            return Some(StoneButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 6706 {
            return Some(StoneButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 6710 {
            return Some(StoneButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 6712 {
            return Some(StoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 6702 {
            return Some(StoneButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 6716 {
            return Some(StoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 6697 {
            return Some(StoneButton {
                r#powered: false,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 6705 {
            return Some(StoneButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 6700 {
            return Some(StoneButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Floor,
            });
        }
        if state_id == 6698 {
            return Some(StoneButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 6711 {
            return Some(StoneButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 6695 {
            return Some(StoneButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 6714 {
            return Some(StoneButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 6703 {
            return Some(StoneButton {
                r#facing: Facing::North,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 6708 {
            return Some(StoneButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 6707 {
            return Some(StoneButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::West,
            });
        }
        if state_id == 6709 {
            return Some(StoneButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6701 {
            return Some(StoneButton {
                r#facing: Facing::East,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 6717 {
            return Some(StoneButton {
                r#facing: Facing::East,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 6694 {
            return Some(StoneButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 6696 {
            return Some(StoneButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::South,
            });
        }
        if state_id == 6704 {
            return Some(StoneButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 6713 {
            return Some(StoneButton {
                r#powered: false,
                r#facing: Facing::South,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 6715 {
            return Some(StoneButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

