use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoweredRail {
    pub powered: bool,
    pub waterlogged: bool,
    pub r#shape: Shape,
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
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#shape == Shape::AscendingEast && block_state.r#waterlogged == true { return 1991; }
        if block_state.r#shape == Shape::AscendingNorth && block_state.r#waterlogged == false && block_state.r#powered == true { return 1996; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#shape == Shape::AscendingSouth { return 1997; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::EastWest && block_state.r#powered == true { return 1989; }
        if block_state.r#powered == false && block_state.r#shape == Shape::AscendingNorth && block_state.r#waterlogged == false { return 2008; }
        if block_state.r#powered == false && block_state.r#shape == Shape::AscendingSouth && block_state.r#waterlogged == false { return 2010; }
        if block_state.r#powered == false && block_state.r#shape == Shape::AscendingWest && block_state.r#waterlogged == true { return 2005; }
        if block_state.r#powered == true && block_state.r#shape == Shape::AscendingWest && block_state.r#waterlogged == true { return 1993; }
        if block_state.r#powered == true && block_state.r#shape == Shape::AscendingNorth && block_state.r#waterlogged == true { return 1995; }
        if block_state.r#shape == Shape::EastWest && block_state.r#waterlogged == false && block_state.r#powered == true { return 1990; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#shape == Shape::AscendingSouth { return 1998; }
        if block_state.r#shape == Shape::AscendingEast && block_state.r#powered == false && block_state.r#waterlogged == true { return 2003; }
        if block_state.r#shape == Shape::AscendingEast && block_state.r#waterlogged == false && block_state.r#powered == false { return 2004; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#shape == Shape::NorthSouth { return 2000; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#shape == Shape::AscendingWest { return 1994; }
        if block_state.r#shape == Shape::AscendingWest && block_state.r#waterlogged == false && block_state.r#powered == false { return 2006; }
        if block_state.r#shape == Shape::AscendingSouth && block_state.r#waterlogged == true && block_state.r#powered == false { return 2009; }
        if block_state.r#shape == Shape::EastWest && block_state.r#waterlogged == true && block_state.r#powered == false { return 2001; }
        if block_state.r#shape == Shape::NorthSouth && block_state.r#waterlogged == true && block_state.r#powered == true { return 1987; }
        if block_state.r#shape == Shape::NorthSouth && block_state.r#waterlogged == true && block_state.r#powered == false { return 1999; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#shape == Shape::EastWest { return 2002; }
        if block_state.r#shape == Shape::NorthSouth && block_state.r#powered == true && block_state.r#waterlogged == false { return 1988; }
        if block_state.r#powered == true && block_state.r#shape == Shape::AscendingEast && block_state.r#waterlogged == false { return 1992; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#shape == Shape::AscendingNorth { return 2007; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1991 {
            return Some(PoweredRail {
                r#powered: true,
                r#shape: Shape::AscendingEast,
                r#waterlogged: true,
            });
        }
        if state_id == 1996 {
            return Some(PoweredRail {
                r#shape: Shape::AscendingNorth,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 1997 {
            return Some(PoweredRail {
                r#powered: true,
                r#waterlogged: true,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 1989 {
            return Some(PoweredRail {
                r#waterlogged: true,
                r#shape: Shape::EastWest,
                r#powered: true,
            });
        }
        if state_id == 2008 {
            return Some(PoweredRail {
                r#powered: false,
                r#shape: Shape::AscendingNorth,
                r#waterlogged: false,
            });
        }
        if state_id == 2010 {
            return Some(PoweredRail {
                r#powered: false,
                r#shape: Shape::AscendingSouth,
                r#waterlogged: false,
            });
        }
        if state_id == 2005 {
            return Some(PoweredRail {
                r#powered: false,
                r#shape: Shape::AscendingWest,
                r#waterlogged: true,
            });
        }
        if state_id == 1993 {
            return Some(PoweredRail {
                r#powered: true,
                r#shape: Shape::AscendingWest,
                r#waterlogged: true,
            });
        }
        if state_id == 1995 {
            return Some(PoweredRail {
                r#powered: true,
                r#shape: Shape::AscendingNorth,
                r#waterlogged: true,
            });
        }
        if state_id == 1990 {
            return Some(PoweredRail {
                r#shape: Shape::EastWest,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 1998 {
            return Some(PoweredRail {
                r#waterlogged: false,
                r#powered: true,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 2003 {
            return Some(PoweredRail {
                r#shape: Shape::AscendingEast,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2004 {
            return Some(PoweredRail {
                r#shape: Shape::AscendingEast,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2000 {
            return Some(PoweredRail {
                r#waterlogged: false,
                r#powered: false,
                r#shape: Shape::NorthSouth,
            });
        }
        if state_id == 1994 {
            return Some(PoweredRail {
                r#waterlogged: false,
                r#powered: true,
                r#shape: Shape::AscendingWest,
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
                r#shape: Shape::AscendingSouth,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2001 {
            return Some(PoweredRail {
                r#shape: Shape::EastWest,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 1987 {
            return Some(PoweredRail {
                r#shape: Shape::NorthSouth,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 1999 {
            return Some(PoweredRail {
                r#shape: Shape::NorthSouth,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2002 {
            return Some(PoweredRail {
                r#waterlogged: false,
                r#powered: false,
                r#shape: Shape::EastWest,
            });
        }
        if state_id == 1988 {
            return Some(PoweredRail {
                r#shape: Shape::NorthSouth,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 1992 {
            return Some(PoweredRail {
                r#powered: true,
                r#shape: Shape::AscendingEast,
                r#waterlogged: false,
            });
        }
        if state_id == 2007 {
            return Some(PoweredRail {
                r#waterlogged: true,
                r#powered: false,
                r#shape: Shape::AscendingNorth,
            });
        }
        return None;
    }
}

