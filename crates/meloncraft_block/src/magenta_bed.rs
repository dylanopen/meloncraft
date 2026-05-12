use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaBed {
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

impl BlockState for MagentaBed {
    fn to_id(self) -> i32 {
        if block_state.r#part == Part::Head && block_state.r#occupied == true && block_state.r#facing == Facing::South { return 1767; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::South && block_state.r#part == Part::Foot { return 1770; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::South && block_state.r#part == Part::Head { return 1769; }
        if block_state.r#facing == Facing::North && block_state.r#part == Part::Head && block_state.r#occupied == true { return 1763; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == false && block_state.r#part == Part::Head { return 1765; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::North && block_state.r#occupied == false { return 1766; }
        if block_state.r#occupied == true && block_state.r#part == Part::Foot && block_state.r#facing == Facing::South { return 1768; }
        if block_state.r#facing == Facing::East && block_state.r#part == Part::Head && block_state.r#occupied == false { return 1777; }
        if block_state.r#facing == Facing::East && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1778; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::West { return 1773; }
        if block_state.r#facing == Facing::East && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1775; }
        if block_state.r#facing == Facing::West && block_state.r#part == Part::Foot && block_state.r#occupied == false { return 1774; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1771; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::East && block_state.r#occupied == true { return 1776; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1772; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1764; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1767 {
            return Some(MagentaBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 1770 {
            return Some(MagentaBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1769 {
            return Some(MagentaBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Head,
            });
        }
        if state_id == 1763 {
            return Some(MagentaBed {
                r#facing: Facing::North,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1765 {
            return Some(MagentaBed {
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1766 {
            return Some(MagentaBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1768 {
            return Some(MagentaBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        if state_id == 1777 {
            return Some(MagentaBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1778 {
            return Some(MagentaBed {
                r#facing: Facing::East,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1773 {
            return Some(MagentaBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 1775 {
            return Some(MagentaBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1774 {
            return Some(MagentaBed {
                r#facing: Facing::West,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1771 {
            return Some(MagentaBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1776 {
            return Some(MagentaBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: true,
            });
        }
        if state_id == 1772 {
            return Some(MagentaBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1764 {
            return Some(MagentaBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        return None;
    }
}

