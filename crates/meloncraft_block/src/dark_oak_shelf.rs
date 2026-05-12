use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakShelf {
    pub r#facing: Facing,
    pub waterlogged: bool,
    pub powered: bool,
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

impl BlockState for DarkOakShelf {
    fn to_id(self) -> i32 {
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == false { return 2732; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true { return 2739; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West { return 2754; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 2719; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == false { return 2745; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::West { return 2752; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center { return 2731; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 2747; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 2761; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#facing == Facing::North { return 2722; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 2759; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 2742; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left { return 2782; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center { return 2755; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected { return 2727; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South && block_state.r#powered == false { return 2750; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Left { return 2781; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false { return 2728; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 2729; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true && block_state.r#facing == Facing::North { return 2723; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center { return 2772; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true { return 2740; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#powered == true { return 2751; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false { return 2760; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right { return 2738; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right { return 2753; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#waterlogged == true { return 2779; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 2736; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left { return 2726; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true { return 2735; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::West { return 2763; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#waterlogged == true { return 2773; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == true { return 2758; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false { return 2720; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left { return 2749; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left { return 2765; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 2734; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right { return 2737; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#powered == false { return 2744; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right { return 2721; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::West { return 2764; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center { return 2748; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East { return 2769; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false { return 2746; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false { return 2724; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 2733; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East && block_state.r#powered == false { return 2776; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 2777; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East { return 2770; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#facing == Facing::East { return 2778; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == false { return 2768; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left { return 2774; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#powered == false { return 2766; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center { return 2756; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == true { return 2767; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center { return 2780; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#facing == Facing::East { return 2775; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#powered == true { return 2741; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 2762; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected { return 2743; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == false { return 2730; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 2757; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true { return 2725; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true { return 2771; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2732 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2739 {
            return Some(DarkOakShelf {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 2754 {
            return Some(DarkOakShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 2719 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2745 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2752 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2731 {
            return Some(DarkOakShelf {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2747 {
            return Some(DarkOakShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2761 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 2722 {
            return Some(DarkOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2759 {
            return Some(DarkOakShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2742 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 2782 {
            return Some(DarkOakShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2755 {
            return Some(DarkOakShelf {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2727 {
            return Some(DarkOakShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2750 {
            return Some(DarkOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2781 {
            return Some(DarkOakShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2728 {
            return Some(DarkOakShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 2729 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 2723 {
            return Some(DarkOakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2772 {
            return Some(DarkOakShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2740 {
            return Some(DarkOakShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 2751 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2760 {
            return Some(DarkOakShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2738 {
            return Some(DarkOakShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2753 {
            return Some(DarkOakShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2779 {
            return Some(DarkOakShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2736 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2726 {
            return Some(DarkOakShelf {
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2735 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2763 {
            return Some(DarkOakShelf {
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
            });
        }
        if state_id == 2773 {
            return Some(DarkOakShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2758 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 2720 {
            return Some(DarkOakShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2749 {
            return Some(DarkOakShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2765 {
            return Some(DarkOakShelf {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2734 {
            return Some(DarkOakShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 2737 {
            return Some(DarkOakShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2744 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2721 {
            return Some(DarkOakShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2764 {
            return Some(DarkOakShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
            });
        }
        if state_id == 2748 {
            return Some(DarkOakShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2769 {
            return Some(DarkOakShelf {
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 2746 {
            return Some(DarkOakShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 2724 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2733 {
            return Some(DarkOakShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 2776 {
            return Some(DarkOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 2777 {
            return Some(DarkOakShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2770 {
            return Some(DarkOakShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 2778 {
            return Some(DarkOakShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2768 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2774 {
            return Some(DarkOakShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2766 {
            return Some(DarkOakShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2756 {
            return Some(DarkOakShelf {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2767 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2780 {
            return Some(DarkOakShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2775 {
            return Some(DarkOakShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2741 {
            return Some(DarkOakShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#powered: true,
            });
        }
        if state_id == 2762 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2743 {
            return Some(DarkOakShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2730 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2757 {
            return Some(DarkOakShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2725 {
            return Some(DarkOakShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2771 {
            return Some(DarkOakShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

