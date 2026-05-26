use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bell {
    pub powered: bool,
    pub r#facing: Facing,
    pub r#attachment: Attachment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attachment {
    Floor,
    Ceiling,
    SingleWall,
    DoubleWall,
}

impl BlockState for Bell {
    fn to_id(&self) -> i32 {
        if self.r#powered == false
            && self.r#attachment == Attachment::Ceiling
            && self.r#facing == Facing::East
        {
            return 20618;
        }
        if self.r#attachment == Attachment::Floor
            && self.r#powered == false
            && self.r#facing == Facing::East
        {
            return 20610;
        }
        if self.r#attachment == Attachment::Ceiling
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 20611;
        }
        if self.r#powered == false
            && self.r#attachment == Attachment::Floor
            && self.r#facing == Facing::West
        {
            return 20608;
        }
        if self.r#attachment == Attachment::SingleWall
            && self.r#facing == Facing::East
            && self.r#powered == true
        {
            return 20625;
        }
        if self.r#attachment == Attachment::Ceiling
            && self.r#powered == true
            && self.r#facing == Facing::West
        {
            return 20615;
        }
        if self.r#attachment == Attachment::SingleWall
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 20619;
        }
        if self.r#facing == Facing::East
            && self.r#attachment == Attachment::Ceiling
            && self.r#powered == true
        {
            return 20617;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#attachment == Attachment::Floor
        {
            return 20607;
        }
        if self.r#attachment == Attachment::Floor
            && self.r#facing == Facing::North
            && self.r#powered == false
        {
            return 20604;
        }
        if self.r#facing == Facing::East
            && self.r#attachment == Attachment::SingleWall
            && self.r#powered == false
        {
            return 20626;
        }
        if self.r#facing == Facing::North
            && self.r#attachment == Attachment::DoubleWall
            && self.r#powered == true
        {
            return 20627;
        }
        if self.r#attachment == Attachment::Ceiling
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 20613;
        }
        if self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#attachment == Attachment::Ceiling
        {
            return 20612;
        }
        if self.r#attachment == Attachment::DoubleWall
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 20632;
        }
        if self.r#attachment == Attachment::Ceiling
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 20614;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#attachment == Attachment::SingleWall
        {
            return 20622;
        }
        if self.r#facing == Facing::North
            && self.r#attachment == Attachment::DoubleWall
            && self.r#powered == false
        {
            return 20628;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#attachment == Attachment::DoubleWall
        {
            return 20629;
        }
        if self.r#attachment == Attachment::Floor
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 20603;
        }
        if self.r#attachment == Attachment::Floor
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 20605;
        }
        if self.r#attachment == Attachment::SingleWall
            && self.r#facing == Facing::West
            && self.r#powered == true
        {
            return 20623;
        }
        if self.r#attachment == Attachment::DoubleWall
            && self.r#facing == Facing::West
            && self.r#powered == true
        {
            return 20631;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#attachment == Attachment::DoubleWall
        {
            return 20633;
        }
        if self.r#attachment == Attachment::DoubleWall
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 20634;
        }
        if self.r#powered == true
            && self.r#attachment == Attachment::Floor
            && self.r#facing == Facing::East
        {
            return 20609;
        }
        if self.r#attachment == Attachment::DoubleWall
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 20630;
        }
        if self.r#attachment == Attachment::SingleWall
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 20624;
        }
        if self.r#powered == false
            && self.r#attachment == Attachment::SingleWall
            && self.r#facing == Facing::North
        {
            return 20620;
        }
        if self.r#attachment == Attachment::Floor
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 20606;
        }
        if self.r#attachment == Attachment::Ceiling
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 20616;
        }
        if self.r#facing == Facing::South
            && self.r#attachment == Attachment::SingleWall
            && self.r#powered == true
        {
            return 20621;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20618 {
            return Some(Bell {
                r#powered: false,
                r#attachment: Attachment::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 20610 {
            return Some(Bell {
                r#attachment: Attachment::Floor,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 20611 {
            return Some(Bell {
                r#attachment: Attachment::Ceiling,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 20608 {
            return Some(Bell {
                r#powered: false,
                r#attachment: Attachment::Floor,
                r#facing: Facing::West,
            });
        }
        if state_id == 20625 {
            return Some(Bell {
                r#attachment: Attachment::SingleWall,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 20615 {
            return Some(Bell {
                r#attachment: Attachment::Ceiling,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 20619 {
            return Some(Bell {
                r#attachment: Attachment::SingleWall,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 20617 {
            return Some(Bell {
                r#facing: Facing::East,
                r#attachment: Attachment::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 20607 {
            return Some(Bell {
                r#facing: Facing::West,
                r#powered: true,
                r#attachment: Attachment::Floor,
            });
        }
        if state_id == 20604 {
            return Some(Bell {
                r#attachment: Attachment::Floor,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 20626 {
            return Some(Bell {
                r#facing: Facing::East,
                r#attachment: Attachment::SingleWall,
                r#powered: false,
            });
        }
        if state_id == 20627 {
            return Some(Bell {
                r#facing: Facing::North,
                r#attachment: Attachment::DoubleWall,
                r#powered: true,
            });
        }
        if state_id == 20613 {
            return Some(Bell {
                r#attachment: Attachment::Ceiling,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 20612 {
            return Some(Bell {
                r#facing: Facing::North,
                r#powered: false,
                r#attachment: Attachment::Ceiling,
            });
        }
        if state_id == 20632 {
            return Some(Bell {
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 20614 {
            return Some(Bell {
                r#attachment: Attachment::Ceiling,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 20622 {
            return Some(Bell {
                r#facing: Facing::South,
                r#powered: false,
                r#attachment: Attachment::SingleWall,
            });
        }
        if state_id == 20628 {
            return Some(Bell {
                r#facing: Facing::North,
                r#attachment: Attachment::DoubleWall,
                r#powered: false,
            });
        }
        if state_id == 20629 {
            return Some(Bell {
                r#powered: true,
                r#facing: Facing::South,
                r#attachment: Attachment::DoubleWall,
            });
        }
        if state_id == 20603 {
            return Some(Bell {
                r#attachment: Attachment::Floor,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 20605 {
            return Some(Bell {
                r#attachment: Attachment::Floor,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 20623 {
            return Some(Bell {
                r#attachment: Attachment::SingleWall,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 20631 {
            return Some(Bell {
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 20633 {
            return Some(Bell {
                r#facing: Facing::East,
                r#powered: true,
                r#attachment: Attachment::DoubleWall,
            });
        }
        if state_id == 20634 {
            return Some(Bell {
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 20609 {
            return Some(Bell {
                r#powered: true,
                r#attachment: Attachment::Floor,
                r#facing: Facing::East,
            });
        }
        if state_id == 20630 {
            return Some(Bell {
                r#attachment: Attachment::DoubleWall,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 20624 {
            return Some(Bell {
                r#attachment: Attachment::SingleWall,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 20620 {
            return Some(Bell {
                r#powered: false,
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
        if state_id == 20616 {
            return Some(Bell {
                r#attachment: Attachment::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 20621 {
            return Some(Bell {
                r#facing: Facing::South,
                r#attachment: Attachment::SingleWall,
                r#powered: true,
            });
        }
        return None;
    }
}
