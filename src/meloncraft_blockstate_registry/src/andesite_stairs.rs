use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndesiteStairs {
    pub r#facing: Facing,
    pub r#half: Half,
    pub waterlogged: bool,
    pub r#shape: Shape,
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

impl BlockState for AndesiteStairs {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 15923;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
        {
            return 15930;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15929;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 15939;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
        {
            return 15940;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 15924;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 15933;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
        {
            return 15900;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
        {
            return 15942;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 15947;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 15915;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
        {
            return 15925;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 15956;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 15957;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 15914;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 15968;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 15969;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 15966;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 15944;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
        {
            return 15917;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 15927;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 15932;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
        {
            return 15901;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
        {
            return 15959;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 15905;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 15951;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 15936;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
        {
            return 15963;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
        {
            return 15935;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 15938;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
        {
            return 15896;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
        {
            return 15962;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 15897;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
        {
            return 15910;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
        {
            return 15902;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
        {
            return 15922;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 15894;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 15958;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
        {
            return 15943;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
        {
            return 15931;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 15906;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 15961;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
        {
            return 15909;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 15913;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
        {
            return 15941;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 15918;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 15908;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 15949;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 15921;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
        {
            return 15960;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 15899;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 15967;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 15903;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
        {
            return 15948;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 15895;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 15920;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
        {
            return 15928;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
        {
            return 15970;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 15953;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 15971;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15945;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
        {
            return 15954;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 15934;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::East
        {
            return 15964;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 15973;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
        {
            return 15937;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
        {
            return 15965;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
        {
            return 15911;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 15946;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
        {
            return 15907;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
        {
            return 15972;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
        {
            return 15950;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 15919;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 15912;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 15926;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 15952;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 15904;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
        {
            return 15898;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 15916;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
        {
            return 15955;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15923 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15930 {
            return Some(AndesiteStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15929 {
            return Some(AndesiteStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15939 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15940 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15924 {
            return Some(AndesiteStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15933 {
            return Some(AndesiteStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15900 {
            return Some(AndesiteStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15942 {
            return Some(AndesiteStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15947 {
            return Some(AndesiteStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15915 {
            return Some(AndesiteStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15925 {
            return Some(AndesiteStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15956 {
            return Some(AndesiteStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 15957 {
            return Some(AndesiteStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15914 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15968 {
            return Some(AndesiteStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15969 {
            return Some(AndesiteStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15966 {
            return Some(AndesiteStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15944 {
            return Some(AndesiteStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15917 {
            return Some(AndesiteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 15927 {
            return Some(AndesiteStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15932 {
            return Some(AndesiteStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15901 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15959 {
            return Some(AndesiteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15905 {
            return Some(AndesiteStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 15951 {
            return Some(AndesiteStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15936 {
            return Some(AndesiteStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15963 {
            return Some(AndesiteStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15935 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 15938 {
            return Some(AndesiteStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15896 {
            return Some(AndesiteStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 15962 {
            return Some(AndesiteStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15897 {
            return Some(AndesiteStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 15910 {
            return Some(AndesiteStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15902 {
            return Some(AndesiteStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15922 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 15894 {
            return Some(AndesiteStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15958 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15943 {
            return Some(AndesiteStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15931 {
            return Some(AndesiteStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15906 {
            return Some(AndesiteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15961 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15909 {
            return Some(AndesiteStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15913 {
            return Some(AndesiteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15941 {
            return Some(AndesiteStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15918 {
            return Some(AndesiteStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15908 {
            return Some(AndesiteStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15949 {
            return Some(AndesiteStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15921 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15960 {
            return Some(AndesiteStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 15899 {
            return Some(AndesiteStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15967 {
            return Some(AndesiteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15903 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15948 {
            return Some(AndesiteStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15895 {
            return Some(AndesiteStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15920 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15928 {
            return Some(AndesiteStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15970 {
            return Some(AndesiteStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 15953 {
            return Some(AndesiteStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15971 {
            return Some(AndesiteStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15945 {
            return Some(AndesiteStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15954 {
            return Some(AndesiteStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15934 {
            return Some(AndesiteStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15964 {
            return Some(AndesiteStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15973 {
            return Some(AndesiteStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15937 {
            return Some(AndesiteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15965 {
            return Some(AndesiteStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15911 {
            return Some(AndesiteStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15946 {
            return Some(AndesiteStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15907 {
            return Some(AndesiteStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15972 {
            return Some(AndesiteStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15950 {
            return Some(AndesiteStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15919 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15912 {
            return Some(AndesiteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15926 {
            return Some(AndesiteStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15952 {
            return Some(AndesiteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15904 {
            return Some(AndesiteStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 15898 {
            return Some(AndesiteStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 15916 {
            return Some(AndesiteStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15955 {
            return Some(AndesiteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        return None;
    }
}
