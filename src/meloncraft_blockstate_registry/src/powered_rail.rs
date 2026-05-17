use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoweredRail {
    pub r#shape: Shape,
    pub powered: bool,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
}

impl BlockState for PoweredRail {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::AscendingNorth && self.r#waterlogged == true && self.r#powered == true { return 1995; }
        if self.r#waterlogged == true && self.r#shape == Shape::NorthSouth && self.r#powered == false { return 1999; }
        if self.r#waterlogged == true && self.r#shape == Shape::AscendingEast && self.r#powered == false { return 2003; }
        if self.r#shape == Shape::AscendingSouth && self.r#powered == true && self.r#waterlogged == true { return 1997; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#shape == Shape::AscendingEast { return 1992; }
        if self.r#shape == Shape::AscendingWest && self.r#waterlogged == false && self.r#powered == false { return 2006; }
        if self.r#waterlogged == true && self.r#powered == false && self.r#shape == Shape::AscendingSouth { return 2009; }
        if self.r#shape == Shape::AscendingSouth && self.r#powered == false && self.r#waterlogged == false { return 2010; }
        if self.r#waterlogged == false && self.r#shape == Shape::EastWest && self.r#powered == true { return 1990; }
        if self.r#shape == Shape::AscendingNorth && self.r#powered == false && self.r#waterlogged == true { return 2007; }
        if self.r#shape == Shape::NorthSouth && self.r#waterlogged == false && self.r#powered == false { return 2000; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#shape == Shape::AscendingSouth { return 1998; }
        if self.r#powered == true && self.r#shape == Shape::AscendingEast && self.r#waterlogged == true { return 1991; }
        if self.r#powered == true && self.r#shape == Shape::NorthSouth && self.r#waterlogged == true { return 1987; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#shape == Shape::AscendingNorth { return 1996; }
        if self.r#waterlogged == true && self.r#shape == Shape::AscendingWest && self.r#powered == true { return 1993; }
        if self.r#shape == Shape::EastWest && self.r#powered == false && self.r#waterlogged == true { return 2001; }
        if self.r#waterlogged == false && self.r#shape == Shape::AscendingEast && self.r#powered == false { return 2004; }
        if self.r#powered == true && self.r#shape == Shape::EastWest && self.r#waterlogged == true { return 1989; }
        if self.r#shape == Shape::AscendingWest && self.r#waterlogged == false && self.r#powered == true { return 1994; }
        if self.r#shape == Shape::EastWest && self.r#waterlogged == false && self.r#powered == false { return 2002; }
        if self.r#powered == false && self.r#shape == Shape::AscendingWest && self.r#waterlogged == true { return 2005; }
        if self.r#powered == false && self.r#shape == Shape::AscendingNorth && self.r#waterlogged == false { return 2008; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#shape == Shape::NorthSouth { return 1988; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1995 {
            return Some(PoweredRail {
                r#shape: Shape::AscendingNorth,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 1999 {
            return Some(PoweredRail {
                r#waterlogged: true,
                r#shape: Shape::NorthSouth,
                r#powered: false,
            });
        }
        if state_id == 2003 {
            return Some(PoweredRail {
                r#waterlogged: true,
                r#shape: Shape::AscendingEast,
                r#powered: false,
            });
        }
        if state_id == 1997 {
            return Some(PoweredRail {
                r#shape: Shape::AscendingSouth,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 1992 {
            return Some(PoweredRail {
                r#powered: true,
                r#waterlogged: false,
                r#shape: Shape::AscendingEast,
            });
        }
        if state_id == 2006 {
            return Some(PoweredRail {
                r#shape: Shape::AscendingWest,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2009 {
            return Some(PoweredRail {
                r#waterlogged: true,
                r#powered: false,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 2010 {
            return Some(PoweredRail {
                r#shape: Shape::AscendingSouth,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 1990 {
            return Some(PoweredRail {
                r#waterlogged: false,
                r#shape: Shape::EastWest,
                r#powered: true,
            });
        }
        if state_id == 2007 {
            return Some(PoweredRail {
                r#shape: Shape::AscendingNorth,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2000 {
            return Some(PoweredRail {
                r#shape: Shape::NorthSouth,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 1998 {
            return Some(PoweredRail {
                r#waterlogged: false,
                r#powered: true,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 1991 {
            return Some(PoweredRail {
                r#powered: true,
                r#shape: Shape::AscendingEast,
                r#waterlogged: true,
            });
        }
        if state_id == 1987 {
            return Some(PoweredRail {
                r#powered: true,
                r#shape: Shape::NorthSouth,
                r#waterlogged: true,
            });
        }
        if state_id == 1996 {
            return Some(PoweredRail {
                r#powered: true,
                r#waterlogged: false,
                r#shape: Shape::AscendingNorth,
            });
        }
        if state_id == 1993 {
            return Some(PoweredRail {
                r#waterlogged: true,
                r#shape: Shape::AscendingWest,
                r#powered: true,
            });
        }
        if state_id == 2001 {
            return Some(PoweredRail {
                r#shape: Shape::EastWest,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2004 {
            return Some(PoweredRail {
                r#waterlogged: false,
                r#shape: Shape::AscendingEast,
                r#powered: false,
            });
        }
        if state_id == 1989 {
            return Some(PoweredRail {
                r#powered: true,
                r#shape: Shape::EastWest,
                r#waterlogged: true,
            });
        }
        if state_id == 1994 {
            return Some(PoweredRail {
                r#shape: Shape::AscendingWest,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2002 {
            return Some(PoweredRail {
                r#shape: Shape::EastWest,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2005 {
            return Some(PoweredRail {
                r#powered: false,
                r#shape: Shape::AscendingWest,
                r#waterlogged: true,
            });
        }
        if state_id == 2008 {
            return Some(PoweredRail {
                r#powered: false,
                r#shape: Shape::AscendingNorth,
                r#waterlogged: false,
            });
        }
        if state_id == 1988 {
            return Some(PoweredRail {
                r#waterlogged: false,
                r#powered: true,
                r#shape: Shape::NorthSouth,
            });
        }
        return None;
    }
}

