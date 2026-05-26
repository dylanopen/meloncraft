use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandstoneStairs {
    pub r#facing: Facing,
    pub r#half: Half,
    pub r#shape: Shape,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

impl BlockState for SandstoneStairs {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
        {
            return 9347;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
        {
            return 9352;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
        {
            return 9364;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 9359;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 9367;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 9366;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 9365;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
        {
            return 9329;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 9363;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 9333;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 9338;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 9296;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
        {
            return 9356;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 9292;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 9340;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 9369;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 9298;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 9346;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 9337;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 9311;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 9336;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
        {
            return 9294;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 9310;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 9332;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 9361;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 9345;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 9342;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 9321;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
        {
            return 9315;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 9334;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 9350;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 9351;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 9371;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
        {
            return 9354;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 9344;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 9360;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 9370;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 9319;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
        {
            return 9308;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
        {
            return 9300;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 9303;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 9318;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
        {
            return 9305;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 9323;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 9355;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 9306;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 9312;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 9317;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 9304;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 9293;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
        {
            return 9295;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
        {
            return 9307;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 9314;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 9343;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 9328;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 9313;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
        {
            return 9299;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 9353;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 9324;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
        {
            return 9331;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 9339;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerLeft
        {
            return 9325;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 9322;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
        {
            return 9335;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
        {
            return 9330;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
        {
            return 9341;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 9348;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
        {
            return 9326;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
        {
            return 9327;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 9358;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 9301;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
        {
            return 9309;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 9297;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 9349;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 9357;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 9316;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 9362;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 9320;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 9302;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 9368;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9347 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9352 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9364 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9359 {
            return Some(SandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 9367 {
            return Some(SandstoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 9366 {
            return Some(SandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9365 {
            return Some(SandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9329 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9363 {
            return Some(SandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9333 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 9338 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 9296 {
            return Some(SandstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 9356 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 9292 {
            return Some(SandstoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 9340 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 9369 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 9298 {
            return Some(SandstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 9346 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9337 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 9311 {
            return Some(SandstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9336 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 9294 {
            return Some(SandstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 9310 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9332 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 9361 {
            return Some(SandstoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 9345 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9342 {
            return Some(SandstoneStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9321 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9315 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 9334 {
            return Some(SandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 9350 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 9351 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9371 {
            return Some(SandstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9354 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 9344 {
            return Some(SandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9360 {
            return Some(SandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 9370 {
            return Some(SandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9319 {
            return Some(SandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9308 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 9300 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9303 {
            return Some(SandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 9318 {
            return Some(SandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9305 {
            return Some(SandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 9323 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 9355 {
            return Some(SandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9306 {
            return Some(SandstoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9312 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 9317 {
            return Some(SandstoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 9304 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9293 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 9295 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 9307 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 9314 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9343 {
            return Some(SandstoneStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 9328 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9313 {
            return Some(SandstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 9299 {
            return Some(SandstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9353 {
            return Some(SandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 9324 {
            return Some(SandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9331 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9339 {
            return Some(SandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 9325 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9322 {
            return Some(SandstoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9335 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 9330 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9341 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 9348 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 9326 {
            return Some(SandstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9327 {
            return Some(SandstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9358 {
            return Some(SandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 9301 {
            return Some(SandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 9309 {
            return Some(SandstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9297 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 9349 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9357 {
            return Some(SandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 9316 {
            return Some(SandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 9362 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9320 {
            return Some(SandstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9302 {
            return Some(SandstoneStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9368 {
            return Some(SandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
