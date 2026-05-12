use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchShelf {
    pub r#side_chain: SideChain,
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub powered: bool,
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

impl BlockState for BirchShelf {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true && block_state.r#waterlogged == false { return 2564; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true { return 2529; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 2583; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 2535; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::South { return 2552; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::West { return 2561; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left { return 2534; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 2527; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::West && block_state.r#powered == false { return 2573; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 2541; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 2544; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false { return 2553; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::East { return 2585; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true { return 2563; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false { return 2555; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#powered == true { return 2577; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::East { return 2590; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true { return 2547; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == true { return 2545; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 2559; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::West { return 2567; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true && block_state.r#facing == Facing::West { return 2562; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected { return 2543; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false && block_state.r#powered == true { return 2580; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#powered == true { return 2531; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 2549; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 2528; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 2536; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 2533; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false { return 2558; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false && block_state.r#waterlogged == false { return 2574; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false { return 2550; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 2557; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true { return 2546; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2578; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#waterlogged == true { return 2565; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 2540; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::North { return 2530; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false { return 2560; }
        if block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#waterlogged == false { return 2570; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center { return 2571; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected { return 2551; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right { return 2538; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 2589; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::South { return 2548; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 2584; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2586; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::West { return 2569; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::South { return 2554; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::East { return 2588; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == true { return 2566; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 2582; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::North && block_state.r#powered == true { return 2532; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::North { return 2537; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false { return 2556; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false { return 2542; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false { return 2568; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected { return 2576; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center { return 2579; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected { return 2575; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#side_chain == SideChain::Left { return 2581; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center { return 2587; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false { return 2539; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 2572; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2564 {
            return Some(BirchShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2529 {
            return Some(BirchShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2583 {
            return Some(BirchShelf {
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2535 {
            return Some(BirchShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2552 {
            return Some(BirchShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
            });
        }
        if state_id == 2561 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2534 {
            return Some(BirchShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2527 {
            return Some(BirchShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2573 {
            return Some(BirchShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 2541 {
            return Some(BirchShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2544 {
            return Some(BirchShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2553 {
            return Some(BirchShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 2585 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2563 {
            return Some(BirchShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 2555 {
            return Some(BirchShelf {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2577 {
            return Some(BirchShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2590 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2547 {
            return Some(BirchShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2545 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 2559 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2567 {
            return Some(BirchShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
            });
        }
        if state_id == 2562 {
            return Some(BirchShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2543 {
            return Some(BirchShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2580 {
            return Some(BirchShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2531 {
            return Some(BirchShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#powered: true,
            });
        }
        if state_id == 2549 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2528 {
            return Some(BirchShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 2536 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2533 {
            return Some(BirchShelf {
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2558 {
            return Some(BirchShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 2574 {
            return Some(BirchShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2550 {
            return Some(BirchShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2557 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2546 {
            return Some(BirchShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2578 {
            return Some(BirchShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2565 {
            return Some(BirchShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2540 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 2530 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2560 {
            return Some(BirchShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2570 {
            return Some(BirchShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2571 {
            return Some(BirchShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2551 {
            return Some(BirchShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2538 {
            return Some(BirchShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2589 {
            return Some(BirchShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2548 {
            return Some(BirchShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
            });
        }
        if state_id == 2584 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2586 {
            return Some(BirchShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2569 {
            return Some(BirchShelf {
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 2554 {
            return Some(BirchShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 2588 {
            return Some(BirchShelf {
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
            });
        }
        if state_id == 2566 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 2582 {
            return Some(BirchShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2532 {
            return Some(BirchShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 2537 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2556 {
            return Some(BirchShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2542 {
            return Some(BirchShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2568 {
            return Some(BirchShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2576 {
            return Some(BirchShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2579 {
            return Some(BirchShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2575 {
            return Some(BirchShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2581 {
            return Some(BirchShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2587 {
            return Some(BirchShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2539 {
            return Some(BirchShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2572 {
            return Some(BirchShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

