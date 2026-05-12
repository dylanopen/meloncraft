use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakButton {
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

impl BlockState for DarkOakButton {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#face == Face::Wall { return 10630; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Floor && block_state.r#powered == false { return 10618; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10634; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == false && block_state.r#facing == Facing::West { return 10638; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#face == Face::Floor { return 10624; }
        if block_state.r#powered == true && block_state.r#face == Face::Wall && block_state.r#facing == Facing::East { return 10631; }
        if block_state.r#powered == true && block_state.r#face == Face::Wall && block_state.r#facing == Facing::West { return 10629; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#face == Face::Wall { return 10627; }
        if block_state.r#powered == true && block_state.r#face == Face::Floor && block_state.r#facing == Facing::North { return 10617; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::South && block_state.r#powered == false { return 10620; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == false && block_state.r#facing == Facing::South { return 10636; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == true && block_state.r#facing == Facing::East { return 10639; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::West && block_state.r#powered == false { return 10622; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#face == Face::Floor { return 10623; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::South && block_state.r#powered == false { return 10628; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::South && block_state.r#powered == true { return 10619; }
        if block_state.r#facing == Facing::West && block_state.r#face == Face::Floor && block_state.r#powered == true { return 10621; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::North && block_state.r#powered == true { return 10625; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::North { return 10626; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::East { return 10632; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Ceiling && block_state.r#powered == true { return 10635; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == true && block_state.r#facing == Facing::West { return 10637; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == false { return 10640; }
        if block_state.r#powered == true && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North { return 10633; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10630 {
            return Some(DarkOakButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10618 {
            return Some(DarkOakButton {
                r#facing: Facing::North,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 10634 {
            return Some(DarkOakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10638 {
            return Some(DarkOakButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10624 {
            return Some(DarkOakButton {
                r#powered: false,
                r#facing: Facing::East,
                r#face: Face::Floor,
            });
        }
        if state_id == 10631 {
            return Some(DarkOakButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::East,
            });
        }
        if state_id == 10629 {
            return Some(DarkOakButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::West,
            });
        }
        if state_id == 10627 {
            return Some(DarkOakButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Wall,
            });
        }
        if state_id == 10617 {
            return Some(DarkOakButton {
                r#powered: true,
                r#face: Face::Floor,
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
        if state_id == 10636 {
            return Some(DarkOakButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10639 {
            return Some(DarkOakButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10622 {
            return Some(DarkOakButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10623 {
            return Some(DarkOakButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10628 {
            return Some(DarkOakButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10619 {
            return Some(DarkOakButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10621 {
            return Some(DarkOakButton {
                r#facing: Facing::West,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 10625 {
            return Some(DarkOakButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10626 {
            return Some(DarkOakButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 10632 {
            return Some(DarkOakButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::East,
            });
        }
        if state_id == 10635 {
            return Some(DarkOakButton {
                r#facing: Facing::South,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 10637 {
            return Some(DarkOakButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10640 {
            return Some(DarkOakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10633 {
            return Some(DarkOakButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

