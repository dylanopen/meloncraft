use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceButton {
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

impl BlockState for SpruceButton {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#face == Face::Floor { return 10497; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10514; }
        if block_state.r#face == Face::Floor && block_state.r#powered == false && block_state.r#facing == Facing::North { return 10498; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::South && block_state.r#powered == false { return 10508; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#face == Face::Ceiling { return 10513; }
        if block_state.r#facing == Facing::West && block_state.r#face == Face::Ceiling && block_state.r#powered == false { return 10518; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == true { return 10503; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#face == Face::Wall { return 10509; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#face == Face::Wall { return 10510; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#face == Face::Ceiling { return 10520; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::West { return 10502; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Wall && block_state.r#powered == true { return 10505; }
        if block_state.r#facing == Facing::East && block_state.r#face == Face::Ceiling && block_state.r#powered == true { return 10519; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::South && block_state.r#powered == false { return 10500; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#face == Face::Floor { return 10501; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#face == Face::Wall { return 10506; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#face == Face::Ceiling { return 10517; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::East { return 10512; }
        if block_state.r#face == Face::Wall && block_state.r#powered == true && block_state.r#facing == Facing::East { return 10511; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#face == Face::Ceiling { return 10516; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#face == Face::Floor { return 10499; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#face == Face::Ceiling { return 10515; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#face == Face::Floor { return 10504; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#face == Face::Wall { return 10507; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10497 {
            return Some(SpruceButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10514 {
            return Some(SpruceButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10498 {
            return Some(SpruceButton {
                r#face: Face::Floor,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10508 {
            return Some(SpruceButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10513 {
            return Some(SpruceButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10518 {
            return Some(SpruceButton {
                r#facing: Facing::West,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 10503 {
            return Some(SpruceButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10509 {
            return Some(SpruceButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Wall,
            });
        }
        if state_id == 10510 {
            return Some(SpruceButton {
                r#powered: false,
                r#facing: Facing::West,
                r#face: Face::Wall,
            });
        }
        if state_id == 10520 {
            return Some(SpruceButton {
                r#powered: false,
                r#facing: Facing::East,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10502 {
            return Some(SpruceButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::West,
            });
        }
        if state_id == 10505 {
            return Some(SpruceButton {
                r#facing: Facing::North,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10519 {
            return Some(SpruceButton {
                r#facing: Facing::East,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 10500 {
            return Some(SpruceButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10501 {
            return Some(SpruceButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10506 {
            return Some(SpruceButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Wall,
            });
        }
        if state_id == 10517 {
            return Some(SpruceButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10512 {
            return Some(SpruceButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 10511 {
            return Some(SpruceButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10516 {
            return Some(SpruceButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10499 {
            return Some(SpruceButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 10515 {
            return Some(SpruceButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10504 {
            return Some(SpruceButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10507 {
            return Some(SpruceButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        return None;
    }
}

