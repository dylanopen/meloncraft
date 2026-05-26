use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryButton {
    pub r#face: Face,
    pub powered: bool,
    pub r#facing: Facing,
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

impl BlockState for CherryButton {
    fn to_id(&self) -> i32 {
        if self.r#powered == true && self.r#facing == Facing::North && self.r#face == Face::Floor {
            return 10593;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::South {
            return 10603;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::East && self.r#powered == false {
            return 10608;
        }
        if self.r#facing == Facing::South && self.r#face == Face::Ceiling && self.r#powered == true
        {
            return 10611;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::East {
            return 10607;
        }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#face == Face::Ceiling
        {
            return 10609;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::South && self.r#powered == true {
            return 10595;
        }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Wall {
            return 10602;
        }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::West {
            return 10606;
        }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#face == Face::Ceiling
        {
            return 10612;
        }
        if self.r#facing == Facing::West && self.r#face == Face::Floor && self.r#powered == true {
            return 10597;
        }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::West && self.r#powered == false
        {
            return 10614;
        }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#face == Face::Floor {
            return 10598;
        }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Floor {
            return 10600;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::South && self.r#powered == false {
            return 10604;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::North {
            return 10601;
        }
        if self.r#face == Face::Ceiling && self.r#powered == true && self.r#facing == Facing::West {
            return 10613;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::South && self.r#powered == false {
            return 10596;
        }
        if self.r#facing == Facing::North && self.r#face == Face::Floor && self.r#powered == false {
            return 10594;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::West {
            return 10605;
        }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#face == Face::Ceiling {
            return 10615;
        }
        if self.r#face == Face::Ceiling && self.r#powered == false && self.r#facing == Facing::East
        {
            return 10616;
        }
        if self.r#facing == Facing::East && self.r#face == Face::Floor && self.r#powered == true {
            return 10599;
        }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::North
        {
            return 10610;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10593 {
            return Some(CherryButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Floor,
            });
        }
        if state_id == 10603 {
            return Some(CherryButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10608 {
            return Some(CherryButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10611 {
            return Some(CherryButton {
                r#facing: Facing::South,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 10607 {
            return Some(CherryButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10609 {
            return Some(CherryButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10595 {
            return Some(CherryButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10602 {
            return Some(CherryButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10606 {
            return Some(CherryButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::West,
            });
        }
        if state_id == 10612 {
            return Some(CherryButton {
                r#facing: Facing::South,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10597 {
            return Some(CherryButton {
                r#facing: Facing::West,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 10614 {
            return Some(CherryButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10598 {
            return Some(CherryButton {
                r#powered: false,
                r#facing: Facing::West,
                r#face: Face::Floor,
            });
        }
        if state_id == 10600 {
            return Some(CherryButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10604 {
            return Some(CherryButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10601 {
            return Some(CherryButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10613 {
            return Some(CherryButton {
                r#face: Face::Ceiling,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10596 {
            return Some(CherryButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10594 {
            return Some(CherryButton {
                r#facing: Facing::North,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 10605 {
            return Some(CherryButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10615 {
            return Some(CherryButton {
                r#facing: Facing::East,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10616 {
            return Some(CherryButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 10599 {
            return Some(CherryButton {
                r#facing: Facing::East,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 10610 {
            return Some(CherryButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
