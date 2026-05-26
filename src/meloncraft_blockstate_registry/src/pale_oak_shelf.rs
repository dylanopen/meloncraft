use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakShelf {
    pub powered: bool,
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#side_chain: SideChain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideChain {
    Unconnected,
    Right,
    Center,
    Left,
}

impl BlockState for PaleOakShelf {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
        {
            return 3013;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Center
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 3036;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Center
        {
            return 3028;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
        {
            return 3015;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 3000;
        }
        if self.r#waterlogged == true
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::West
        {
            return 3019;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 3031;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 3020;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 3007;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 2987;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
        {
            return 2981;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 3038;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 2994;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 3014;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 2984;
        }
        if self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 3006;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 3029;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2992;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 3004;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::East
            && self.r#powered == true
        {
            return 3030;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2993;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
        {
            return 3001;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
        {
            return 2996;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
        {
            return 2990;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
        {
            return 2988;
        }
        if self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 2991;
        }
        if self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
        {
            return 3016;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 2976;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 2998;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#powered == true
        {
            return 3025;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::West
        {
            return 3021;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 3003;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 2982;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::North
        {
            return 2980;
        }
        if self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2983;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 2975;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
        {
            return 3010;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 3024;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2999;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#side_chain == SideChain::Left
        {
            return 2989;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::East
        {
            return 3026;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 3033;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
        {
            return 3032;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 3034;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 2986;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 2995;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
        {
            return 3018;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
        {
            return 3023;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 3022;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::South
        {
            return 3002;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
        {
            return 3035;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
        {
            return 2997;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 3012;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 2979;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::West
            && self.r#powered == true
        {
            return 3008;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 2977;
        }
        if self.r#waterlogged == true
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::West
        {
            return 3009;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#side_chain == SideChain::Left
        {
            return 3005;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 3037;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#side_chain == SideChain::Right
        {
            return 3017;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 2978;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
        {
            return 2985;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
            && self.r#powered == true
        {
            return 3027;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#side_chain == SideChain::Center
        {
            return 3011;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3013 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3036 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3028 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3015 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 3000 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 3019 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
            });
        }
        if state_id == 3031 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 3020 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 3007 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2987 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2981 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3038 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2994 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 3014 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2984 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 3006 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3029 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2992 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3004 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 3030 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 2993 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 3001 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2996 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 2990 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2988 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2991 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 3016 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2976 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2998 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 3025 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 3021 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 3003 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2982 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2980 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
            });
        }
        if state_id == 2983 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2975 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 3010 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3024 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2999 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2989 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3026 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 3033 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 3032 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 3034 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2986 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2995 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 3018 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3023 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 3022 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3002 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 3035 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2997 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 3012 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2979 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 3008 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 2977 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 3009 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 3005 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3037 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 3017 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2978 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2985 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 3027 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 3011 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        return None;
    }
}
