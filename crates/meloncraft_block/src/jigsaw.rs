use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Jigsaw {
    pub r#orientation: Orientation,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
    EastUp,
    NorthUp,
    SouthUp,
}

impl BlockState for Jigsaw {
    fn to_id(&self) -> i32 {
        if self.r#orientation == Orientation::UpSouth { return 21530; }
        if self.r#orientation == Orientation::DownSouth { return 21526; }
        if self.r#orientation == Orientation::UpEast { return 21528; }
        if self.r#orientation == Orientation::SouthUp { return 21535; }
        if self.r#orientation == Orientation::DownNorth { return 21525; }
        if self.r#orientation == Orientation::WestUp { return 21532; }
        if self.r#orientation == Orientation::NorthUp { return 21534; }
        if self.r#orientation == Orientation::DownEast { return 21524; }
        if self.r#orientation == Orientation::DownWest { return 21527; }
        if self.r#orientation == Orientation::EastUp { return 21533; }
        if self.r#orientation == Orientation::UpNorth { return 21529; }
        if self.r#orientation == Orientation::UpWest { return 21531; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21530 {
            return Some(Jigsaw {
                r#orientation: Orientation::UpSouth,
            });
        }
        if state_id == 21526 {
            return Some(Jigsaw {
                r#orientation: Orientation::DownSouth,
            });
        }
        if state_id == 21528 {
            return Some(Jigsaw {
                r#orientation: Orientation::UpEast,
            });
        }
        if state_id == 21535 {
            return Some(Jigsaw {
                r#orientation: Orientation::SouthUp,
            });
        }
        if state_id == 21525 {
            return Some(Jigsaw {
                r#orientation: Orientation::DownNorth,
            });
        }
        if state_id == 21532 {
            return Some(Jigsaw {
                r#orientation: Orientation::WestUp,
            });
        }
        if state_id == 21534 {
            return Some(Jigsaw {
                r#orientation: Orientation::NorthUp,
            });
        }
        if state_id == 21524 {
            return Some(Jigsaw {
                r#orientation: Orientation::DownEast,
            });
        }
        if state_id == 21527 {
            return Some(Jigsaw {
                r#orientation: Orientation::DownWest,
            });
        }
        if state_id == 21533 {
            return Some(Jigsaw {
                r#orientation: Orientation::EastUp,
            });
        }
        if state_id == 21529 {
            return Some(Jigsaw {
                r#orientation: Orientation::UpNorth,
            });
        }
        if state_id == 21531 {
            return Some(Jigsaw {
                r#orientation: Orientation::UpWest,
            });
        }
        return None;
    }
}

