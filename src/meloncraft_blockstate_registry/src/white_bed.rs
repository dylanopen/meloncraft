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
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Foot {
            return 1738;
        }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Head {
            return 1745;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == true {
            return 1740;
        }
        if self.r#part == Part::Head && self.r#facing == Facing::South && self.r#occupied == false {
            return 1737;
        }
        if self.r#occupied == false && self.r#part == Part::Head && self.r#facing == Facing::North {
            return 1733;
        }
        if self.r#part == Part::Head && self.r#occupied == true && self.r#facing == Facing::North {
            return 1731;
        }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::West {
            return 1742;
        }
        if self.r#occupied == false && self.r#facing == Facing::West && self.r#part == Part::Head {
            return 1741;
        }
        if self.r#facing == Facing::East && self.r#occupied == true && self.r#part == Part::Foot {
            return 1744;
        }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::West {
            return 1739;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::North && self.r#occupied == true {
            return 1732;
        }
        if self.r#facing == Facing::South && self.r#part == Part::Head && self.r#occupied == true {
            return 1735;
        }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::East {
            return 1746;
        }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::East {
            return 1743;
        }
        if self.r#facing == Facing::North && self.r#occupied == false && self.r#part == Part::Foot {
            return 1734;
        }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::South {
            return 1736;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1738 {
            return Some(WhiteBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1745 {
            return Some(WhiteBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1740 {
            return Some(WhiteBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1737 {
            return Some(WhiteBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1733 {
            return Some(WhiteBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        if state_id == 1731 {
            return Some(WhiteBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 1742 {
            return Some(WhiteBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1741 {
            return Some(WhiteBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1744 {
            return Some(WhiteBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1739 {
            return Some(WhiteBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1732 {
            return Some(WhiteBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        if state_id == 1735 {
            return Some(WhiteBed {
                r#facing: Facing::South,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1746 {
            return Some(WhiteBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::East,
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
                r#facing: Facing::North,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1736 {
            return Some(WhiteBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
