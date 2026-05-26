use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaBed {
    pub r#part: Part,
    pub r#facing: Facing,
    pub occupied: bool,
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
    fn to_id(&self) -> i32 {
        if self.r#part == Part::Foot && self.r#facing == Facing::North && self.r#occupied == true {
            return 1764;
        }
        if self.r#part == Part::Foot && self.r#occupied == false && self.r#facing == Facing::North {
            return 1766;
        }
        if self.r#part == Part::Foot && self.r#occupied == false && self.r#facing == Facing::East {
            return 1778;
        }
        if self.r#facing == Facing::West && self.r#occupied == true && self.r#part == Part::Foot {
            return 1772;
        }
        if self.r#occupied == false && self.r#facing == Facing::West && self.r#part == Part::Head {
            return 1773;
        }
        if self.r#occupied == false && self.r#facing == Facing::East && self.r#part == Part::Head {
            return 1777;
        }
        if self.r#part == Part::Head && self.r#facing == Facing::East && self.r#occupied == true {
            return 1775;
        }
        if self.r#facing == Facing::South && self.r#occupied == true && self.r#part == Part::Foot {
            return 1768;
        }
        if self.r#facing == Facing::South && self.r#occupied == true && self.r#part == Part::Head {
            return 1767;
        }
        if self.r#part == Part::Head && self.r#facing == Facing::South && self.r#occupied == false {
            return 1769;
        }
        if self.r#part == Part::Head && self.r#facing == Facing::West && self.r#occupied == true {
            return 1771;
        }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::West {
            return 1774;
        }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::East {
            return 1776;
        }
        if self.r#facing == Facing::North && self.r#part == Part::Head && self.r#occupied == false {
            return 1765;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::South && self.r#occupied == false {
            return 1770;
        }
        if self.r#part == Part::Head && self.r#facing == Facing::North && self.r#occupied == true {
            return 1763;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1764 {
            return Some(MagentaBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        if state_id == 1766 {
            return Some(MagentaBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 1778 {
            return Some(MagentaBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1772 {
            return Some(MagentaBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1773 {
            return Some(MagentaBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1777 {
            return Some(MagentaBed {
                r#occupied: false,
                r#facing: Facing::East,
                r#part: Part::Head,
            });
        }
        if state_id == 1775 {
            return Some(MagentaBed {
                r#part: Part::Head,
                r#facing: Facing::East,
                r#occupied: true,
            });
        }
        if state_id == 1768 {
            return Some(MagentaBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1767 {
            return Some(MagentaBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1769 {
            return Some(MagentaBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1771 {
            return Some(MagentaBed {
                r#part: Part::Head,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1774 {
            return Some(MagentaBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1776 {
            return Some(MagentaBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1765 {
            return Some(MagentaBed {
                r#facing: Facing::North,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1770 {
            return Some(MagentaBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1763 {
            return Some(MagentaBed {
                r#part: Part::Head,
                r#facing: Facing::North,
                r#occupied: true,
            });
        }
        return None;
    }
}
