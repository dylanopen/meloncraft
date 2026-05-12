use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryShelf {
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

impl BlockState for CherryShelf {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false { return 2630; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East { return 2647; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == false { return 2600; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#facing == Facing::West { return 2632; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#powered == false { return 2616; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 2626; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 2652; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#facing == Facing::North { return 2594; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 2645; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North { return 2598; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false { return 2617; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 2627; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true { return 2642; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false && block_state.r#facing == Facing::South { return 2621; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 2654; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false && block_state.r#powered == true { return 2628; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#waterlogged == true { return 2609; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center { return 2603; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 2608; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false && block_state.r#powered == true { return 2624; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#powered == false { return 2635; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 2629; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 2591; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false && block_state.r#powered == true { return 2610; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 2625; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West && block_state.r#powered == false { return 2633; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::South { return 2619; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left { return 2606; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 2605; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == true { return 2631; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 2599; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false { return 2596; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 2634; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center { return 2636; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 2613; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#facing == Facing::South { return 2618; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true { return 2641; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 2649; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East { return 2648; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false { return 2640; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center { return 2644; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 2653; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East && block_state.r#powered == true { return 2643; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 2646; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 2623; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 2611; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true { return 2595; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 2602; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center { return 2651; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left { return 2637; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::North { return 2592; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true { return 2593; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 2638; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true && block_state.r#facing == Facing::South { return 2612; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#powered == true { return 2597; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 2615; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center { return 2620; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2650; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true { return 2607; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Left { return 2622; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center { return 2604; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#waterlogged == true { return 2639; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left { return 2614; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 2601; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2630 {
            return Some(CherryShelf {
                r#powered: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2647 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
            });
        }
        if state_id == 2600 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2632 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2616 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2626 {
            return Some(CherryShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2652 {
            return Some(CherryShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2594 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2645 {
            return Some(CherryShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2598 {
            return Some(CherryShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 2617 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 2627 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2642 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2621 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2654 {
            return Some(CherryShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2628 {
            return Some(CherryShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2609 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2603 {
            return Some(CherryShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2608 {
            return Some(CherryShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 2624 {
            return Some(CherryShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2635 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2629 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2591 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2610 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2625 {
            return Some(CherryShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2633 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 2619 {
            return Some(CherryShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
            });
        }
        if state_id == 2606 {
            return Some(CherryShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2605 {
            return Some(CherryShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2631 {
            return Some(CherryShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2599 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 2596 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2634 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 2636 {
            return Some(CherryShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2613 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2618 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2641 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2649 {
            return Some(CherryShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2648 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
            });
        }
        if state_id == 2640 {
            return Some(CherryShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2644 {
            return Some(CherryShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2653 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2643 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 2646 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2623 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2611 {
            return Some(CherryShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2595 {
            return Some(CherryShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 2602 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2651 {
            return Some(CherryShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2637 {
            return Some(CherryShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2592 {
            return Some(CherryShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
            });
        }
        if state_id == 2593 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2638 {
            return Some(CherryShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2612 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2597 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 2615 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2620 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2650 {
            return Some(CherryShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2607 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
            });
        }
        if state_id == 2622 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2604 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2639 {
            return Some(CherryShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2614 {
            return Some(CherryShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2601 {
            return Some(CherryShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

