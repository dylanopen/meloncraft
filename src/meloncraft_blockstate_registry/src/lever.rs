use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lever {
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

impl BlockState for Lever {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#powered == true && self.r#face == Face::Wall {
            return 6580;
        }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::East && self.r#powered == false
        {
            return 6593;
        }
        if self.r#powered == false && self.r#face == Face::Floor && self.r#facing == Facing::North {
            return 6571;
        }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Floor {
            return 6574;
        }
        if self.r#powered == true && self.r#face == Face::Wall && self.r#facing == Facing::North {
            return 6578;
        }
        if self.r#face == Face::Floor && self.r#powered == false && self.r#facing == Facing::West {
            return 6575;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::North && self.r#powered == true {
            return 6570;
        }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#face == Face::Ceiling
        {
            return 6588;
        }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#face == Face::Ceiling
        {
            return 6586;
        }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#face == Face::Ceiling
        {
            return 6589;
        }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#face == Face::Wall {
            return 6585;
        }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::West && self.r#powered == false
        {
            return 6591;
        }
        if self.r#facing == Facing::South && self.r#face == Face::Floor && self.r#powered == true {
            return 6572;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::South && self.r#powered == false {
            return 6573;
        }
        if self.r#facing == Facing::East && self.r#face == Face::Floor && self.r#powered == false {
            return 6577;
        }
        if self.r#face == Face::Ceiling && self.r#powered == false && self.r#facing == Facing::North
        {
            return 6587;
        }
        if self.r#face == Face::Wall && self.r#powered == false && self.r#facing == Facing::South {
            return 6581;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::West && self.r#powered == false {
            return 6583;
        }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Wall {
            return 6579;
        }
        if self.r#face == Face::Floor && self.r#powered == true && self.r#facing == Facing::East {
            return 6576;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::East && self.r#powered == true {
            return 6584;
        }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Ceiling {
            return 6590;
        }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::East {
            return 6592;
        }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#face == Face::Wall {
            return 6582;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6580 {
            return Some(Lever {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        if state_id == 6593 {
            return Some(Lever {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 6571 {
            return Some(Lever {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 6574 {
            return Some(Lever {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Floor,
            });
        }
        if state_id == 6578 {
            return Some(Lever {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 6575 {
            return Some(Lever {
                r#face: Face::Floor,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6570 {
            return Some(Lever {
                r#face: Face::Floor,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 6588 {
            return Some(Lever {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 6586 {
            return Some(Lever {
                r#facing: Facing::North,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 6589 {
            return Some(Lever {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 6585 {
            return Some(Lever {
                r#powered: false,
                r#facing: Facing::East,
                r#face: Face::Wall,
            });
        }
        if state_id == 6591 {
            return Some(Lever {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 6572 {
            return Some(Lever {
                r#facing: Facing::South,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 6573 {
            return Some(Lever {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 6577 {
            return Some(Lever {
                r#facing: Facing::East,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 6587 {
            return Some(Lever {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6581 {
            return Some(Lever {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6583 {
            return Some(Lever {
                r#face: Face::Wall,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 6579 {
            return Some(Lever {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 6576 {
            return Some(Lever {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6584 {
            return Some(Lever {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 6590 {
            return Some(Lever {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 6592 {
            return Some(Lever {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 6582 {
            return Some(Lever {
                r#facing: Facing::West,
                r#powered: true,
                r#face: Face::Wall,
            });
        }
        return None;
    }
}
