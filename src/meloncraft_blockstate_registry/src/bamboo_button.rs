use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooButton {
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

impl BlockState for BambooButton {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#face == Face::Ceiling && self.r#powered == true { return 10705; }
        if self.r#face == Face::Ceiling && self.r#powered == true && self.r#facing == Facing::West { return 10709; }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::South { return 10700; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#face == Face::Floor { return 10691; }
        if self.r#facing == Facing::North && self.r#face == Face::Floor && self.r#powered == false { return 10690; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#face == Face::Floor { return 10692; }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == false { return 10694; }
        if self.r#facing == Facing::East && self.r#face == Face::Wall && self.r#powered == false { return 10704; }
        if self.r#facing == Facing::North && self.r#face == Face::Wall && self.r#powered == false { return 10698; }
        if self.r#facing == Facing::East && self.r#face == Face::Wall && self.r#powered == true { return 10703; }
        if self.r#facing == Facing::West && self.r#face == Face::Wall && self.r#powered == false { return 10702; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#face == Face::Ceiling { return 10706; }
        if self.r#facing == Facing::West && self.r#face == Face::Wall && self.r#powered == true { return 10701; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Floor { return 10696; }
        if self.r#face == Face::Wall && self.r#facing == Facing::North && self.r#powered == true { return 10697; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#face == Face::Ceiling { return 10707; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#face == Face::Ceiling { return 10708; }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#face == Face::Ceiling { return 10710; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#face == Face::Ceiling { return 10711; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Ceiling { return 10712; }
        if self.r#face == Face::Wall && self.r#facing == Facing::South && self.r#powered == true { return 10699; }
        if self.r#face == Face::Floor && self.r#facing == Facing::East && self.r#powered == true { return 10695; }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == true { return 10693; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#face == Face::Floor { return 10689; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10705 {
            return Some(BambooButton {
                r#facing: Facing::North,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 10709 {
            return Some(BambooButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10700 {
            return Some(BambooButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 10691 {
            return Some(BambooButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 10690 {
            return Some(BambooButton {
                r#facing: Facing::North,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 10692 {
            return Some(BambooButton {
                r#powered: false,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 10694 {
            return Some(BambooButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10704 {
            return Some(BambooButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10698 {
            return Some(BambooButton {
                r#facing: Facing::North,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10703 {
            return Some(BambooButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10702 {
            return Some(BambooButton {
                r#facing: Facing::West,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10706 {
            return Some(BambooButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10701 {
            return Some(BambooButton {
                r#facing: Facing::West,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10696 {
            return Some(BambooButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10697 {
            return Some(BambooButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10707 {
            return Some(BambooButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10708 {
            return Some(BambooButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10710 {
            return Some(BambooButton {
                r#powered: false,
                r#facing: Facing::West,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10711 {
            return Some(BambooButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10712 {
            return Some(BambooButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10699 {
            return Some(BambooButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10695 {
            return Some(BambooButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10693 {
            return Some(BambooButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10689 {
            return Some(BambooButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        return None;
    }
}

