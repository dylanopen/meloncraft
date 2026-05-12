use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bell {
    pub r#attachment: Attachment,
    pub r#facing: Facing,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attachment {
    Floor,
    Ceiling,
    SingleWall,
    DoubleWall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Bell {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#attachment == Attachment::SingleWall && block_state.r#facing == Facing::West { return 20623; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#attachment == Attachment::Ceiling { return 20617; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#attachment == Attachment::Ceiling { return 20612; }
        if block_state.r#facing == Facing::East && block_state.r#attachment == Attachment::Floor && block_state.r#powered == true { return 20609; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#attachment == Attachment::Floor { return 20604; }
        if block_state.r#facing == Facing::South && block_state.r#attachment == Attachment::SingleWall && block_state.r#powered == false { return 20622; }
        if block_state.r#powered == false && block_state.r#attachment == Attachment::Floor && block_state.r#facing == Facing::West { return 20608; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#attachment == Attachment::Floor { return 20610; }
        if block_state.r#attachment == Attachment::Floor && block_state.r#facing == Facing::North && block_state.r#powered == true { return 20603; }
        if block_state.r#attachment == Attachment::SingleWall && block_state.r#facing == Facing::West && block_state.r#powered == false { return 20624; }
        if block_state.r#attachment == Attachment::DoubleWall && block_state.r#facing == Facing::South && block_state.r#powered == false { return 20630; }
        if block_state.r#attachment == Attachment::Ceiling && block_state.r#facing == Facing::South && block_state.r#powered == true { return 20613; }
        if block_state.r#attachment == Attachment::DoubleWall && block_state.r#facing == Facing::South && block_state.r#powered == true { return 20629; }
        if block_state.r#attachment == Attachment::Ceiling && block_state.r#facing == Facing::North && block_state.r#powered == true { return 20611; }
        if block_state.r#powered == false && block_state.r#attachment == Attachment::Ceiling && block_state.r#facing == Facing::South { return 20614; }
        if block_state.r#powered == false && block_state.r#attachment == Attachment::SingleWall && block_state.r#facing == Facing::East { return 20626; }
        if block_state.r#powered == false && block_state.r#attachment == Attachment::DoubleWall && block_state.r#facing == Facing::East { return 20634; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#attachment == Attachment::DoubleWall { return 20627; }
        if block_state.r#powered == true && block_state.r#attachment == Attachment::SingleWall && block_state.r#facing == Facing::North { return 20619; }
        if block_state.r#attachment == Attachment::Floor && block_state.r#powered == false && block_state.r#facing == Facing::South { return 20606; }
        if block_state.r#attachment == Attachment::Floor && block_state.r#facing == Facing::South && block_state.r#powered == true { return 20605; }
        if block_state.r#attachment == Attachment::SingleWall && block_state.r#powered == false && block_state.r#facing == Facing::North { return 20620; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#attachment == Attachment::SingleWall { return 20625; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#attachment == Attachment::SingleWall { return 20621; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#attachment == Attachment::Floor { return 20607; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#attachment == Attachment::Ceiling { return 20618; }
        if block_state.r#powered == false && block_state.r#attachment == Attachment::DoubleWall && block_state.r#facing == Facing::West { return 20632; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#attachment == Attachment::DoubleWall { return 20631; }
        if block_state.r#attachment == Attachment::DoubleWall && block_state.r#facing == Facing::East && block_state.r#powered == true { return 20633; }
        if block_state.r#attachment == Attachment::Ceiling && block_state.r#facing == Facing::West && block_state.r#powered == false { return 20616; }
        if block_state.r#attachment == Attachment::DoubleWall && block_state.r#facing == Facing::North && block_state.r#powered == false { return 20628; }
        if block_state.r#attachment == Attachment::Ceiling && block_state.r#powered == true && block_state.r#facing == Facing::West { return 20615; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20623 {
            return Some(Bell {
                r#powered: true,
                r#attachment: Attachment::SingleWall,
                r#facing: Facing::West,
            });
        }
        if state_id == 20617 {
            return Some(Bell {
                r#facing: Facing::East,
                r#powered: true,
                r#attachment: Attachment::Ceiling,
            });
        }
        if state_id == 20612 {
            return Some(Bell {
                r#powered: false,
                r#facing: Facing::North,
                r#attachment: Attachment::Ceiling,
            });
        }
        if state_id == 20609 {
            return Some(Bell {
                r#facing: Facing::East,
                r#attachment: Attachment::Floor,
                r#powered: true,
            });
        }
        if state_id == 20604 {
            return Some(Bell {
                r#powered: false,
                r#facing: Facing::North,
                r#attachment: Attachment::Floor,
            });
        }
        if state_id == 20622 {
            return Some(Bell {
                r#facing: Facing::South,
                r#attachment: Attachment::SingleWall,
                r#powered: false,
            });
        }
        if state_id == 20608 {
            return Some(Bell {
                r#powered: false,
                r#attachment: Attachment::Floor,
                r#facing: Facing::West,
            });
        }
        if state_id == 20610 {
            return Some(Bell {
                r#facing: Facing::East,
                r#powered: false,
                r#attachment: Attachment::Floor,
            });
        }
        if state_id == 20603 {
            return Some(Bell {
                r#attachment: Attachment::Floor,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 20624 {
            return Some(Bell {
                r#attachment: Attachment::SingleWall,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 20630 {
            return Some(Bell {
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 20613 {
            return Some(Bell {
                r#attachment: Attachment::Ceiling,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 20629 {
            return Some(Bell {
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 20611 {
            return Some(Bell {
                r#attachment: Attachment::Ceiling,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 20614 {
            return Some(Bell {
                r#powered: false,
                r#attachment: Attachment::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 20626 {
            return Some(Bell {
                r#powered: false,
                r#attachment: Attachment::SingleWall,
                r#facing: Facing::East,
            });
        }
        if state_id == 20634 {
            return Some(Bell {
                r#powered: false,
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::East,
            });
        }
        if state_id == 20627 {
            return Some(Bell {
                r#powered: true,
                r#facing: Facing::North,
                r#attachment: Attachment::DoubleWall,
            });
        }
        if state_id == 20619 {
            return Some(Bell {
                r#powered: true,
                r#attachment: Attachment::SingleWall,
                r#facing: Facing::North,
            });
        }
        if state_id == 20606 {
            return Some(Bell {
                r#attachment: Attachment::Floor,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 20605 {
            return Some(Bell {
                r#attachment: Attachment::Floor,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 20620 {
            return Some(Bell {
                r#attachment: Attachment::SingleWall,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 20625 {
            return Some(Bell {
                r#facing: Facing::East,
                r#powered: true,
                r#attachment: Attachment::SingleWall,
            });
        }
        if state_id == 20621 {
            return Some(Bell {
                r#facing: Facing::South,
                r#powered: true,
                r#attachment: Attachment::SingleWall,
            });
        }
        if state_id == 20607 {
            return Some(Bell {
                r#facing: Facing::West,
                r#powered: true,
                r#attachment: Attachment::Floor,
            });
        }
        if state_id == 20618 {
            return Some(Bell {
                r#facing: Facing::East,
                r#powered: false,
                r#attachment: Attachment::Ceiling,
            });
        }
        if state_id == 20632 {
            return Some(Bell {
                r#powered: false,
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::West,
            });
        }
        if state_id == 20631 {
            return Some(Bell {
                r#powered: true,
                r#facing: Facing::West,
                r#attachment: Attachment::DoubleWall,
            });
        }
        if state_id == 20633 {
            return Some(Bell {
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 20616 {
            return Some(Bell {
                r#attachment: Attachment::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 20628 {
            return Some(Bell {
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 20615 {
            return Some(Bell {
                r#attachment: Attachment::Ceiling,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

