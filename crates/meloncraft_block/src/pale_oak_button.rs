use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakButton {
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

impl BlockState for PaleOakButton {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Ceiling && block_state.r#powered == true { return 10657; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#face == Face::Floor { return 10648; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == true { return 10663; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == true && block_state.r#facing == Facing::South { return 10659; }
        if block_state.r#powered == true && block_state.r#face == Face::Floor && block_state.r#facing == Facing::North { return 10641; }
        if block_state.r#powered == true && block_state.r#face == Face::Floor && block_state.r#facing == Facing::West { return 10645; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#face == Face::Wall { return 10653; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#face == Face::Floor { return 10643; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::West { return 10654; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::East && block_state.r#powered == true { return 10655; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::East && block_state.r#powered == false { return 10656; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#face == Face::Floor { return 10646; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == false && block_state.r#facing == Facing::South { return 10660; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Wall && block_state.r#powered == true { return 10651; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#face == Face::Floor { return 10647; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10642; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10650; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10658; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West && block_state.r#powered == true { return 10661; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#face == Face::Wall { return 10649; }
        if block_state.r#facing == Facing::West && block_state.r#face == Face::Ceiling && block_state.r#powered == false { return 10662; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == false { return 10664; }
        if block_state.r#face == Face::Floor && block_state.r#powered == false && block_state.r#facing == Facing::South { return 10644; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Wall && block_state.r#powered == false { return 10652; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10657 {
            return Some(PaleOakButton {
                r#facing: Facing::North,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 10648 {
            return Some(PaleOakButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10663 {
            return Some(PaleOakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10659 {
            return Some(PaleOakButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10641 {
            return Some(PaleOakButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 10645 {
            return Some(PaleOakButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::West,
            });
        }
        if state_id == 10653 {
            return Some(PaleOakButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Wall,
            });
        }
        if state_id == 10643 {
            return Some(PaleOakButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 10654 {
            return Some(PaleOakButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10655 {
            return Some(PaleOakButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10656 {
            return Some(PaleOakButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10646 {
            return Some(PaleOakButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10660 {
            return Some(PaleOakButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10651 {
            return Some(PaleOakButton {
                r#facing: Facing::South,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10647 {
            return Some(PaleOakButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10642 {
            return Some(PaleOakButton {
                r#face: Face::Floor,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10650 {
            return Some(PaleOakButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10658 {
            return Some(PaleOakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10661 {
            return Some(PaleOakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10649 {
            return Some(PaleOakButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Wall,
            });
        }
        if state_id == 10662 {
            return Some(PaleOakButton {
                r#facing: Facing::West,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 10664 {
            return Some(PaleOakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10644 {
            return Some(PaleOakButton {
                r#face: Face::Floor,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10652 {
            return Some(PaleOakButton {
                r#facing: Facing::South,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        return None;
    }
}

