use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeBed {
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

impl BlockState for LimeBed {
    fn to_id(&self) -> i32 {
        if self.r#part == Part::Head && self.r#occupied == true && self.r#facing == Facing::West {
            return 1819;
        }
        if self.r#occupied == false && self.r#part == Part::Head && self.r#facing == Facing::West {
            return 1821;
        }
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Head {
            return 1817;
        }
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Foot {
            return 1822;
        }
        if self.r#occupied == false && self.r#facing == Facing::South && self.r#part == Part::Foot {
            return 1818;
        }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Foot {
            return 1826;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == true {
            return 1820;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::South && self.r#occupied == true {
            return 1816;
        }
        if self.r#occupied == true && self.r#facing == Facing::East && self.r#part == Part::Head {
            return 1823;
        }
        if self.r#occupied == false && self.r#part == Part::Head && self.r#facing == Facing::North {
            return 1813;
        }
        if self.r#facing == Facing::North && self.r#part == Part::Foot && self.r#occupied == false {
            return 1814;
        }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Head {
            return 1825;
        }
        if self.r#facing == Facing::North && self.r#occupied == true && self.r#part == Part::Head {
            return 1811;
        }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::South {
            return 1815;
        }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::North {
            return 1812;
        }
        if self.r#part == Part::Foot && self.r#occupied == true && self.r#facing == Facing::East {
            return 1824;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1819 {
            return Some(LimeBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 1821 {
            return Some(LimeBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::West,
            });
        }
        if state_id == 1817 {
            return Some(LimeBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1822 {
            return Some(LimeBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1818 {
            return Some(LimeBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1826 {
            return Some(LimeBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1820 {
            return Some(LimeBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1816 {
            return Some(LimeBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: true,
            });
        }
        if state_id == 1823 {
            return Some(LimeBed {
                r#occupied: true,
                r#facing: Facing::East,
                r#part: Part::Head,
            });
        }
        if state_id == 1813 {
            return Some(LimeBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        if state_id == 1814 {
            return Some(LimeBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: false,
            });
        }
        if state_id == 1825 {
            return Some(LimeBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1811 {
            return Some(LimeBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1815 {
            return Some(LimeBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1812 {
            return Some(LimeBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1824 {
            return Some(LimeBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
