use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteBed {
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

impl BlockState for WhiteBed {
    fn to_id(self) -> i32 {
        if block_state.r#occupied == true && block_state.r#facing == Facing::West && block_state.r#part == Part::Foot { return 1740; }
        if block_state.r#occupied == false && block_state.r#part == Part::Head && block_state.r#facing == Facing::West { return 1741; }
        if block_state.r#occupied == true && block_state.r#facing == Facing::East && block_state.r#part == Part::Foot { return 1744; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::North { return 1733; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::North && block_state.r#occupied == true { return 1731; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::East && block_state.r#part == Part::Head { return 1745; }
        if block_state.r#part == Part::Head && block_state.r#occupied == false && block_state.r#facing == Facing::South { return 1737; }
        if block_state.r#occupied == false && block_state.r#part == Part::Foot && block_state.r#facing == Facing::South { return 1738; }
        if block_state.r#part == Part::Foot && block_state.r#occupied == false && block_state.r#facing == Facing::East { return 1746; }
        if block_state.r#facing == Facing::North && block_state.r#occupied == true && block_state.r#part == Part::Foot { return 1732; }
        if block_state.r#occupied == true && block_state.r#part == Part::Head && block_state.r#facing == Facing::East { return 1743; }
        if block_state.r#occupied == false && block_state.r#facing == Facing::North && block_state.r#part == Part::Foot { return 1734; }
        if block_state.r#facing == Facing::West && block_state.r#occupied == true && block_state.r#part == Part::Head { return 1739; }
        if block_state.r#part == Part::Head && block_state.r#facing == Facing::South && block_state.r#occupied == true { return 1735; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::South && block_state.r#occupied == true { return 1736; }
        if block_state.r#part == Part::Foot && block_state.r#facing == Facing::West && block_state.r#occupied == false { return 1742; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1740 {
            return Some(WhiteBed {
                r#occupied: true,
                r#facing: Facing::West,
                r#part: Part::Foot,
            });
        }
        if state_id == 1741 {
            return Some(WhiteBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1744 {
            return Some(WhiteBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Foot,
            });
        }
        if state_id == 1733 {
            return Some(WhiteBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 1731 {
            return Some(WhiteBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        if state_id == 1745 {
            return Some(WhiteBed {
                r#occupied: false,
                r#facing: Facing::East,
                r#part: Part::Head,
            });
        }
        if state_id == 1737 {
            return Some(WhiteBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1738 {
            return Some(WhiteBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        if state_id == 1746 {
            return Some(WhiteBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1732 {
            return Some(WhiteBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1743 {
            return Some(WhiteBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        if state_id == 1734 {
            return Some(WhiteBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        if state_id == 1739 {
            return Some(WhiteBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1735 {
            return Some(WhiteBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1736 {
            return Some(WhiteBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1742 {
            return Some(WhiteBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        return None;
    }
}

