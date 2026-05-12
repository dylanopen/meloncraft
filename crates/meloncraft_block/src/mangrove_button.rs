use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveButton {
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

impl BlockState for MangroveButton {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#face == Face::Wall { return 10675; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::North { return 10666; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == true && block_state.r#facing == Facing::South { return 10683; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::West { return 10669; }
        if block_state.r#facing == Facing::East && block_state.r#face == Face::Wall && block_state.r#powered == true { return 10679; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::West && block_state.r#powered == false { return 10670; }
        if block_state.r#powered == true && block_state.r#face == Face::Wall && block_state.r#facing == Facing::North { return 10673; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Floor && block_state.r#powered == false { return 10668; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::West && block_state.r#powered == true { return 10677; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#face == Face::Ceiling { return 10687; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#face == Face::Ceiling { return 10688; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::East { return 10680; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#face == Face::Floor { return 10667; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West { return 10686; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::West { return 10678; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Floor && block_state.r#powered == true { return 10665; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::South { return 10676; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#face == Face::Ceiling { return 10681; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North { return 10682; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Wall && block_state.r#powered == false { return 10674; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == false && block_state.r#facing == Facing::South { return 10684; }
        if block_state.r#powered == true && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West { return 10685; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == true { return 10671; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == false { return 10672; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10675 {
            return Some(MangroveButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Wall,
            });
        }
        if state_id == 10666 {
            return Some(MangroveButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 10683 {
            return Some(MangroveButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10669 {
            return Some(MangroveButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10679 {
            return Some(MangroveButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10670 {
            return Some(MangroveButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10673 {
            return Some(MangroveButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 10668 {
            return Some(MangroveButton {
                r#facing: Facing::South,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 10677 {
            return Some(MangroveButton {
                r#face: Face::Wall,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10687 {
            return Some(MangroveButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10688 {
            return Some(MangroveButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10680 {
            return Some(MangroveButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::East,
            });
        }
        if state_id == 10667 {
            return Some(MangroveButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 10686 {
            return Some(MangroveButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 10678 {
            return Some(MangroveButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10665 {
            return Some(MangroveButton {
                r#facing: Facing::North,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 10676 {
            return Some(MangroveButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 10681 {
            return Some(MangroveButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10682 {
            return Some(MangroveButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 10674 {
            return Some(MangroveButton {
                r#facing: Facing::North,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10684 {
            return Some(MangroveButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10685 {
            return Some(MangroveButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
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
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        return None;
    }
}

