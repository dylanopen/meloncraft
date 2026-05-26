use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakShelf {
    pub r#side_chain: SideChain,
    pub waterlogged: bool,
    pub powered: bool,
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideChain {
    Unconnected,
    Right,
    Center,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for OakShelf {
    fn to_id(&self) -> i32 {
        if self.r#powered == false
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 2970;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
        {
            return 2952;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
        {
            return 2955;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
        {
            return 2971;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 2957;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 2944;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#powered == false
        {
            return 2919;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
        {
            return 2937;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Center
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 2956;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 2933;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 2973;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 2914;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
            && self.r#facing == Facing::West
        {
            return 2951;
        }
        if self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
        {
            return 2958;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 2935;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 2913;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 2915;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
        {
            return 2938;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 2941;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2945;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
        {
            return 2946;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2959;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 2967;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 2936;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
        {
            return 2929;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2943;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
        {
            return 2934;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
            && self.r#powered == true
            && self.r#facing == Facing::East
        {
            return 2966;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::East
        {
            return 2972;
        }
        if self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
        {
            return 2948;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 2926;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::East
        {
            return 2961;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 2920;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 2928;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
        {
            return 2962;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 2965;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::East
        {
            return 2974;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 2960;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
        {
            return 2954;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
        {
            return 2942;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
        {
            return 2912;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
        {
            return 2921;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 2969;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 2924;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::West
        {
            return 2947;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Left
        {
            return 2950;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::West
        {
            return 2949;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 2916;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 2963;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::North
        {
            return 2922;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
        {
            return 2930;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Center
        {
            return 2964;
        }
        if self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Center
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 2931;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 2953;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
        {
            return 2940;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2917;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 2923;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::North
        {
            return 2925;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
        {
            return 2932;
        }
        if self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 2939;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
        {
            return 2918;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2968;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 2927;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
        {
            return 2911;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2970 {
            return Some(OakShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2952 {
            return Some(OakShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2955 {
            return Some(OakShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2971 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2957 {
            return Some(OakShelf {
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 2944 {
            return Some(OakShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 2919 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 2937 {
            return Some(OakShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2956 {
            return Some(OakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2933 {
            return Some(OakShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2973 {
            return Some(OakShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2914 {
            return Some(OakShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2951 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2958 {
            return Some(OakShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2935 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2913 {
            return Some(OakShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 2915 {
            return Some(OakShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 2938 {
            return Some(OakShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2941 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2945 {
            return Some(OakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2946 {
            return Some(OakShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2959 {
            return Some(OakShelf {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2967 {
            return Some(OakShelf {
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2936 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2929 {
            return Some(OakShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2943 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2934 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2966 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2972 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
            });
        }
        if state_id == 2948 {
            return Some(OakShelf {
                r#powered: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 2926 {
            return Some(OakShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2961 {
            return Some(OakShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2920 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2928 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 2962 {
            return Some(OakShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2965 {
            return Some(OakShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2974 {
            return Some(OakShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2960 {
            return Some(OakShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2954 {
            return Some(OakShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2942 {
            return Some(OakShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2912 {
            return Some(OakShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2921 {
            return Some(OakShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2969 {
            return Some(OakShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2924 {
            return Some(OakShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2947 {
            return Some(OakShelf {
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
            });
        }
        if state_id == 2950 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2949 {
            return Some(OakShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2916 {
            return Some(OakShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2963 {
            return Some(OakShelf {
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2922 {
            return Some(OakShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 2930 {
            return Some(OakShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2964 {
            return Some(OakShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2931 {
            return Some(OakShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2953 {
            return Some(OakShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2940 {
            return Some(OakShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
            });
        }
        if state_id == 2917 {
            return Some(OakShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2923 {
            return Some(OakShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2925 {
            return Some(OakShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2932 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
            });
        }
        if state_id == 2939 {
            return Some(OakShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2918 {
            return Some(OakShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2968 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2927 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2911 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
            });
        }
        return None;
    }
}
