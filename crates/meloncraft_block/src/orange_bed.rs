use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeBed {
    pub occupied: bool,
    pub r#facing: Facing,
    pub r#part: Part,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Head,
    Foot,
}

impl BlockState for OrangeBed {
    fn to_id(&self) -> i32 {
        if self.r#part == Part::Head && self.r#occupied == true && self.r#facing == Facing::South { return 1751; }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::West { return 1755; }
        if self.r#part == Part::Head && self.r#occupied == true && self.r#facing == Facing::East { return 1759; }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Foot { return 1762; }
        if self.r#facing == Facing::North && self.r#occupied == true && self.r#part == Part::Head { return 1747; }
        if self.r#part == Part::Foot && self.r#facing == Facing::South && self.r#occupied == false { return 1754; }
        if self.r#part == Part::Head && self.r#occupied == false && self.r#facing == Facing::East { return 1761; }
        if self.r#facing == Facing::North && self.r#occupied == false && self.r#part == Part::Head { return 1749; }
        if self.r#facing == Facing::South && self.r#part == Part::Foot && self.r#occupied == true { return 1752; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::North { return 1750; }
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Head { return 1757; }
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Head { return 1753; }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == true { return 1756; }
        if self.r#facing == Facing::North && self.r#part == Part::Foot && self.r#occupied == true { return 1748; }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == false { return 1758; }
        if self.r#occupied == true && self.r#facing == Facing::East && self.r#part == Part::Foot { return 1760; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1751 {
            return Some(OrangeBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 1755 {
            return Some(OrangeBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1759 {
            return Some(OrangeBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 1762 {
            return Some(OrangeBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1747 {
            return Some(OrangeBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1754 {
            return Some(OrangeBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1761 {
            return Some(OrangeBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1749 {
            return Some(OrangeBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1752 {
            return Some(OrangeBed {
                r#facing: Facing::South,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1750 {
            return Some(OrangeBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1757 {
            return Some(OrangeBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1753 {
            return Some(OrangeBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1756 {
            return Some(OrangeBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1748 {
            return Some(OrangeBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1758 {
            return Some(OrangeBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1760 {
            return Some(OrangeBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Foot,
            });
        }
        return None;
    }
}

