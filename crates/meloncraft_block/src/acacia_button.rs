use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaButton {
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

impl BlockState for AcaciaButton {
    fn to_id(self) -> i32 {
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::South && block_state.r#powered == true { return 10571; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::North { return 10570; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#face == Face::Floor { return 10572; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#face == Face::Wall { return 10582; }
        if block_state.r#powered == true && block_state.r#face == Face::Floor && block_state.r#facing == Facing::North { return 10569; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::East { return 10576; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#face == Face::Wall { return 10581; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#face == Face::Wall { return 10584; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#face == Face::Ceiling { return 10587; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::West && block_state.r#powered == true { return 10573; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Wall && block_state.r#powered == false { return 10580; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#face == Face::Wall { return 10577; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::East && block_state.r#powered == true { return 10583; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#face == Face::Ceiling { return 10586; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == true && block_state.r#facing == Facing::West { return 10589; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::North { return 10578; }
        if block_state.r#facing == Facing::West && block_state.r#face == Face::Floor && block_state.r#powered == false { return 10574; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#face == Face::Ceiling { return 10585; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Ceiling && block_state.r#powered == false { return 10588; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::South && block_state.r#powered == true { return 10579; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == true { return 10591; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West { return 10590; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == false { return 10592; }
        if block_state.r#facing == Facing::East && block_state.r#face == Face::Floor && block_state.r#powered == true { return 10575; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10571 {
            return Some(AcaciaButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10570 {
            return Some(AcaciaButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 10572 {
            return Some(AcaciaButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10582 {
            return Some(AcaciaButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10569 {
            return Some(AcaciaButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 10576 {
            return Some(AcaciaButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::East,
            });
        }
        if state_id == 10581 {
            return Some(AcaciaButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 10584 {
            return Some(AcaciaButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10587 {
            return Some(AcaciaButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10573 {
            return Some(AcaciaButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10580 {
            return Some(AcaciaButton {
                r#facing: Facing::South,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10577 {
            return Some(AcaciaButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 10583 {
            return Some(AcaciaButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10586 {
            return Some(AcaciaButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10589 {
            return Some(AcaciaButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10578 {
            return Some(AcaciaButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 10574 {
            return Some(AcaciaButton {
                r#facing: Facing::West,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 10585 {
            return Some(AcaciaButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10588 {
            return Some(AcaciaButton {
                r#facing: Facing::South,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 10579 {
            return Some(AcaciaButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10591 {
            return Some(AcaciaButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10590 {
            return Some(AcaciaButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 10592 {
            return Some(AcaciaButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10575 {
            return Some(AcaciaButton {
                r#facing: Facing::East,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        return None;
    }
}

