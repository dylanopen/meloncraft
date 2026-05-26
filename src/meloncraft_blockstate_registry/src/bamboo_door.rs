use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooDoor {
    pub r#half: Half,
    pub r#hinge: Hinge,
    pub r#facing: Facing,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BambooDoor {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
        {
            return 14401;
        }
        if self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Left
        {
            return 14419;
        }
        if self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
        {
            return 14381;
        }
        if self.r#half == Half::Lower
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
            && self.r#open == false
        {
            return 14412;
        }
        if self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 14383;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#open == false
            && self.r#half == Half::Upper
            && self.r#powered == false
        {
            return 14421;
        }
        if self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#open == true
        {
            return 14386;
        }
        if self.r#open == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
        {
            return 14409;
        }
        if self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 14411;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 14394;
        }
        if self.r#open == true
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
        {
            return 14406;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
        {
            return 14372;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#powered == false
        {
            return 14373;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 14384;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#half == Half::Upper
        {
            return 14418;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
        {
            return 14420;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#open == true
        {
            return 14427;
        }
        if self.r#facing == Facing::West
            && self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#powered == false
        {
            return 14417;
        }
        if self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#hinge == Hinge::Left
        {
            return 14410;
        }
        if self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#powered == false
        {
            return 14423;
        }
        if self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#facing == Facing::West
        {
            return 14402;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#powered == true
        {
            return 14374;
        }
        if self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#open == false
        {
            return 14433;
        }
        if self.r#hinge == Hinge::Right
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#facing == Facing::South
            && self.r#open == false
        {
            return 14400;
        }
        if self.r#half == Half::Lower
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
        {
            return 14378;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#powered == true
        {
            return 14380;
        }
        if self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#facing == Facing::West
        {
            return 14403;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::West
        {
            return 14414;
        }
        if self.r#hinge == Hinge::Right
            && self.r#facing == Facing::West
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == false
        {
            return 14415;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
            && self.r#half == Half::Upper
        {
            return 14370;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
        {
            return 14428;
        }
        if self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 14392;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#facing == Facing::North
        {
            return 14376;
        }
        if self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 14397;
        }
        if self.r#powered == false
            && self.r#open == true
            && self.r#half == Half::Lower
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
        {
            return 14431;
        }
        if self.r#facing == Facing::West
            && self.r#open == false
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
        {
            return 14404;
        }
        if self.r#powered == false
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::West
        {
            return 14407;
        }
        if self.r#open == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
            && self.r#powered == false
        {
            return 14379;
        }
        if self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 14399;
        }
        if self.r#half == Half::Upper
            && self.r#open == true
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#powered == false
        {
            return 14375;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
        {
            return 14382;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == false
        {
            return 14385;
        }
        if self.r#open == true
            && self.r#half == Half::Upper
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Left
        {
            return 14371;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::South
        {
            return 14393;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 14396;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
            && self.r#half == Half::Lower
        {
            return 14413;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#open == true
        {
            return 14395;
        }
        if self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::West
        {
            return 14416;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == true
        {
            return 14430;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
        {
            return 14429;
        }
        if self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#open == true
        {
            return 14387;
        }
        if self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Upper
        {
            return 14388;
        }
        if self.r#facing == Facing::West
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#powered == false
        {
            return 14405;
        }
        if self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#half == Half::Lower
            && self.r#powered == true
        {
            return 14432;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
        {
            return 14377;
        }
        if self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
            && self.r#open == true
        {
            return 14422;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == true
        {
            return 14424;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#hinge == Hinge::Right
            && self.r#powered == true
        {
            return 14398;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::East
        {
            return 14425;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#powered == false
        {
            return 14391;
        }
        if self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#half == Half::Lower
        {
            return 14426;
        }
        if self.r#powered == true
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 14408;
        }
        if self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#open == false
            && self.r#half == Half::Upper
        {
            return 14389;
        }
        if self.r#hinge == Hinge::Right
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#facing == Facing::South
            && self.r#open == true
        {
            return 14390;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14401 {
            return Some(BambooDoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 14419 {
            return Some(BambooDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14381 {
            return Some(BambooDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 14412 {
            return Some(BambooDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 14383 {
            return Some(BambooDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 14421 {
            return Some(BambooDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 14386 {
            return Some(BambooDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 14409 {
            return Some(BambooDoor {
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14411 {
            return Some(BambooDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 14394 {
            return Some(BambooDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 14406 {
            return Some(BambooDoor {
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14372 {
            return Some(BambooDoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 14373 {
            return Some(BambooDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 14384 {
            return Some(BambooDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14418 {
            return Some(BambooDoor {
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 14420 {
            return Some(BambooDoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14427 {
            return Some(BambooDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14417 {
            return Some(BambooDoor {
                r#facing: Facing::West,
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14410 {
            return Some(BambooDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14423 {
            return Some(BambooDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14402 {
            return Some(BambooDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 14374 {
            return Some(BambooDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14433 {
            return Some(BambooDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 14400 {
            return Some(BambooDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 14378 {
            return Some(BambooDoor {
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14380 {
            return Some(BambooDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14403 {
            return Some(BambooDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 14414 {
            return Some(BambooDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14415 {
            return Some(BambooDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14370 {
            return Some(BambooDoor {
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 14428 {
            return Some(BambooDoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14392 {
            return Some(BambooDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14376 {
            return Some(BambooDoor {
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::North,
            });
        }
        if state_id == 14397 {
            return Some(BambooDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14431 {
            return Some(BambooDoor {
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14404 {
            return Some(BambooDoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14407 {
            return Some(BambooDoor {
                r#powered: false,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 14379 {
            return Some(BambooDoor {
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 14399 {
            return Some(BambooDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14375 {
            return Some(BambooDoor {
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 14382 {
            return Some(BambooDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14385 {
            return Some(BambooDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14371 {
            return Some(BambooDoor {
                r#open: true,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14393 {
            return Some(BambooDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 14396 {
            return Some(BambooDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 14413 {
            return Some(BambooDoor {
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Lower,
            });
        }
        if state_id == 14395 {
            return Some(BambooDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 14416 {
            return Some(BambooDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14430 {
            return Some(BambooDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14429 {
            return Some(BambooDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 14387 {
            return Some(BambooDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 14388 {
            return Some(BambooDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14405 {
            return Some(BambooDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14432 {
            return Some(BambooDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 14377 {
            return Some(BambooDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14422 {
            return Some(BambooDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14424 {
            return Some(BambooDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14398 {
            return Some(BambooDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 14425 {
            return Some(BambooDoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 14391 {
            return Some(BambooDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14426 {
            return Some(BambooDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14408 {
            return Some(BambooDoor {
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14389 {
            return Some(BambooDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14390 {
            return Some(BambooDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        return None;
    }
}
