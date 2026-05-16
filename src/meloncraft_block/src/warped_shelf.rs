use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedShelf {
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

impl BlockState for WarpedShelf {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#side_chain == SideChain::Left { return 3150; }
        if self.r#powered == false && self.r#side_chain == SideChain::Right && self.r#facing == Facing::North && self.r#waterlogged == true { return 3113; }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#side_chain == SideChain::Unconnected && self.r#waterlogged == false { return 3104; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#powered == true && self.r#side_chain == SideChain::Left { return 3109; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#side_chain == SideChain::Right && self.r#waterlogged == false { return 3114; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#side_chain == SideChain::Center && self.r#waterlogged == true { return 3115; }
        if self.r#side_chain == SideChain::Left && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::South { return 3134; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#side_chain == SideChain::Right && self.r#waterlogged == false { return 3138; }
        if self.r#facing == Facing::East && self.r#side_chain == SideChain::Right && self.r#powered == false && self.r#waterlogged == false { return 3162; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#side_chain == SideChain::Left && self.r#facing == Facing::North { return 3110; }
        if self.r#powered == true && self.r#side_chain == SideChain::Right && self.r#facing == Facing::South && self.r#waterlogged == false { return 3122; }
        if self.r#side_chain == SideChain::Left && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#powered == true { return 3125; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#side_chain == SideChain::Center && self.r#powered == true { return 3107; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#side_chain == SideChain::Center { return 3123; }
        if self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::North && self.r#side_chain == SideChain::Unconnected { return 3111; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#side_chain == SideChain::Unconnected && self.r#waterlogged == false { return 3144; }
        if self.r#side_chain == SideChain::Unconnected && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#powered == false { return 3127; }
        if self.r#facing == Facing::East && self.r#side_chain == SideChain::Unconnected && self.r#powered == true && self.r#waterlogged == false { return 3152; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#powered == false && self.r#side_chain == SideChain::Center { return 3163; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#side_chain == SideChain::Left && self.r#powered == true { return 3141; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#powered == true && self.r#side_chain == SideChain::Center { return 3124; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#side_chain == SideChain::Right { return 3129; }
        if self.r#side_chain == SideChain::Center && self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == false { return 3116; }
        if self.r#facing == Facing::West && self.r#side_chain == SideChain::Center && self.r#waterlogged == false && self.r#powered == false { return 3148; }
        if self.r#side_chain == SideChain::Left && self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == true { return 3165; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#side_chain == SideChain::Left && self.r#waterlogged == true { return 3133; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#powered == false && self.r#side_chain == SideChain::Unconnected { return 3159; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == false && self.r#side_chain == SideChain::Unconnected { return 3112; }
        if self.r#facing == Facing::West && self.r#side_chain == SideChain::Center && self.r#waterlogged == true && self.r#powered == true { return 3139; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#side_chain == SideChain::Center && self.r#facing == Facing::East { return 3155; }
        if self.r#powered == true && self.r#side_chain == SideChain::Right && self.r#facing == Facing::East && self.r#waterlogged == true { return 3153; }
        if self.r#powered == true && self.r#side_chain == SideChain::Center && self.r#facing == Facing::North && self.r#waterlogged == false { return 3108; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#side_chain == SideChain::Left && self.r#powered == false { return 3117; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#powered == false && self.r#side_chain == SideChain::Unconnected { return 3128; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#side_chain == SideChain::Left && self.r#facing == Facing::West { return 3142; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#side_chain == SideChain::Unconnected { return 3160; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#powered == false && self.r#side_chain == SideChain::Center { return 3132; }
        if self.r#facing == Facing::West && self.r#side_chain == SideChain::Unconnected && self.r#powered == false && self.r#waterlogged == true { return 3143; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#powered == true && self.r#side_chain == SideChain::Right { return 3105; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#side_chain == SideChain::Right && self.r#powered == true { return 3154; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#side_chain == SideChain::Unconnected && self.r#facing == Facing::North { return 3103; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#side_chain == SideChain::Right && self.r#powered == false { return 3130; }
        if self.r#powered == false && self.r#side_chain == SideChain::Center && self.r#waterlogged == true && self.r#facing == Facing::South { return 3131; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#side_chain == SideChain::Center && self.r#waterlogged == false { return 3156; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#side_chain == SideChain::Left && self.r#powered == true { return 3158; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#side_chain == SideChain::Unconnected && self.r#waterlogged == false { return 3120; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#waterlogged == false && self.r#side_chain == SideChain::Center { return 3140; }
        if self.r#waterlogged == true && self.r#side_chain == SideChain::Left && self.r#facing == Facing::East && self.r#powered == true { return 3157; }
        if self.r#facing == Facing::East && self.r#side_chain == SideChain::Center && self.r#waterlogged == false && self.r#powered == false { return 3164; }
        if self.r#side_chain == SideChain::Unconnected && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#powered == true { return 3119; }
        if self.r#side_chain == SideChain::Left && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#powered == false { return 3149; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#powered == true && self.r#side_chain == SideChain::Right { return 3137; }
        if self.r#side_chain == SideChain::Unconnected && self.r#powered == true && self.r#facing == Facing::West && self.r#waterlogged == true { return 3135; }
        if self.r#powered == false && self.r#side_chain == SideChain::Right && self.r#waterlogged == true && self.r#facing == Facing::East { return 3161; }
        if self.r#waterlogged == false && self.r#side_chain == SideChain::Left && self.r#powered == false && self.r#facing == Facing::East { return 3166; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#side_chain == SideChain::Unconnected && self.r#waterlogged == false { return 3136; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#side_chain == SideChain::Center && self.r#powered == false { return 3147; }
        if self.r#side_chain == SideChain::Right && self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::West { return 3145; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#side_chain == SideChain::Unconnected { return 3151; }
        if self.r#waterlogged == false && self.r#side_chain == SideChain::Right && self.r#facing == Facing::North && self.r#powered == true { return 3106; }
        if self.r#side_chain == SideChain::Left && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#powered == false { return 3118; }
        if self.r#waterlogged == false && self.r#side_chain == SideChain::Right && self.r#powered == false && self.r#facing == Facing::West { return 3146; }
        if self.r#side_chain == SideChain::Right && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#powered == true { return 3121; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#waterlogged == false && self.r#side_chain == SideChain::Left { return 3126; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3150 {
            return Some(WarpedShelf {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3113 {
            return Some(WarpedShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 3104 {
            return Some(WarpedShelf {
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 3109 {
            return Some(WarpedShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3114 {
            return Some(WarpedShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 3115 {
            return Some(WarpedShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 3134 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 3138 {
            return Some(WarpedShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 3162 {
            return Some(WarpedShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3110 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 3122 {
            return Some(WarpedShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 3125 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 3107 {
            return Some(WarpedShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 3123 {
            return Some(WarpedShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3111 {
            return Some(WarpedShelf {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3144 {
            return Some(WarpedShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 3127 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 3152 {
            return Some(WarpedShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 3163 {
            return Some(WarpedShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3141 {
            return Some(WarpedShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: true,
            });
        }
        if state_id == 3124 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3129 {
            return Some(WarpedShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3116 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3148 {
            return Some(WarpedShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3165 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 3133 {
            return Some(WarpedShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 3159 {
            return Some(WarpedShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3112 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3139 {
            return Some(WarpedShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 3155 {
            return Some(WarpedShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
            });
        }
        if state_id == 3153 {
            return Some(WarpedShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 3108 {
            return Some(WarpedShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 3117 {
            return Some(WarpedShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 3128 {
            return Some(WarpedShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3142 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 3160 {
            return Some(WarpedShelf {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3132 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3143 {
            return Some(WarpedShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 3105 {
            return Some(WarpedShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3154 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 3103 {
            return Some(WarpedShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
            });
        }
        if state_id == 3130 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 3131 {
            return Some(WarpedShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 3156 {
            return Some(WarpedShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 3158 {
            return Some(WarpedShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#powered: true,
            });
        }
        if state_id == 3120 {
            return Some(WarpedShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 3140 {
            return Some(WarpedShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3157 {
            return Some(WarpedShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 3164 {
            return Some(WarpedShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3119 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 3149 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 3137 {
            return Some(WarpedShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3135 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 3161 {
            return Some(WarpedShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 3166 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 3136 {
            return Some(WarpedShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 3147 {
            return Some(WarpedShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 3145 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 3151 {
            return Some(WarpedShelf {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3106 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 3118 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3146 {
            return Some(WarpedShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 3121 {
            return Some(WarpedShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 3126 {
            return Some(WarpedShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        return None;
    }
}

