use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveShelf {
    pub powered: bool,
    pub r#side_chain: SideChain,
    pub waterlogged: bool,
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

impl BlockState for MangroveShelf {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 2875; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left { return 2885; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false { return 2909; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 2849; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == true { return 2891; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == true { return 2864; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Center { return 2884; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true { return 2859; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 2848; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true { return 2881; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected { return 2895; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East { return 2900; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true { return 2901; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right { return 2873; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::East { return 2902; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true && block_state.r#powered == false { return 2871; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == true { return 2880; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false { return 2874; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == true { return 2854; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 2865; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South && block_state.r#powered == false { return 2877; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::North { return 2856; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false && block_state.r#powered == false { return 2858; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 2907; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == false { return 2878; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left { return 2853; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#waterlogged == true { return 2889; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::South { return 2876; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East && block_state.r#powered == false { return 2903; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Unconnected { return 2887; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center { return 2883; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right { return 2905; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::North { return 2847; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center { return 2867; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 2863; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::East { return 2896; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true { return 2868; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::North { return 2855; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 2879; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left { return 2869; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false && block_state.r#powered == true { return 2852; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false && block_state.r#waterlogged == true { return 2893; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right { return 2898; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::West { return 2892; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true && block_state.r#powered == true { return 2851; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected { return 2872; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Left { return 2861; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == true { return 2886; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left { return 2862; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true { return 2882; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == false { return 2890; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == false { return 2860; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true { return 2850; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == true { return 2897; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 2910; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true { return 2857; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#waterlogged == false { return 2906; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == false { return 2870; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2866; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center { return 2899; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 2908; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false { return 2894; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false { return 2888; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East { return 2904; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2875 {
            return Some(MangroveShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2885 {
            return Some(MangroveShelf {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2909 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2849 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2891 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2864 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2884 {
            return Some(MangroveShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2859 {
            return Some(MangroveShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2848 {
            return Some(MangroveShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 2881 {
            return Some(MangroveShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2895 {
            return Some(MangroveShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2900 {
            return Some(MangroveShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
            });
        }
        if state_id == 2901 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2873 {
            return Some(MangroveShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2902 {
            return Some(MangroveShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 2871 {
            return Some(MangroveShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2880 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 2874 {
            return Some(MangroveShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 2854 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2865 {
            return Some(MangroveShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2877 {
            return Some(MangroveShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2856 {
            return Some(MangroveShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
            });
        }
        if state_id == 2858 {
            return Some(MangroveShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2907 {
            return Some(MangroveShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2878 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2853 {
            return Some(MangroveShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2889 {
            return Some(MangroveShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2876 {
            return Some(MangroveShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
            });
        }
        if state_id == 2903 {
            return Some(MangroveShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 2887 {
            return Some(MangroveShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2883 {
            return Some(MangroveShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2905 {
            return Some(MangroveShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2847 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2867 {
            return Some(MangroveShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2863 {
            return Some(MangroveShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2896 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2868 {
            return Some(MangroveShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 2855 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2879 {
            return Some(MangroveShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2869 {
            return Some(MangroveShelf {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2852 {
            return Some(MangroveShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2893 {
            return Some(MangroveShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2898 {
            return Some(MangroveShelf {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2892 {
            return Some(MangroveShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
            });
        }
        if state_id == 2851 {
            return Some(MangroveShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2872 {
            return Some(MangroveShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2861 {
            return Some(MangroveShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2886 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 2862 {
            return Some(MangroveShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2882 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2890 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2860 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2850 {
            return Some(MangroveShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2897 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2910 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2857 {
            return Some(MangroveShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2906 {
            return Some(MangroveShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2870 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2866 {
            return Some(MangroveShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2899 {
            return Some(MangroveShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2908 {
            return Some(MangroveShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2894 {
            return Some(MangroveShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2888 {
            return Some(MangroveShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 2904 {
            return Some(MangroveShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

