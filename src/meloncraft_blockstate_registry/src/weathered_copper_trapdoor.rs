use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperTrapdoor {
    pub waterlogged: bool,
    pub powered: bool,
    pub r#facing: Facing,
    pub r#half: Half,
    pub open: bool,
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

impl BlockState for WeatheredCopperTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#open == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 26527;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#open == false
        {
            return 26562;
        }
        if self.r#half == Half::Top
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#waterlogged == false
        {
            return 26542;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 26564;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 26548;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#powered == false
        {
            return 26532;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 26576;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Top
        {
            return 26543;
        }
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 26554;
        }
        if self.r#open == false
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 26563;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#open == true
            && self.r#waterlogged == false
        {
            return 26558;
        }
        if self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#open == false
        {
            return 26579;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#powered == true
        {
            return 26557;
        }
        if self.r#open == true
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 26541;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 26560;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 26588;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 26530;
        }
        if self.r#open == false
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 26546;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 26549;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 26585;
        }
        if self.r#waterlogged == true
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#powered == false
        {
            return 26555;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 26567;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#open == false
        {
            return 26545;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Bottom
        {
            return 26584;
        }
        if self.r#waterlogged == true
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#open == false
        {
            return 26587;
        }
        if self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 26582;
        }
        if self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 26537;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 26577;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Top
        {
            return 26578;
        }
        if self.r#open == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 26565;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 26525;
        }
        if self.r#waterlogged == false
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#powered == true
        {
            return 26566;
        }
        if self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 26569;
        }
        if self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 26538;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#open == false
        {
            return 26547;
        }
        if self.r#facing == Facing::East
            && self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 26573;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#waterlogged == false
        {
            return 26544;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 26550;
        }
        if self.r#open == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#half == Half::Bottom
        {
            return 26552;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#open == true
        {
            return 26551;
        }
        if self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#open == false
        {
            return 26556;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 26559;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#open == true
        {
            return 26535;
        }
        if self.r#open == true
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 26526;
        }
        if self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 26539;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::East
        {
            return 26583;
        }
        if self.r#facing == Facing::North
            && self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 26536;
        }
        if self.r#open == false
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 26572;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 26575;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#open == true
            && self.r#powered == true
        {
            return 26534;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#open == true
            && self.r#half == Half::Top
        {
            return 26528;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#powered == true
        {
            return 26561;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 26540;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#half == Half::Top
            && self.r#open == true
        {
            return 26574;
        }
        if self.r#facing == Facing::West
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 26568;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 26571;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#waterlogged == true
        {
            return 26581;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
        {
            return 26586;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#open == false
            && self.r#powered == false
        {
            return 26580;
        }
        if self.r#open == false
            && self.r#half == Half::Top
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 26531;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == false
        {
            return 26529;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#open == false
        {
            return 26553;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#powered == true
        {
            return 26570;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#open == true
        {
            return 26533;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26527 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26562 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26542 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26564 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 26548 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26532 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26576 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 26543 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26554 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 26563 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26558 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26579 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26557 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 26541 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26560 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26588 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26530 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26546 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 26549 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26585 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26555 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26567 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26545 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26584 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26587 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 26582 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26537 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26577 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 26578 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26565 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26525 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26566 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26569 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26538 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26547 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26573 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26544 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26550 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 26552 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26551 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26556 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26559 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 26535 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26526 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 26539 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 26583 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26536 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26572 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 26575 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 26534 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26528 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26561 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 26540 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26574 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26568 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26571 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26581 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26586 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 26580 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26531 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26529 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26553 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26570 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26533 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
                r#open: true,
            });
        }
        return None;
    }
}
