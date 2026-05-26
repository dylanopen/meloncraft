use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceShelf {
    pub powered: bool,
    pub r#facing: Facing,
    pub r#side_chain: SideChain,
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
pub enum SideChain {
    Unconnected,
    Right,
    Center,
    Left,
}

impl BlockState for SpruceShelf {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#side_chain == SideChain::Center
        {
            return 3091;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 3049;
        }
        if self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
        {
            return 3062;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Unconnected
        {
            return 3071;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::East
        {
            return 3088;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::North
        {
            return 3044;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::South
        {
            return 3058;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::South
        {
            return 3064;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
        {
            return 3095;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Center
        {
            return 3067;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
        {
            return 3087;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 3072;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 3098;
        }
        if self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 3068;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 3102;
        }
        if self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Unconnected
        {
            return 3080;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
        {
            return 3051;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Center
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 3052;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 3063;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Center
            && self.r#powered == false
        {
            return 3084;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 3089;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 3090;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 3050;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 3069;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 3074;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 3040;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 3099;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 3094;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 3059;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 3039;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
        {
            return 3046;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
        {
            return 3055;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#side_chain == SideChain::Left
        {
            return 3101;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Unconnected
        {
            return 3056;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Left
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 3086;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Left
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 3078;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 3073;
        }
        if self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
        {
            return 3061;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 3070;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 3075;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 3057;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 3047;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
        {
            return 3065;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
            && self.r#powered == false
        {
            return 3066;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 3081;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 3041;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 3077;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
        {
            return 3083;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 3100;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Center
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 3043;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Left
            && self.r#powered == true
        {
            return 3093;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
        {
            return 3054;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Right
            && self.r#powered == true
        {
            return 3042;
        }
        if self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
        {
            return 3092;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::North
            && self.r#powered == false
        {
            return 3053;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 3079;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 3096;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Left
            && self.r#powered == false
        {
            return 3085;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 3082;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 3045;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 3060;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::West
        {
            return 3076;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 3097;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 3048;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3091 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3049 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 3062 {
            return Some(SpruceShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 3071 {
            return Some(SpruceShelf {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3088 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
            });
        }
        if state_id == 3044 {
            return Some(SpruceShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
            });
        }
        if state_id == 3058 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 3064 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
            });
        }
        if state_id == 3095 {
            return Some(SpruceShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 3067 {
            return Some(SpruceShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3087 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
            });
        }
        if state_id == 3072 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3098 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3068 {
            return Some(SpruceShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3102 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3080 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3051 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3052 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3063 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 3084 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 3089 {
            return Some(SpruceShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 3090 {
            return Some(SpruceShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 3050 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 3069 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 3074 {
            return Some(SpruceShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 3040 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 3099 {
            return Some(SpruceShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 3094 {
            return Some(SpruceShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 3059 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 3039 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3046 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3055 {
            return Some(SpruceShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 3101 {
            return Some(SpruceShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3056 {
            return Some(SpruceShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3086 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3078 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 3073 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 3061 {
            return Some(SpruceShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 3070 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 3075 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 3057 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 3047 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 3065 {
            return Some(SpruceShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 3066 {
            return Some(SpruceShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 3081 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 3041 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 3077 {
            return Some(SpruceShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 3083 {
            return Some(SpruceShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3100 {
            return Some(SpruceShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3043 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 3093 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#powered: true,
            });
        }
        if state_id == 3054 {
            return Some(SpruceShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 3042 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 3092 {
            return Some(SpruceShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 3053 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 3079 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 3096 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3085 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 3082 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3045 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 3060 {
            return Some(SpruceShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 3076 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
            });
        }
        if state_id == 3097 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 3048 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
