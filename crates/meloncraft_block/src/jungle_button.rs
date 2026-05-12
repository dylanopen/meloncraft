use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleButton {
    pub r#face: Face,
    pub r#facing: Facing,
    pub powered: bool,
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

impl BlockState for JungleButton {
    fn to_id(self) -> i32 {
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::North { return 10545; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::West { return 10549; }
        if block_state.r#face == Face::Wall && block_state.r#powered == true && block_state.r#facing == Facing::West { return 10557; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#face == Face::Wall { return 10558; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10554; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == true { return 10567; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::West { return 10550; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == true { return 10551; }
        if block_state.r#powered == true && block_state.r#face == Face::Wall && block_state.r#facing == Facing::East { return 10559; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#face == Face::Wall { return 10560; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West { return 10566; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Floor && block_state.r#powered == true { return 10547; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::South && block_state.r#powered == false { return 10548; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#face == Face::Ceiling { return 10562; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#face == Face::Wall { return 10555; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East { return 10568; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == true && block_state.r#facing == Facing::West { return 10565; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10546; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::East && block_state.r#powered == false { return 10552; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North && block_state.r#powered == true { return 10561; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South && block_state.r#powered == false { return 10564; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South && block_state.r#powered == true { return 10563; }
        if block_state.r#face == Face::Wall && block_state.r#powered == true && block_state.r#facing == Facing::North { return 10553; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::South { return 10556; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10545 {
            return Some(JungleButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10549 {
            return Some(JungleButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10557 {
            return Some(JungleButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10558 {
            return Some(JungleButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10554 {
            return Some(JungleButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10567 {
            return Some(JungleButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10550 {
            return Some(JungleButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::West,
            });
        }
        if state_id == 10551 {
            return Some(JungleButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10559 {
            return Some(JungleButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::East,
            });
        }
        if state_id == 10560 {
            return Some(JungleButton {
                r#powered: false,
                r#facing: Facing::East,
                r#face: Face::Wall,
            });
        }
        if state_id == 10566 {
            return Some(JungleButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 10547 {
            return Some(JungleButton {
                r#facing: Facing::South,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 10548 {
            return Some(JungleButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10562 {
            return Some(JungleButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10555 {
            return Some(JungleButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 10568 {
            return Some(JungleButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 10565 {
            return Some(JungleButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10546 {
            return Some(JungleButton {
                r#face: Face::Floor,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10552 {
            return Some(JungleButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10561 {
            return Some(JungleButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10564 {
            return Some(JungleButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10563 {
            return Some(JungleButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10553 {
            return Some(JungleButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10556 {
            return Some(JungleButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

