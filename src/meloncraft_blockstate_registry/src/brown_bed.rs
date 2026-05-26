use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownBed {
    pub occupied: bool,
    pub r#part: Part,
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

impl BlockState for BrownBed {
    fn to_id(&self) -> i32 {
        if self.r#occupied == false && self.r#facing == Facing::North && self.r#part == Part::Head {
            return 1925;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == true {
            return 1932;
        }
        if self.r#facing == Facing::South && self.r#occupied == false && self.r#part == Part::Head {
            return 1929;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == false {
            return 1934;
        }
        if self.r#part == Part::Foot && self.r#occupied == true && self.r#facing == Facing::South {
            return 1928;
        }
        if self.r#occupied == true && self.r#facing == Facing::North && self.r#part == Part::Foot {
            return 1924;
        }
        if self.r#facing == Facing::East && self.r#occupied == true && self.r#part == Part::Foot {
            return 1936;
        }
        if self.r#facing == Facing::East && self.r#part == Part::Head && self.r#occupied == false {
            return 1937;
        }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Foot {
            return 1938;
        }
        if self.r#part == Part::Head && self.r#occupied == true && self.r#facing == Facing::North {
            return 1923;
        }
        if self.r#facing == Facing::West && self.r#part == Part::Head && self.r#occupied == true {
            return 1931;
        }
        if self.r#facing == Facing::West && self.r#occupied == false && self.r#part == Part::Head {
            return 1933;
        }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::North {
            return 1926;
        }
        if self.r#part == Part::Head && self.r#occupied == true && self.r#facing == Facing::East {
            return 1935;
        }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::South {
            return 1927;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::South && self.r#occupied == false {
            return 1930;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1925 {
            return Some(BrownBed {
                r#occupied: false,
                r#facing: Facing::North,
                r#part: Part::Head,
            });
        }
        if state_id == 1932 {
            return Some(BrownBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1929 {
            return Some(BrownBed {
                r#facing: Facing::South,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1934 {
            return Some(BrownBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1928 {
            return Some(BrownBed {
                r#part: Part::Foot,
                r#occupied: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 1924 {
            return Some(BrownBed {
                r#occupied: true,
                r#facing: Facing::North,
                r#part: Part::Foot,
            });
        }
        if state_id == 1936 {
            return Some(BrownBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1937 {
            return Some(BrownBed {
                r#facing: Facing::East,
                r#part: Part::Head,
                r#occupied: false,
            });
        }
        if state_id == 1938 {
            return Some(BrownBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Foot,
            });
        }
        if state_id == 1923 {
            return Some(BrownBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 1931 {
            return Some(BrownBed {
                r#facing: Facing::West,
                r#part: Part::Head,
                r#occupied: true,
            });
        }
        if state_id == 1933 {
            return Some(BrownBed {
                r#facing: Facing::West,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1926 {
            return Some(BrownBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1935 {
            return Some(BrownBed {
                r#part: Part::Head,
                r#occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 1927 {
            return Some(BrownBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1930 {
            return Some(BrownBed {
                r#part: Part::Foot,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        return None;
    }
}
