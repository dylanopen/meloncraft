use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneButton {
    pub r#facing: Facing,
    pub r#face: Face,
    pub powered: bool,
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

impl BlockState for PolishedBlackstoneButton {
    fn to_id(&self) -> i32 {
        if self.r#face == Face::Wall && self.r#facing == Facing::North && self.r#powered == true { return 22551; }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == false { return 22548; }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#face == Face::Ceiling { return 22559; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#face == Face::Ceiling { return 22565; }
        if self.r#facing == Facing::South && self.r#face == Face::Ceiling && self.r#powered == false { return 22562; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#face == Face::Floor { return 22550; }
        if self.r#powered == true && self.r#face == Face::Wall && self.r#facing == Facing::South { return 22553; }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::West { return 22563; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::East && self.r#powered == false { return 22566; }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::West { return 22556; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::North && self.r#powered == false { return 22560; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#face == Face::Floor { return 22545; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Floor { return 22547; }
        if self.r#facing == Facing::West && self.r#face == Face::Wall && self.r#powered == true { return 22555; }
        if self.r#face == Face::Floor && self.r#powered == true && self.r#facing == Facing::North { return 22543; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#face == Face::Floor { return 22546; }
        if self.r#facing == Facing::East && self.r#face == Face::Wall && self.r#powered == false { return 22558; }
        if self.r#face == Face::Wall && self.r#facing == Facing::North && self.r#powered == false { return 22552; }
        if self.r#face == Face::Floor && self.r#facing == Facing::East && self.r#powered == true { return 22549; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#face == Face::Wall { return 22554; }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::East { return 22557; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#face == Face::Ceiling { return 22561; }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::West && self.r#powered == false { return 22564; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#face == Face::Floor { return 22544; }
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
        if state_id == 22548 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 22559 {
            return Some(PolishedBlackstoneButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 22565 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 22562 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::South,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 22550 {
            return Some(PolishedBlackstoneButton {
                r#powered: false,
                r#facing: Facing::East,
                r#face: Face::Floor,
            });
        }
        if state_id == 22553 {
            return Some(PolishedBlackstoneButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 22563 {
            return Some(PolishedBlackstoneButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 22566 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 22556 {
            return Some(PolishedBlackstoneButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::West,
            });
        }
        if state_id == 22560 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 22545 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 22547 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 22555 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::West,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 22543 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 22546 {
            return Some(PolishedBlackstoneButton {
                r#powered: false,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 22558 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 22552 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 22549 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 22554 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 22557 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 22561 {
            return Some(PolishedBlackstoneButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 22564 {
            return Some(PolishedBlackstoneButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 22544 {
            return Some(PolishedBlackstoneButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Floor,
            });
        }
        return None;
    }
}

