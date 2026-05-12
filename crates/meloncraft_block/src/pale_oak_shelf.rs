use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakShelf {
    pub waterlogged: bool,
    pub powered: bool,
    pub r#side_chain: SideChain,
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

impl BlockState for PaleOakShelf {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false && block_state.r#powered == false { return 2988; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East && block_state.r#powered == true { return 3025; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West { return 3009; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::North && block_state.r#powered == true { return 2980; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 2987; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#waterlogged == false { return 2992; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true { return 3010; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#waterlogged == false { return 3014; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Unconnected { return 3016; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false && block_state.r#waterlogged == true { return 3021; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 3028; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false { return 3030; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right { return 2993; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == true { return 2989; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected { return 2975; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2986; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center { return 2979; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 2976; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false { return 3032; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == true { return 3031; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right { return 2985; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center { return 3012; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left { return 2998; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false && block_state.r#waterlogged == false { return 3022; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 3023; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left { return 3037; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false { return 3038; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Left { return 2981; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == true { return 3008; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#waterlogged == true { return 3007; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right { return 2994; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Center { return 3011; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Center { return 3019; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false && block_state.r#powered == false { return 3034; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 2991; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#waterlogged == true { return 2977; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Left { return 2982; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == false { return 3026; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left { return 2997; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true { return 2996; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center { return 3020; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 3004; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == false { return 3000; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#waterlogged == false { return 3002; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false { return 2984; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 3027; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 2999; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center { return 3003; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South { return 3006; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#facing == Facing::West { return 3013; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 2983; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true { return 3005; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 3024; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 3001; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false { return 3015; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#powered == false { return 3033; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#facing == Facing::East { return 3036; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Right { return 3018; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::East { return 3029; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 3017; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 2995; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East { return 3035; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#powered == false { return 2990; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right { return 2978; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2988 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 3025 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 3009 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 2980 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 2987 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2992 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 3010 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 3014 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 3016 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3021 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 3028 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 3030 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2993 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2989 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2975 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2986 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2979 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2976 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 3032 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 3031 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2985 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3012 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2998 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3022 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3023 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 3037 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3038 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2981 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3008 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 3007 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2994 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3011 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3019 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3034 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2991 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2977 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2982 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 3026 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2997 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2996 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 3020 {
            return Some(PaleOakShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3004 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 3000 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 3002 {
            return Some(PaleOakShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2984 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 3027 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 2999 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 3003 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 3006 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 3013 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2983 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 3005 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 3024 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 3001 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
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
        if state_id == 3033 {
            return Some(PaleOakShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 3036 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 3018 {
            return Some(PaleOakShelf {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 3029 {
            return Some(PaleOakShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 3017 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 2995 {
            return Some(PaleOakShelf {
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 3035 {
            return Some(PaleOakShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
            });
        }
        if state_id == 2990 {
            return Some(PaleOakShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2978 {
            return Some(PaleOakShelf {
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        return None;
    }
}

