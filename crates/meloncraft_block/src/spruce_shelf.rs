use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceShelf {
    pub waterlogged: bool,
    pub powered: bool,
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

impl BlockState for SpruceShelf {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Left { return 3045; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center { return 3052; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Left && block_state.r#powered == true { return 3094; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 3101; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false { return 3086; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left { return 3061; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 3049; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right { return 3097; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true { return 3040; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false { return 3102; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center { return 3043; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false { return 3080; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true { return 3051; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 3042; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == false { return 3048; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 3059; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Center { return 3075; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 3077; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 3078; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 3091; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 3066; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 3093; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East { return 3098; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East { return 3092; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true { return 3057; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Unconnected { return 3088; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South { return 3070; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false { return 3044; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#waterlogged == false { return 3090; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 3068; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::North { return 3050; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Unconnected { return 3087; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 3076; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#facing == Facing::West { return 3071; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == false { return 3082; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East { return 3100; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true { return 3083; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false { return 3099; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false { return 3081; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::North { return 3041; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true { return 3067; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right { return 3074; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North { return 3053; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right { return 3065; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 3084; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false { return 3062; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 3054; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left { return 3046; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 3047; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left { return 3069; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Unconnected { return 3095; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected { return 3039; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 3058; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == false { return 3064; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 3055; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false { return 3060; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 3085; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right { return 3073; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Unconnected { return 3056; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::South && block_state.r#powered == false { return 3063; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected { return 3079; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 3096; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 3089; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true { return 3072; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3045 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3052 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3094 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#powered: true,
            });
        }
        if state_id == 3101 {
            return Some(SpruceShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 3086 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 3061 {
            return Some(SpruceShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3049 {
            return Some(SpruceShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 3097 {
            return Some(SpruceShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3040 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
            });
        }
        if state_id == 3102 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3043 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3080 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 3051 {
            return Some(SpruceShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 3042 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 3048 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3059 {
            return Some(SpruceShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 3075 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3077 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 3078 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 3091 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 3066 {
            return Some(SpruceShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 3093 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 3098 {
            return Some(SpruceShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 3092 {
            return Some(SpruceShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
            });
        }
        if state_id == 3057 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 3088 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3070 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 3044 {
            return Some(SpruceShelf {
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 3090 {
            return Some(SpruceShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 3068 {
            return Some(SpruceShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 3050 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 3087 {
            return Some(SpruceShelf {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3076 {
            return Some(SpruceShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 3071 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 3082 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3100 {
            return Some(SpruceShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
            });
        }
        if state_id == 3083 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 3099 {
            return Some(SpruceShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 3081 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 3041 {
            return Some(SpruceShelf {
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 3067 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 3074 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3053 {
            return Some(SpruceShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 3065 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3084 {
            return Some(SpruceShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#facing: Facing::West,
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
        if state_id == 3054 {
            return Some(SpruceShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 3046 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3047 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 3069 {
            return Some(SpruceShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3095 {
            return Some(SpruceShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3039 {
            return Some(SpruceShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3058 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 3064 {
            return Some(SpruceShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3055 {
            return Some(SpruceShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3060 {
            return Some(SpruceShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 3085 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 3073 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3056 {
            return Some(SpruceShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3063 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 3079 {
            return Some(SpruceShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3096 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 3089 {
            return Some(SpruceShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 3072 {
            return Some(SpruceShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
            });
        }
        return None;
    }
}

