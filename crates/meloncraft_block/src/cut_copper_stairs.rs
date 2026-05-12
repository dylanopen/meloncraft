use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutCopperStairs {
    pub waterlogged: bool,
    pub r#half: Half,
    pub r#facing: Facing,
    pub r#shape: Shape,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

impl BlockState for CutCopperStairs {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25370; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 25416; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 25382; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25421; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 25415; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::East { return 25437; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 25438; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight { return 25400; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 25384; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 25397; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 25427; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 25394; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 25396; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 25406; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 25378; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 25367; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 25377; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 25383; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 25433; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 25379; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 25387; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 25431; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 25395; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 25419; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 25389; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 25423; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 25365; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 25405; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25408; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West { return 25411; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 25381; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 25425; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 25426; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 25434; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 25375; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom { return 25422; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 25429; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight { return 25440; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 25428; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 25444; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 25443; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 25369; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 25412; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 25409; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 25435; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25410; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 25380; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 25386; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 25376; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 25388; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 25413; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 25432; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 25436; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 25372; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 25398; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 25402; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 25441; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top { return 25371; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West { return 25418; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 25374; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 25439; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 25424; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South { return 25391; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::South { return 25403; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 25414; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 25407; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::South { return 25404; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 25366; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 25373; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 25442; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top { return 25393; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 25401; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 25390; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 25420; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 25399; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25417; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25430; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25392; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 25368; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 25385; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25370 {
            return Some(CutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25416 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25382 {
            return Some(CutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25421 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25415 {
            return Some(CutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 25437 {
            return Some(CutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 25438 {
            return Some(CutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25400 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25384 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25397 {
            return Some(CutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25427 {
            return Some(CutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25394 {
            return Some(CutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25396 {
            return Some(CutCopperStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 25406 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 25378 {
            return Some(CutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25367 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25377 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25383 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 25433 {
            return Some(CutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25379 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25387 {
            return Some(CutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25431 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25395 {
            return Some(CutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25419 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25389 {
            return Some(CutCopperStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 25423 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25365 {
            return Some(CutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25405 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25408 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25411 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 25381 {
            return Some(CutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25425 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25426 {
            return Some(CutCopperStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25434 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25375 {
            return Some(CutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 25422 {
            return Some(CutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25429 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25440 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25428 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25444 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25443 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25369 {
            return Some(CutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25412 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 25409 {
            return Some(CutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 25435 {
            return Some(CutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25410 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25380 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25386 {
            return Some(CutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 25376 {
            return Some(CutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25388 {
            return Some(CutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25413 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 25432 {
            return Some(CutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25436 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25372 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 25398 {
            return Some(CutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25402 {
            return Some(CutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25441 {
            return Some(CutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25371 {
            return Some(CutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25418 {
            return Some(CutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 25374 {
            return Some(CutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 25439 {
            return Some(CutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25424 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25391 {
            return Some(CutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 25403 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 25414 {
            return Some(CutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25407 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 25404 {
            return Some(CutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 25366 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 25373 {
            return Some(CutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25442 {
            return Some(CutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 25393 {
            return Some(CutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 25401 {
            return Some(CutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25390 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25420 {
            return Some(CutCopperStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25399 {
            return Some(CutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25417 {
            return Some(CutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25430 {
            return Some(CutCopperStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25392 {
            return Some(CutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25368 {
            return Some(CutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 25385 {
            return Some(CutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

