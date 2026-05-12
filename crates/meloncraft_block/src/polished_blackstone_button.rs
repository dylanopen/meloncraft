use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneButton {
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

impl BlockState for PolishedBlackstoneButton {
    fn to_id(self) -> i32 {
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::North && block_state.r#powered == true { return 22551; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#face == Face::Floor { return 22546; }
        if block_state.r#face == Face::Wall && block_state.r#powered == true && block_state.r#facing == Facing::West { return 22555; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::North { return 22552; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Floor && block_state.r#powered == true { return 22545; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West && block_state.r#powered == false { return 22564; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West && block_state.r#powered == true { return 22563; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == false { return 22566; }
        if block_state.r#powered == true && block_state.r#face == Face::Wall && block_state.r#facing == Facing::South { return 22553; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::West { return 22556; }
        if block_state.r#face == Face::Ceiling && block_state.r#powered == true && block_state.r#facing == Facing::South { return 22561; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::West && block_state.r#powered == true { return 22547; }
        if block_state.r#facing == Facing::North && block_state.r#face == Face::Ceiling && block_state.r#powered == false { return 22560; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::North { return 22543; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#face == Face::Floor { return 22550; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::South { return 22554; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East && block_state.r#powered == true { return 22565; }
        if block_state.r#powered == true && block_state.r#face == Face::Floor && block_state.r#facing == Facing::East { return 22549; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#face == Face::Wall { return 22557; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::East { return 22558; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#face == Face::Floor { return 22544; }
        if block_state.r#powered == true && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North { return 22559; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South { return 22562; }
        if block_state.r#facing == Facing::West && block_state.r#face == Face::Floor && block_state.r#powered == false { return 22548; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22551 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 22546 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 22555 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 22552 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 22545 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::South,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 22564 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 22563 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 22566 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 22553 {
            return Some(PolishedBlackstoneButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 22556 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 22561 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 22547 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 22560 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::North,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 22543 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 22550 {
            return Some(PolishedBlackstoneButton {
                r#powered: false,
                r#facing: Facing::East,
                r#face: Face::Floor,
            });
        }
        if state_id == 22554 {
            return Some(PolishedBlackstoneButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 22565 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 22549 {
            return Some(PolishedBlackstoneButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::East,
            });
        }
        if state_id == 22557 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 22558 {
            return Some(PolishedBlackstoneButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::East,
            });
        }
        if state_id == 22544 {
            return Some(PolishedBlackstoneButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Floor,
            });
        }
        if state_id == 22559 {
            return Some(PolishedBlackstoneButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 22562 {
            return Some(PolishedBlackstoneButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 22548 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::West,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        return None;
    }
}

