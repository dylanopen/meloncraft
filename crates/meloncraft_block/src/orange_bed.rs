use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeBed {
    pub r#part: Part,
    pub occupied: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Head,
    Foot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for OrangeBed {
    fn to_id(self) -> i32 {
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::West { return 1755; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::South { return 1753; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::West { return 1757; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::East && block_state.r#part == Part::Foot { return 1760; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::South && block_state.r#occupied == false { return 1754; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1747; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::South && block_state.r#occupied == true { return 1752; }
        if block_state.r#facing == Facing::North && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1750; }
        if block_state.r#facing == Facing::East && block_state.r#part == Part::Head && block_state.r#occupied == true { return 1759; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::North { return 1749; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::South { return 1751; }
        if block_state.r#facing == Facing::North && block_state.r#part == Part::Foot && block_state.r#occupied == true { return 1748; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::West && block_state.r#occupied == true { return 1756; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::East { return 1761; }
        if block_state.r#facing == Facing::East && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1762; }
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1758; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1755 {
            return Some(OrangeBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1753 {
            return Some(OrangeBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1757 {
            return Some(OrangeBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1760 {
            return Some(OrangeBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Foot,
            });
        }
        if state_id == 1754 {
            return Some(OrangeBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1747 {
            return Some(OrangeBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1752 {
            return Some(OrangeBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1750 {
            return Some(OrangeBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1759 {
            return Some(OrangeBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1749 {
            return Some(OrangeBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 1751 {
            return Some(OrangeBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1748 {
            return Some(OrangeBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1756 {
            return Some(OrangeBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1761 {
            return Some(OrangeBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        if state_id == 1762 {
            return Some(OrangeBed {
                r#facing: Facing::East,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1758 {
            return Some(OrangeBed {
                r#facing: Facing::West,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        return None;
    }
}

