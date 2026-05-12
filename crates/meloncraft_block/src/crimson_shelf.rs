use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonShelf {
    pub r#side_chain: SideChain,
    pub powered: bool,
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

impl BlockState for CrimsonShelf {
    fn to_id(self) -> i32 {
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::South { return 2671; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center { return 2675; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true && block_state.r#powered == false { return 2717; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#waterlogged == true { return 2661; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::South { return 2674; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == false { return 2698; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 2679; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#waterlogged == false { return 2704; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false { return 2712; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center { return 2716; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false { return 2718; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right { return 2705; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right { return 2658; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected { return 2688; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 2677; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2666; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South { return 2678; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::East { return 2709; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false && block_state.r#powered == false { return 2668; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::West { return 2693; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true { return 2665; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::West && block_state.r#powered == true { return 2691; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true { return 2708; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2682; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false && block_state.r#powered == true { return 2706; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Left { return 2685; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false { return 2681; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left { return 2701; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 2703; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center { return 2660; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true { return 2657; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 2670; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center { return 2692; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left { return 2669; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East { return 2714; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 2676; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false { return 2700; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == false { return 2664; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 2696; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 2713; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 2680; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Right { return 2689; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false { return 2702; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South { return 2686; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#powered == true { return 2672; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false { return 2684; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 2655; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false { return 2656; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true { return 2683; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 2663; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center { return 2659; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false { return 2711; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true { return 2690; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::East { return 2710; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 2662; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected { return 2695; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::East { return 2707; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#powered == false { return 2697; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 2673; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true { return 2694; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false { return 2699; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true { return 2667; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 2687; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false { return 2715; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2671 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2675 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2717 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2661 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2674 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 2698 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2679 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2704 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2712 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2716 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2718 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2705 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2658 {
            return Some(CrimsonShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2688 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2677 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2666 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2678 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 2709 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 2668 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2693 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2665 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2691 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 2708 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 2682 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2706 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2685 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2681 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 2701 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2703 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2660 {
            return Some(CrimsonShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2657 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2670 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 2692 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2669 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2714 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 2676 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2700 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2664 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2696 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 2713 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2680 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2689 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2702 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 2686 {
            return Some(CrimsonShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 2672 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2684 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2655 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2656 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2683 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2663 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2659 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2711 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 2690 {
            return Some(CrimsonShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2710 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 2662 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 2695 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2707 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2697 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2673 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2694 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2699 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2667 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2687 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2715 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        return None;
    }
}

