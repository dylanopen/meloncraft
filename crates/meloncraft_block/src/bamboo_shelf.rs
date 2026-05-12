use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooShelf {
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

impl BlockState for BambooShelf {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::West { return 2495; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false { return 2492; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2498; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 2513; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false { return 2470; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 2503; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right { return 2490; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center { return 2500; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Center { return 2516; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 2521; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true { return 2502; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true { return 2496; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Left { return 2509; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false { return 2526; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected { return 2480; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true { return 2514; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North { return 2469; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == true { return 2493; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true { return 2497; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#powered == false { return 2505; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 2524; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected { return 2463; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::North { return 2473; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#side_chain == SideChain::Left { return 2501; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 2472; }
        if block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::West && block_state.r#powered == false { return 2508; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right { return 2466; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 2485; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Left && block_state.r#powered == false { return 2525; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false { return 2523; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 2507; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == true { return 2477; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == false { return 2471; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Right { return 2522; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected { return 2504; }
        if block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 2487; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 2479; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Center { return 2476; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::North { return 2478; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == false { return 2506; }
        if block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Unconnected && block_state.r#powered == true && block_state.r#facing == Facing::East { return 2511; }
        if block_state.r#powered == true && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 2515; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == false { return 2520; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#side_chain == SideChain::Center { return 2484; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#side_chain == SideChain::Center { return 2491; }
        if block_state.r#powered == false && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 2510; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Unconnected && block_state.r#waterlogged == true { return 2519; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Unconnected { return 2488; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Center { return 2499; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Right { return 2474; }
        if block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Center && block_state.r#waterlogged == true && block_state.r#powered == true { return 2483; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Left && block_state.r#powered == true && block_state.r#waterlogged == true { return 2517; }
        if block_state.r#side_chain == SideChain::Right && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 2489; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#side_chain == SideChain::Right { return 2481; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#side_chain == SideChain::Unconnected { return 2512; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == false { return 2486; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#side_chain == SideChain::Right && block_state.r#facing == Facing::North { return 2465; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false { return 2468; }
        if block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Center && block_state.r#powered == false && block_state.r#waterlogged == true { return 2475; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#side_chain == SideChain::Unconnected { return 2464; }
        if block_state.r#side_chain == SideChain::Left && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == false { return 2494; }
        if block_state.r#facing == Facing::East && block_state.r#side_chain == SideChain::Left && block_state.r#waterlogged == false && block_state.r#powered == true { return 2518; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#side_chain == SideChain::Right && block_state.r#powered == true { return 2482; }
        if block_state.r#side_chain == SideChain::Center && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true { return 2467; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2495 {
            return Some(BambooShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
            });
        }
        if state_id == 2492 {
            return Some(BambooShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 2498 {
            return Some(BambooShelf {
                r#powered: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2513 {
            return Some(BambooShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2470 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2503 {
            return Some(BambooShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2490 {
            return Some(BambooShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2500 {
            return Some(BambooShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2516 {
            return Some(BambooShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2521 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2502 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2496 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2509 {
            return Some(BambooShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2526 {
            return Some(BambooShelf {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 2480 {
            return Some(BambooShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2514 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2469 {
            return Some(BambooShelf {
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 2493 {
            return Some(BambooShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2497 {
            return Some(BambooShelf {
                r#powered: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2505 {
            return Some(BambooShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 2524 {
            return Some(BambooShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2463 {
            return Some(BambooShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2473 {
            return Some(BambooShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 2501 {
            return Some(BambooShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2472 {
            return Some(BambooShelf {
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 2508 {
            return Some(BambooShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 2466 {
            return Some(BambooShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2485 {
            return Some(BambooShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2525 {
            return Some(BambooShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 2523 {
            return Some(BambooShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2507 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 2477 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2471 {
            return Some(BambooShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 2522 {
            return Some(BambooShelf {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2504 {
            return Some(BambooShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2487 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2479 {
            return Some(BambooShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2476 {
            return Some(BambooShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2478 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2506 {
            return Some(BambooShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2511 {
            return Some(BambooShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2515 {
            return Some(BambooShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2520 {
            return Some(BambooShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2484 {
            return Some(BambooShelf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2491 {
            return Some(BambooShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2510 {
            return Some(BambooShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2519 {
            return Some(BambooShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2488 {
            return Some(BambooShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2499 {
            return Some(BambooShelf {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2474 {
            return Some(BambooShelf {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2483 {
            return Some(BambooShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2517 {
            return Some(BambooShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2489 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2481 {
            return Some(BambooShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2512 {
            return Some(BambooShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2486 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2465 {
            return Some(BambooShelf {
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 2468 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2475 {
            return Some(BambooShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2464 {
            return Some(BambooShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2494 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2518 {
            return Some(BambooShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2482 {
            return Some(BambooShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2467 {
            return Some(BambooShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

