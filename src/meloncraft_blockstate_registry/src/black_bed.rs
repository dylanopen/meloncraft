use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackBed {
    pub r#facing: Facing,
    pub r#part: Part,
    pub occupied: bool,
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

impl BlockState for BlackBed {
    fn to_id(&self) -> i32 {
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::South {
            return 1975;
        }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::South {
            return 1976;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::East && self.r#occupied == false {
            return 1986;
        }
        if self.r#part == Part::Head && self.r#facing == Facing::South && self.r#occupied == false {
            return 1977;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::North && self.r#occupied == false {
            return 1974;
        }
        if self.r#part == Part::Head && self.r#occupied == false && self.r#facing == Facing::North {
            return 1973;
        }
        if self.r#facing == Facing::North && self.r#occupied == true && self.r#part == Part::Head {
            return 1971;
        }
        if self.r#facing == Facing::North && self.r#part == Part::Foot && self.r#occupied == true {
            return 1972;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == true {
            return 1980;
        }
        if self.r#occupied == false && self.r#facing == Facing::East && self.r#part == Part::Head {
            return 1985;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == false {
            return 1982;
        }
        if self.r#facing == Facing::West && self.r#occupied == true && self.r#part == Part::Head {
            return 1979;
        }
        if self.r#occupied == false && self.r#facing == Facing::West && self.r#part == Part::Head {
            return 1981;
        }
        if self.r#occupied == false && self.r#part == Part::Foot && self.r#facing == Facing::South {
            return 1978;
        }
        if self.r#facing == Facing::East && self.r#occupied == true && self.r#part == Part::Foot {
            return 1984;
        }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::East {
            return 1983;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1975 {
            return Some(BlackBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1976 {
            return Some(BlackBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        if state_id == 1986 {
            return Some(BlackBed {
                r#part: Part::Foot,
                r#facing: Facing::East,
                r#occupied: false,
            });
        }
        if state_id == 1977 {
            return Some(BlackBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1974 {
            return Some(BlackBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1973 {
            return Some(BlackBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 1971 {
            return Some(BlackBed {
                r#facing: Facing::North,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1972 {
            return Some(BlackBed {
                r#facing: Facing::North,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1980 {
            return Some(BlackBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: true,
            });
        }
        if state_id == 1985 {
            return Some(BlackBed {
                r#occupied: false,
                r#facing: Facing::East,
                r#part: Part::Head,
            });
        }
        if state_id == 1982 {
            return Some(BlackBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1979 {
            return Some(BlackBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        if state_id == 1981 {
            return Some(BlackBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1978 {
            return Some(BlackBed {
                r#occupied: false,
                r#part: Part::Foot,
                r#facing: Facing::South,
            });
        }
        if state_id == 1984 {
            return Some(BlackBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1983 {
            return Some(BlackBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
