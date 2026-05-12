use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleBed {
    pub r#facing: Facing,
    pub occupied: bool,
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

impl BlockState for PurpleBed {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Head { return 1901; }
        if self.r#part == Part::Head && self.r#occupied == false && self.r#facing == Facing::South { return 1897; }
        if self.r#facing == Facing::North && self.r#occupied == true && self.r#part == Part::Foot { return 1892; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::West { return 1900; }
        if self.r#part == Part::Foot && self.r#occupied == false && self.r#facing == Facing::South { return 1898; }
        if self.r#part == Part::Head && self.r#facing == Facing::East && self.r#occupied == false { return 1905; }
        if self.r#occupied == true && self.r#facing == Facing::South && self.r#part == Part::Head { return 1895; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::East { return 1904; }
        if self.r#facing == Facing::East && self.r#occupied == true && self.r#part == Part::Head { return 1903; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::North { return 1894; }
        if self.r#occupied == true && self.r#facing == Facing::West && self.r#part == Part::Head { return 1899; }
        if self.r#occupied == false && self.r#part == Part::Head && self.r#facing == Facing::North { return 1893; }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::South { return 1896; }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Foot { return 1906; }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::West { return 1902; }
        if self.r#facing == Facing::North && self.r#occupied == true && self.r#part == Part::Head { return 1891; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1901 {
            return Some(PurpleBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1897 {
            return Some(PurpleBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1892 {
            return Some(PurpleBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1900 {
            return Some(PurpleBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1898 {
            return Some(PurpleBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 1905 {
            return Some(PurpleBed {
                r#part: Part::Head,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        if state_id == 1895 {
            return Some(PurpleBed {
                r#occupied: true,
                r#facing: Facing::South,
                r#part: Part::Head,
            });
        }
        if state_id == 1904 {
            return Some(PurpleBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::East,
            });
        }
        if state_id == 1903 {
            return Some(PurpleBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1894 {
            return Some(PurpleBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1899 {
            return Some(PurpleBed {
                r#occupied: true,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1893 {
            return Some(PurpleBed {
                r#occupied: false,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        if state_id == 1896 {
            return Some(PurpleBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        if state_id == 1906 {
            return Some(PurpleBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1902 {
            return Some(PurpleBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::West,
            });
        }
        if state_id == 1891 {
            return Some(PurpleBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        return None;
    }
}

