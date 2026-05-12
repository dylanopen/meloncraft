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
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#face == Face::Wall { return 6703; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#face == Face::Wall { return 6709; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::South && block_state.r#powered == true { return 6696; }
        if block_state.r#face == Face::Wall && block_state.r#powered == true && block_state.r#facing == Facing::West { return 6706; }
        if block_state.r#powered == true && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South { return 6712; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == true { return 6716; }
        if block_state.r#facing == Facing::East && block_state.r#face == Face::Ceiling && block_state.r#powered == false { return 6717; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#face == Face::Floor { return 6698; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#face == Face::Floor { return 6695; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::West { return 6699; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::North && block_state.r#powered == true { return 6694; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#face == Face::Wall { return 6707; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::East { return 6701; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#face == Face::Wall { return 6702; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::South && block_state.r#powered == false { return 6705; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#face == Face::Ceiling { return 6714; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == false && block_state.r#facing == Facing::North { return 6711; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::South && block_state.r#powered == false { return 6697; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North && block_state.r#powered == true { return 6710; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#face == Face::Wall { return 6708; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == true { return 6700; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#face == Face::Wall { return 6704; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South { return 6713; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West { return 6715; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6703 {
            return Some(StoneButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 6709 {
            return Some(StoneButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 6696 {
            return Some(StoneButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 6706 {
            return Some(StoneButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6712 {
            return Some(StoneButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 6716 {
            return Some(StoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 6717 {
            return Some(StoneButton {
                r#facing: Facing::East,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 6698 {
            return Some(StoneButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Floor,
            });
        }
        if state_id == 6695 {
            return Some(StoneButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 6699 {
            return Some(StoneButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::West,
            });
        }
        if state_id == 6694 {
            return Some(StoneButton {
                r#face: Face::Floor,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 6707 {
            return Some(StoneButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 6701 {
            return Some(StoneButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::East,
            });
        }
        if state_id == 6702 {
            return Some(StoneButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 6705 {
            return Some(StoneButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 6714 {
            return Some(StoneButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 6711 {
            return Some(StoneButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6697 {
            return Some(StoneButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 6710 {
            return Some(StoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 6708 {
            return Some(StoneButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 6700 {
            return Some(StoneButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 6704 {
            return Some(StoneButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Wall,
            });
        }
        if state_id == 6713 {
            return Some(StoneButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
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

