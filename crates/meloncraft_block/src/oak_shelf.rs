use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakShelf {
    pub r#facing: Facing,
    pub waterlogged: bool,
    pub r#side_chain: SideChain,
    pub powered: bool,
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

impl BlockState for OakShelf {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Left { return 2966; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2922; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false { return 2920; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right { return 2954; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 2914; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 2964; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#powered == true { return 2945; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 2953; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == true { return 2951; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#powered == true { return 2934; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true { return 2961; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 2939; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true { return 2931; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected { return 2967; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::North && block_state.r#powered == false { return 2923; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 2965; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true && block_state.r#powered == true { return 2947; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::West { return 2944; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center { return 2940; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left { return 2918; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false { return 2971; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true { return 2969; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right { return 2938; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 2950; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Left { return 2973; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false { return 2952; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::West { return 2958; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::North { return 2921; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == true { return 2963; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 2913; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false && block_state.r#waterlogged == true { return 2941; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North { return 2917; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 2928; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true { return 2929; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 2936; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false { return 2926; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 2968; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::East { return 2960; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right { return 2937; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left { return 2933; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center { return 2932; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 2927; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2970; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected { return 2911; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center { return 2956; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true && block_state.r#powered == false { return 2919; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false { return 2942; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 2935; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center { return 2924; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 2962; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#facing == Facing::West { return 2946; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true && block_state.r#powered == true { return 2915; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::West && block_state.r#powered == true { return 2949; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false { return 2974; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#waterlogged == true { return 2959; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center { return 2916; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left { return 2957; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Center { return 2955; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 2925; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::West { return 2943; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::South { return 2930; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center { return 2948; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false { return 2972; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected { return 2912; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2966 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2922 {
            return Some(OakShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2920 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
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
        if state_id == 2914 {
            return Some(OakShelf {
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 2964 {
            return Some(OakShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2945 {
            return Some(OakShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2953 {
            return Some(OakShelf {
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 2951 {
            return Some(OakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2934 {
            return Some(OakShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 2961 {
            return Some(OakShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2939 {
            return Some(OakShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2931 {
            return Some(OakShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2967 {
            return Some(OakShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2923 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 2965 {
            return Some(OakShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2947 {
            return Some(OakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2944 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2940 {
            return Some(OakShelf {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2918 {
            return Some(OakShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2971 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2969 {
            return Some(OakShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2938 {
            return Some(OakShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2950 {
            return Some(OakShelf {
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 2973 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2952 {
            return Some(OakShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2958 {
            return Some(OakShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 2921 {
            return Some(OakShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2963 {
            return Some(OakShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2913 {
            return Some(OakShelf {
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2941 {
            return Some(OakShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2917 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 2928 {
            return Some(OakShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2929 {
            return Some(OakShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2936 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2926 {
            return Some(OakShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 2968 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 2960 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2937 {
            return Some(OakShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2933 {
            return Some(OakShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2932 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2927 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2970 {
            return Some(OakShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2911 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2956 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2919 {
            return Some(OakShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2942 {
            return Some(OakShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2935 {
            return Some(OakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2924 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2962 {
            return Some(OakShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2946 {
            return Some(OakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2915 {
            return Some(OakShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2949 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 2974 {
            return Some(OakShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 2959 {
            return Some(OakShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2916 {
            return Some(OakShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2957 {
            return Some(OakShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2955 {
            return Some(OakShelf {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2925 {
            return Some(OakShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2943 {
            return Some(OakShelf {
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
            });
        }
        if state_id == 2930 {
            return Some(OakShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 2948 {
            return Some(OakShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2972 {
            return Some(OakShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 2912 {
            return Some(OakShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        return None;
    }
}

