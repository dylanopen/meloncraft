use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperTrapdoor {
    pub r#half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
    pub r#facing: Facing,
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

impl BlockState for OxidizedCopperTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 26506;
        }
        if self.r#facing == Facing::North
            && self.r#open == true
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 26463;
        }
        if self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#powered == true
        {
            return 26498;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#powered == true
        {
            return 26469;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 26513;
        }
        if self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 26504;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 26515;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#half == Half::Top
        {
            return 26516;
        }
        if self.r#facing == Facing::South
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 26480;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 26518;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#powered == false
        {
            return 26495;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 26503;
        }
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 26481;
        }
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 26482;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#half == Half::Bottom
        {
            return 26507;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 26491;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#half == Half::Top
        {
            return 26493;
        }
        if self.r#powered == true
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 26477;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#open == false
        {
            return 26468;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#powered == false
        {
            return 26496;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 26492;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#open == false
        {
            return 26497;
        }
        if self.r#open == true
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 26510;
        }
        if self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#open == false
        {
            return 26465;
        }
        if self.r#open == true
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 26479;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#open == true
        {
            return 26512;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == true
        {
            return 26489;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 26500;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 26471;
        }
        if self.r#waterlogged == false
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#powered == true
        {
            return 26470;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#open == false
        {
            return 26476;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#open == false
            && self.r#half == Half::Top
        {
            return 26483;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 26519;
        }
        if self.r#open == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 26520;
        }
        if self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 26473;
        }
        if self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 26475;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#open == true
        {
            return 26486;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == true
        {
            return 26466;
        }
        if self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#half == Half::Bottom
        {
            return 26487;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 26505;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#open == true
            && self.r#waterlogged == true
        {
            return 26509;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#open == true
        {
            return 26472;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#open == false
        {
            return 26514;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 26499;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 26521;
        }
        if self.r#waterlogged == true
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 26467;
        }
        if self.r#open == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 26517;
        }
        if self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 26474;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#open == true
        {
            return 26464;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 26488;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#open == true
        {
            return 26494;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#open == false
            && self.r#powered == true
        {
            return 26522;
        }
        if self.r#open == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#powered == true
        {
            return 26478;
        }
        if self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::West
        {
            return 26508;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#open == true
        {
            return 26502;
        }
        if self.r#facing == Facing::North
            && self.r#open == true
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 26461;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#powered == false
        {
            return 26524;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 26501;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#open == true
            && self.r#half == Half::Top
        {
            return 26462;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 26511;
        }
        if self.r#open == false
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 26484;
        }
        if self.r#facing == Facing::East
            && self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 26523;
        }
        if self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 26485;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 26490;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26506 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26463 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26498 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26469 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26513 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 26504 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 26515 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26516 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26480 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26518 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26495 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26503 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26481 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26482 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26507 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26491 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26493 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26477 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26468 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26496 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26492 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26497 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26510 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 26465 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26479 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 26512 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26489 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26500 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26471 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26470 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26476 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26483 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26519 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26520 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26473 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26475 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 26486 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26466 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26487 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26505 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 26509 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26472 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26514 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26499 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 26521 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26467 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 26517 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26474 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26464 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26488 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26494 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 26522 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26478 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26508 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26502 {
            return Some(OxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 26461 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26524 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26501 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26462 {
            return Some(OxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26511 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26484 {
            return Some(OxidizedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26523 {
            return Some(OxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26485 {
            return Some(OxidizedCopperTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 26490 {
            return Some(OxidizedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        return None;
    }
}
