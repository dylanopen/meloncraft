use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedTrapdoor {
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

impl BlockState for WarpedTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 21006;
        }
        if self.r#open == false
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 20981;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 20992;
        }
        if self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#half == Half::Top
        {
            return 21014;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 20979;
        }
        if self.r#open == true
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 20987;
        }
        if self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#facing == Facing::East
        {
            return 21028;
        }
        if self.r#facing == Facing::North
            && self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 20983;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::South
        {
            return 20999;
        }
        if self.r#powered == false
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 21026;
        }
        if self.r#open == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#half == Half::Bottom
        {
            return 20985;
        }
        if self.r#facing == Facing::North
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 20986;
        }
        if self.r#facing == Facing::South
            && self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 21001;
        }
        if self.r#half == Half::Top
            && self.r#powered == false
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 21010;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::East
        {
            return 21033;
        }
        if self.r#waterlogged == true
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 20994;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == true
        {
            return 21024;
        }
        if self.r#powered == false
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 21011;
        }
        if self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#half == Half::Bottom
        {
            return 21002;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::West
        {
            return 21021;
        }
        if self.r#powered == true
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 20997;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
        {
            return 20989;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#open == false
        {
            return 20988;
        }
        if self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#half == Half::Bottom
        {
            return 20991;
        }
        if self.r#waterlogged == true
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#powered == false
        {
            return 21022;
        }
        if self.r#open == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#powered == true
        {
            return 20976;
        }
        if self.r#powered == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#open == true
            && self.r#waterlogged == true
        {
            return 20978;
        }
        if self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#waterlogged == true
        {
            return 21008;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == true
        {
            return 21017;
        }
        if self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#open == false
        {
            return 21007;
        }
        if self.r#facing == Facing::North
            && self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 20984;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Top
        {
            return 20993;
        }
        if self.r#open == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 21013;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#facing == Facing::South
        {
            return 21003;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 20982;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Bottom
        {
            return 21036;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#facing == Facing::East
        {
            return 21039;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 21012;
        }
        if self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 21032;
        }
        if self.r#open == true
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 21016;
        }
        if self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 21038;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#half == Half::Top
            && self.r#open == true
        {
            return 21025;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == true
        {
            return 21005;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 21018;
        }
        if self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 21035;
        }
        if self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 20996;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 21037;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Top
        {
            return 20980;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#open == false
        {
            return 21004;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 21031;
        }
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 20998;
        }
        if self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#open == true
        {
            return 21000;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 21027;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::West
        {
            return 21019;
        }
        if self.r#powered == false
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#facing == Facing::South
        {
            return 20995;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#facing == Facing::East
        {
            return 21034;
        }
        if self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::North
        {
            return 20990;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#open == false
        {
            return 21015;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 21030;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 20977;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#powered == true
        {
            return 21009;
        }
        if self.r#half == Half::Top
            && self.r#open == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#powered == true
        {
            return 21029;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#facing == Facing::West
        {
            return 21023;
        }
        if self.r#open == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#powered == true
        {
            return 21020;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21006 {
            return Some(WarpedTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20981 {
            return Some(WarpedTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 20992 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21014 {
            return Some(WarpedTrapdoor {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 20979 {
            return Some(WarpedTrapdoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 20987 {
            return Some(WarpedTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 21028 {
            return Some(WarpedTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 20983 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 20999 {
            return Some(WarpedTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21026 {
            return Some(WarpedTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 20985 {
            return Some(WarpedTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 20986 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21001 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21010 {
            return Some(WarpedTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 21033 {
            return Some(WarpedTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 20994 {
            return Some(WarpedTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 21024 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 21011 {
            return Some(WarpedTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 21002 {
            return Some(WarpedTrapdoor {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21021 {
            return Some(WarpedTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 20997 {
            return Some(WarpedTrapdoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 20989 {
            return Some(WarpedTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 20988 {
            return Some(WarpedTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 20991 {
            return Some(WarpedTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21022 {
            return Some(WarpedTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 20976 {
            return Some(WarpedTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 20978 {
            return Some(WarpedTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21008 {
            return Some(WarpedTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21017 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 21007 {
            return Some(WarpedTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 20984 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 20993 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21013 {
            return Some(WarpedTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21003 {
            return Some(WarpedTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 20982 {
            return Some(WarpedTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 21036 {
            return Some(WarpedTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21039 {
            return Some(WarpedTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 21012 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21032 {
            return Some(WarpedTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21016 {
            return Some(WarpedTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 21038 {
            return Some(WarpedTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 21025 {
            return Some(WarpedTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 21005 {
            return Some(WarpedTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 21018 {
            return Some(WarpedTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 21035 {
            return Some(WarpedTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 20996 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 21037 {
            return Some(WarpedTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 20980 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21004 {
            return Some(WarpedTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 21031 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20998 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 21000 {
            return Some(WarpedTrapdoor {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 21027 {
            return Some(WarpedTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 21019 {
            return Some(WarpedTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 20995 {
            return Some(WarpedTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 21034 {
            return Some(WarpedTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 20990 {
            return Some(WarpedTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 21015 {
            return Some(WarpedTrapdoor {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 21030 {
            return Some(WarpedTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 20977 {
            return Some(WarpedTrapdoor {
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 21009 {
            return Some(WarpedTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 21029 {
            return Some(WarpedTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 21023 {
            return Some(WarpedTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21020 {
            return Some(WarpedTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        return None;
    }
}
