use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperTrapdoor {
    pub powered: bool,
    pub r#facing: Facing,
    pub open: bool,
    pub r#half: Half,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for OxidizedCopperTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 26465; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true { return 26464; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#powered == true { return 26522; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == true { return 26511; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 26512; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false { return 26482; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26479; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 26491; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 26502; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 26472; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 26490; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 26484; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == true { return 26513; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 26509; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::West { return 26494; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#waterlogged == false { return 26474; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 26517; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == true { return 26470; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false { return 26488; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::West { return 26500; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 26493; }
        if block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#powered == false { return 26475; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::South { return 26485; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == true { return 26495; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true { return 26520; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Top { return 26478; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true { return 26481; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 26483; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true { return 26461; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#facing == Facing::North { return 26466; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == false { return 26467; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North { return 26473; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == false { return 26506; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == true { return 26507; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 26524; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == false { return 26505; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == true { return 26471; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == false { return 26492; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == true { return 26514; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::South { return 26480; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == true { return 26469; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == false { return 26504; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 26508; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true { return 26523; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == false { return 26498; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == true { return 26486; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 26518; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26462; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#powered == false { return 26519; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == false { return 26489; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true { return 26497; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 26468; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::West { return 26501; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == false { return 26496; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#open == true { return 26463; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#open == false { return 26476; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26510; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 26521; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Top { return 26477; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == false { return 26499; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#open == false { return 26515; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false { return 26516; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 26487; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 26503; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26465 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26464 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26522 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 26511 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26512 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26482 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26479 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26491 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26502 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26472 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26490 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26484 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 26513 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26509 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 26494 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26474 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26517 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26470 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26488 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26500 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26493 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 26475 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 26485 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26495 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26520 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26478 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26481 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26483 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26461 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26466 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26467 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26473 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26506 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26507 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26524 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26505 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26471 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26492 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26514 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26480 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26469 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26504 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26508 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26523 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26498 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26486 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26518 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26462 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26519 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26489 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26497 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 26468 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 26501 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26496 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26463 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 26476 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26510 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26521 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26477 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26499 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26515 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 26516 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26487 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26503 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

