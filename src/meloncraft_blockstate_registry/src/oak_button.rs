use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakButton {
    pub r#face: Face,
    pub r#facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Floor,
    Wall,
    Ceiling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for OakButton {
    fn to_id(&self) -> i32 {
        if self.r#face == Face::Wall && self.r#facing == Facing::South && self.r#powered == true {
            return 10483;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::East && self.r#powered == true {
            return 10479;
        }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Wall {
            return 10482;
        }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Wall {
            return 10485;
        }
        if self.r#face == Face::Ceiling && self.r#powered == false && self.r#facing == Facing::North
        {
            return 10490;
        }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::South
        {
            return 10491;
        }
        if self.r#facing == Facing::South && self.r#face == Face::Floor && self.r#powered == true {
            return 10475;
        }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#face == Face::Floor {
            return 10473;
        }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::East {
            return 10488;
        }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::North
        {
            return 10489;
        }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Floor {
            return 10477;
        }
        if self.r#face == Face::Floor && self.r#powered == false && self.r#facing == Facing::West {
            return 10478;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::East && self.r#powered == false {
            return 10480;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::East && self.r#powered == true {
            return 10487;
        }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#face == Face::Wall {
            return 10486;
        }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#face == Face::Ceiling
        {
            return 10492;
        }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#face == Face::Ceiling
        {
            return 10494;
        }
        if self.r#facing == Facing::East && self.r#face == Face::Ceiling && self.r#powered == false
        {
            return 10496;
        }
        if self.r#facing == Facing::South && self.r#face == Face::Floor && self.r#powered == false {
            return 10476;
        }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::West {
            return 10493;
        }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#face == Face::Ceiling {
            return 10495;
        }
        if self.r#facing == Facing::South && self.r#face == Face::Wall && self.r#powered == false {
            return 10484;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::North {
            return 10481;
        }
        if self.r#face == Face::Floor && self.r#powered == false && self.r#facing == Facing::North {
            return 10474;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10483 {
            return Some(OakButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10479 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10482 {
            return Some(OakButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10485 {
            return Some(OakButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 10490 {
            return Some(OakButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10491 {
            return Some(OakButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 10475 {
            return Some(OakButton {
                r#facing: Facing::South,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 10473 {
            return Some(OakButton {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10488 {
            return Some(OakButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::East,
            });
        }
        if state_id == 10489 {
            return Some(OakButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 10477 {
            return Some(OakButton {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 10478 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10480 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10487 {
            return Some(OakButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10486 {
            return Some(OakButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10492 {
            return Some(OakButton {
                r#powered: false,
                r#facing: Facing::South,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10494 {
            return Some(OakButton {
                r#powered: false,
                r#facing: Facing::West,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10496 {
            return Some(OakButton {
                r#facing: Facing::East,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 10476 {
            return Some(OakButton {
                r#facing: Facing::South,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 10493 {
            return Some(OakButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 10495 {
            return Some(OakButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10484 {
            return Some(OakButton {
                r#facing: Facing::South,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10481 {
            return Some(OakButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10474 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
