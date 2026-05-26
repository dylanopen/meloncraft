use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleButton {
    pub powered: bool,
    pub r#face: Face,
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

impl BlockState for JungleButton {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#powered == false && self.r#face == Face::Floor {
            return 10550;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::South && self.r#powered == true {
            return 10555;
        }
        if self.r#facing == Facing::North && self.r#face == Face::Ceiling && self.r#powered == false
        {
            return 10562;
        }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Wall {
            return 10560;
        }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::South
        {
            return 10564;
        }
        if self.r#powered == false && self.r#face == Face::Ceiling && self.r#facing == Facing::West
        {
            return 10566;
        }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Wall {
            return 10554;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == true {
            return 10549;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::North && self.r#powered == true {
            return 10553;
        }
        if self.r#powered == false && self.r#face == Face::Floor && self.r#facing == Facing::East {
            return 10552;
        }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Floor {
            return 10551;
        }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::West {
            return 10565;
        }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Ceiling
        {
            return 10568;
        }
        if self.r#powered == true && self.r#face == Face::Floor && self.r#facing == Facing::South {
            return 10547;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::West && self.r#powered == true {
            return 10557;
        }
        if self.r#facing == Facing::East && self.r#face == Face::Wall && self.r#powered == true {
            return 10559;
        }
        if self.r#powered == true && self.r#face == Face::Floor && self.r#facing == Facing::North {
            return 10545;
        }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::South {
            return 10556;
        }
        if self.r#facing == Facing::North && self.r#face == Face::Ceiling && self.r#powered == true
        {
            return 10561;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::West && self.r#powered == false {
            return 10558;
        }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::South
        {
            return 10563;
        }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Ceiling {
            return 10567;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::South && self.r#powered == false {
            return 10548;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::North && self.r#powered == false {
            return 10546;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10550 {
            return Some(JungleButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10555 {
            return Some(JungleButton {
                r#face: Face::Wall,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10562 {
            return Some(JungleButton {
                r#facing: Facing::North,
                r#face: Face::Ceiling,
                r#powered: false,
            });
        }
        if state_id == 10560 {
            return Some(JungleButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10564 {
            return Some(JungleButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 10566 {
            return Some(JungleButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 10554 {
            return Some(JungleButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10549 {
            return Some(JungleButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10553 {
            return Some(JungleButton {
                r#face: Face::Wall,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10552 {
            return Some(JungleButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::East,
            });
        }
        if state_id == 10551 {
            return Some(JungleButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Floor,
            });
        }
        if state_id == 10565 {
            return Some(JungleButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 10568 {
            return Some(JungleButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10547 {
            return Some(JungleButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::South,
            });
        }
        if state_id == 10557 {
            return Some(JungleButton {
                r#face: Face::Wall,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10559 {
            return Some(JungleButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10545 {
            return Some(JungleButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 10556 {
            return Some(JungleButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 10561 {
            return Some(JungleButton {
                r#facing: Facing::North,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 10558 {
            return Some(JungleButton {
                r#face: Face::Wall,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10563 {
            return Some(JungleButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 10567 {
            return Some(JungleButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10548 {
            return Some(JungleButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10546 {
            return Some(JungleButton {
                r#face: Face::Floor,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        return None;
    }
}
