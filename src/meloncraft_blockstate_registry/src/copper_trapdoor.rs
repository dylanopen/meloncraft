use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperTrapdoor {
    pub open: bool,
    pub r#facing: Facing,
    pub powered: bool,
    pub waterlogged: bool,
    pub r#half: Half,
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

impl BlockState for CopperTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#powered == false
            && self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 26392;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 26386;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 26355;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#powered == false
        {
            return 26363;
        }
        if self.r#facing == Facing::East
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 26388;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 26357;
        }
        if self.r#waterlogged == false
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#half == Half::Top
        {
            return 26384;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 26385;
        }
        if self.r#facing == Facing::West
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#powered == true
        {
            return 26365;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 26364;
        }
        if self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::East
        {
            return 26395;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 26379;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 26361;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 26345;
        }
        if self.r#open == false
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 26356;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Top
        {
            return 26368;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 26370;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 26360;
        }
        if self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 26374;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#open == false
            && self.r#facing == Facing::North
        {
            return 26338;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#open == false
        {
            return 26339;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 26380;
        }
        if self.r#half == Half::Top
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 26337;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#open == true
        {
            return 26343;
        }
        if self.r#waterlogged == true
            && self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#open == true
        {
            return 26389;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 26393;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#open == true
            && self.r#waterlogged == true
        {
            return 26391;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
        {
            return 26346;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 26348;
        }
        if self.r#half == Half::Top
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#open == false
        {
            return 26372;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#powered == true
        {
            return 26358;
        }
        if self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#half == Half::Top
        {
            return 26335;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 26349;
        }
        if self.r#waterlogged == false
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 26354;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#open == true
            && self.r#facing == Facing::West
        {
            return 26376;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 26377;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 26394;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 26375;
        }
        if self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#waterlogged == true
        {
            return 26333;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 26342;
        }
        if self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#half == Half::Top
        {
            return 26367;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 26373;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Top
        {
            return 26381;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#open == false
        {
            return 26396;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#open == true
        {
            return 26352;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == true
        {
            return 26341;
        }
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 26353;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#open == true
        {
            return 26382;
        }
        if self.r#half == Half::Top
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#facing == Facing::East
        {
            return 26387;
        }
        if self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 26390;
        }
        if self.r#half == Half::Top
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#open == false
        {
            return 26371;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#open == false
        {
            return 26378;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#facing == Facing::North
        {
            return 26344;
        }
        if self.r#open == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 26350;
        }
        if self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#facing == Facing::West
        {
            return 26369;
        }
        if self.r#powered == false
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 26336;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#open == true
        {
            return 26366;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 26347;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::North
        {
            return 26334;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 26359;
        }
        if self.r#waterlogged == false
            && self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 26362;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 26383;
        }
        if self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 26340;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 26351;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26392 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26386 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26355 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26363 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26388 {
            return Some(CopperTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26357 {
            return Some(CopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26384 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26385 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26365 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26364 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26395 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26379 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 26361 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26345 {
            return Some(CopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 26356 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 26368 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26370 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26360 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26374 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26338 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26339 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26380 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26337 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26343 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26389 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26393 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26391 {
            return Some(CopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26346 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 26348 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26372 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26358 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26335 {
            return Some(CopperTrapdoor {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26349 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26354 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 26376 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26377 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26394 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26375 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26333 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26342 {
            return Some(CopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26367 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26373 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26381 {
            return Some(CopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26396 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26352 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26341 {
            return Some(CopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26353 {
            return Some(CopperTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26382 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26387 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26390 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26371 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26378 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26344 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26350 {
            return Some(CopperTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26369 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26336 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26366 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 26347 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26334 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26359 {
            return Some(CopperTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26362 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 26383 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 26340 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26351 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        return None;
    }
}
