use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatorRail {
    pub powered: bool,
    pub r#shape: Shape,
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

impl BlockState for ActivatorRail {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#shape == Shape::NorthSouth && block_state.r#waterlogged == false { return 11219; }
        if block_state.r#powered == true && block_state.r#shape == Shape::EastWest && block_state.r#waterlogged == true { return 11208; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#shape == Shape::AscendingSouth { return 11216; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#shape == Shape::AscendingEast { return 11222; }
        if block_state.r#powered == true && block_state.r#shape == Shape::AscendingEast && block_state.r#waterlogged == false { return 11211; }
        if block_state.r#shape == Shape::NorthSouth && block_state.r#powered == false && block_state.r#waterlogged == true { return 11218; }
        if block_state.r#shape == Shape::EastWest && block_state.r#waterlogged == true && block_state.r#powered == false { return 11220; }
        if block_state.r#powered == true && block_state.r#shape == Shape::AscendingEast && block_state.r#waterlogged == true { return 11210; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#shape == Shape::AscendingWest { return 11213; }
        if block_state.r#powered == true && block_state.r#shape == Shape::AscendingNorth && block_state.r#waterlogged == true { return 11214; }
        if block_state.r#shape == Shape::AscendingNorth && block_state.r#powered == false && block_state.r#waterlogged == true { return 11226; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#shape == Shape::AscendingSouth { return 11217; }
        if block_state.r#powered == false && block_state.r#shape == Shape::AscendingNorth && block_state.r#waterlogged == false { return 11227; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::AscendingSouth && block_state.r#powered == false { return 11228; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#shape == Shape::AscendingEast { return 11223; }
        if block_state.r#powered == false && block_state.r#shape == Shape::AscendingSouth && block_state.r#waterlogged == false { return 11229; }
        if block_state.r#powered == true && block_state.r#shape == Shape::EastWest && block_state.r#waterlogged == false { return 11209; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#shape == Shape::AscendingNorth { return 11215; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#shape == Shape::AscendingWest { return 11212; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#shape == Shape::NorthSouth { return 11207; }
        if block_state.r#shape == Shape::EastWest && block_state.r#waterlogged == false && block_state.r#powered == false { return 11221; }
        if block_state.r#powered == false && block_state.r#shape == Shape::AscendingWest && block_state.r#waterlogged == false { return 11225; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::AscendingWest && block_state.r#powered == false { return 11224; }
        if block_state.r#shape == Shape::NorthSouth && block_state.r#powered == true && block_state.r#waterlogged == true { return 11206; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11219 {
            return Some(ActivatorRail {
                r#powered: false,
                r#shape: Shape::NorthSouth,
                r#waterlogged: false,
            });
        }
        if state_id == 11208 {
            return Some(ActivatorRail {
                r#powered: true,
                r#shape: Shape::EastWest,
                r#waterlogged: true,
            });
        }
        if state_id == 11216 {
            return Some(ActivatorRail {
                r#powered: true,
                r#waterlogged: true,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 11222 {
            return Some(ActivatorRail {
                r#powered: false,
                r#waterlogged: true,
                r#shape: Shape::AscendingEast,
            });
        }
        if state_id == 11211 {
            return Some(ActivatorRail {
                r#powered: true,
                r#shape: Shape::AscendingEast,
                r#waterlogged: false,
            });
        }
        if state_id == 11218 {
            return Some(ActivatorRail {
                r#shape: Shape::NorthSouth,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11220 {
            return Some(ActivatorRail {
                r#shape: Shape::EastWest,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 11210 {
            return Some(ActivatorRail {
                r#powered: true,
                r#shape: Shape::AscendingEast,
                r#waterlogged: true,
            });
        }
        if state_id == 11213 {
            return Some(ActivatorRail {
                r#powered: true,
                r#waterlogged: false,
                r#shape: Shape::AscendingWest,
            });
        }
        if state_id == 11214 {
            return Some(ActivatorRail {
                r#powered: true,
                r#shape: Shape::AscendingNorth,
                r#waterlogged: true,
            });
        }
        if state_id == 11226 {
            return Some(ActivatorRail {
                r#shape: Shape::AscendingNorth,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11217 {
            return Some(ActivatorRail {
                r#powered: true,
                r#waterlogged: false,
                r#shape: Shape::AscendingSouth,
            });
        }
        if state_id == 11227 {
            return Some(ActivatorRail {
                r#powered: false,
                r#shape: Shape::AscendingNorth,
                r#waterlogged: false,
            });
        }
        if state_id == 11228 {
            return Some(ActivatorRail {
                r#waterlogged: true,
                r#shape: Shape::AscendingSouth,
                r#powered: false,
            });
        }
        if state_id == 11223 {
            return Some(ActivatorRail {
                r#waterlogged: false,
                r#powered: false,
                r#shape: Shape::AscendingEast,
            });
        }
        if state_id == 11229 {
            return Some(ActivatorRail {
                r#powered: false,
                r#shape: Shape::AscendingSouth,
                r#waterlogged: false,
            });
        }
        if state_id == 11209 {
            return Some(ActivatorRail {
                r#powered: true,
                r#shape: Shape::EastWest,
                r#waterlogged: false,
            });
        }
        if state_id == 11215 {
            return Some(ActivatorRail {
                r#waterlogged: false,
                r#powered: true,
                r#shape: Shape::AscendingNorth,
            });
        }
        if state_id == 11212 {
            return Some(ActivatorRail {
                r#waterlogged: true,
                r#powered: true,
                r#shape: Shape::AscendingWest,
            });
        }
        if state_id == 11207 {
            return Some(ActivatorRail {
                r#powered: true,
                r#waterlogged: false,
                r#shape: Shape::NorthSouth,
            });
        }
        if state_id == 11221 {
            return Some(ActivatorRail {
                r#shape: Shape::EastWest,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 11225 {
            return Some(ActivatorRail {
                r#powered: false,
                r#shape: Shape::AscendingWest,
                r#waterlogged: false,
            });
        }
        if state_id == 11224 {
            return Some(ActivatorRail {
                r#waterlogged: true,
                r#shape: Shape::AscendingWest,
                r#powered: false,
            });
        }
        if state_id == 11206 {
            return Some(ActivatorRail {
                r#shape: Shape::NorthSouth,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

