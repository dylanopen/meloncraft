use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenBed {
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

impl BlockState for GreenBed {
    fn to_id(&self) -> i32 {
        if self.r#part == Part::Head && self.r#facing == Facing::South && self.r#occupied == false {
            return 1945;
        }
        if self.r#part == Part::Head && self.r#occupied == false && self.r#facing == Facing::North {
            return 1941;
        }
        if self.r#occupied == true && self.r#facing == Facing::West && self.r#part == Part::Head {
            return 1947;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::West && self.r#occupied == false {
            return 1950;
        }
        if self.r#occupied == true && self.r#part == Part::Foot && self.r#facing == Facing::North {
            return 1940;
        }
        if self.r#part == Part::Foot && self.r#facing == Facing::North && self.r#occupied == false {
            return 1942;
        }
        if self.r#occupied == false && self.r#facing == Facing::West && self.r#part == Part::Head {
            return 1949;
        }
        if self.r#facing == Facing::East && self.r#part == Part::Foot && self.r#occupied == true {
            return 1952;
        }
        if self.r#occupied == false && self.r#facing == Facing::South && self.r#part == Part::Foot {
            return 1946;
        }
        if self.r#facing == Facing::West && self.r#occupied == true && self.r#part == Part::Foot {
            return 1948;
        }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::South {
            return 1943;
        }
        if self.r#part == Part::Foot && self.r#occupied == false && self.r#facing == Facing::East {
            return 1954;
        }
        if self.r#occupied == true && self.r#part == Part::Head && self.r#facing == Facing::North {
            return 1939;
        }
        if self.r#facing == Facing::South && self.r#occupied == true && self.r#part == Part::Foot {
            return 1944;
        }
        if self.r#facing == Facing::East && self.r#occupied == false && self.r#part == Part::Head {
            return 1953;
        }
        if self.r#facing == Facing::East && self.r#occupied == true && self.r#part == Part::Head {
            return 1951;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1945 {
            return Some(GreenBed {
                r#part: Part::Head,
                r#facing: Facing::South,
                r#occupied: false,
            });
        }
        if state_id == 1941 {
            return Some(GreenBed {
                r#part: Part::Head,
                r#occupied: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 1947 {
            return Some(GreenBed {
                r#occupied: true,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1950 {
            return Some(GreenBed {
                r#part: Part::Foot,
                r#facing: Facing::West,
                r#occupied: false,
            });
        }
        if state_id == 1940 {
            return Some(GreenBed {
                r#occupied: true,
                r#part: Part::Foot,
                r#facing: Facing::North,
            });
        }
        if state_id == 1942 {
            return Some(GreenBed {
                r#part: Part::Foot,
                r#facing: Facing::North,
                r#occupied: false,
            });
        }
        if state_id == 1949 {
            return Some(GreenBed {
                r#occupied: false,
                r#facing: Facing::West,
                r#part: Part::Head,
            });
        }
        if state_id == 1952 {
            return Some(GreenBed {
                r#facing: Facing::East,
                r#part: Part::Foot,
                r#occupied: true,
            });
        }
        if state_id == 1946 {
            return Some(GreenBed {
                r#occupied: false,
                r#facing: Facing::South,
                r#part: Part::Foot,
            });
        }
        if state_id == 1948 {
            return Some(GreenBed {
                r#facing: Facing::West,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1943 {
            return Some(GreenBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::South,
            });
        }
        if state_id == 1954 {
            return Some(GreenBed {
                r#part: Part::Foot,
                r#occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 1939 {
            return Some(GreenBed {
                r#occupied: true,
                r#part: Part::Head,
                r#facing: Facing::North,
            });
        }
        if state_id == 1944 {
            return Some(GreenBed {
                r#facing: Facing::South,
                r#occupied: true,
                r#part: Part::Foot,
            });
        }
        if state_id == 1953 {
            return Some(GreenBed {
                r#facing: Facing::East,
                r#occupied: false,
                r#part: Part::Head,
            });
        }
        if state_id == 1951 {
            return Some(GreenBed {
                r#facing: Facing::East,
                r#occupied: true,
                r#part: Part::Head,
            });
        }
        return None;
    }
}
