use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grindstone {
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

impl BlockState for Grindstone {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#face == Face::Wall { return 20577; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Ceiling { return 20578; }
        if block_state.r#facing == Facing::West && block_state.r#face == Face::Ceiling { return 20580; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East { return 20581; }
        if block_state.r#facing == Facing::West && block_state.r#face == Face::Wall { return 20576; }
        if block_state.r#facing == Facing::West && block_state.r#face == Face::Floor { return 20572; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Floor { return 20570; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::North { return 20574; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::South { return 20571; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Ceiling { return 20579; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Wall { return 20575; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East { return 20573; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20577 {
            return Some(Grindstone {
                r#facing: Facing::East,
                r#face: Face::Wall,
            });
        }
        if state_id == 20578 {
            return Some(Grindstone {
                r#facing: Facing::North,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 20580 {
            return Some(Grindstone {
                r#facing: Facing::West,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 20581 {
            return Some(Grindstone {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 20576 {
            return Some(Grindstone {
                r#facing: Facing::West,
                r#face: Face::Wall,
            });
        }
        if state_id == 20572 {
            return Some(Grindstone {
                r#facing: Facing::West,
                r#face: Face::Floor,
            });
        }
        if state_id == 20570 {
            return Some(Grindstone {
                r#facing: Facing::North,
                r#face: Face::Floor,
            });
        }
        if state_id == 20574 {
            return Some(Grindstone {
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 20571 {
            return Some(Grindstone {
                r#face: Face::Floor,
                r#facing: Facing::South,
            });
        }
        if state_id == 20579 {
            return Some(Grindstone {
                r#facing: Facing::South,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 20575 {
            return Some(Grindstone {
                r#facing: Facing::South,
                r#face: Face::Wall,
            });
        }
        if state_id == 20573 {
            return Some(Grindstone {
                r#face: Face::Floor,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

