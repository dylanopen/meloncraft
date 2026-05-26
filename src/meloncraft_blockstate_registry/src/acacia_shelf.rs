use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaShelf {
    pub powered: bool,
    pub r#side_chain: SideChain,
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for AcaciaShelf {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 2402;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
        {
            return 2444;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
        {
            return 2400;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 2441;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
        {
            return 2419;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::East
        {
            return 2457;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 2404;
        }
        if self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
        {
            return 2451;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
        {
            return 2458;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 2428;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2431;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 2442;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::East
        {
            return 2452;
        }
        if self.r#waterlogged == true
            && self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::East
        {
            return 2455;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Center
        {
            return 2412;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 2426;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 2436;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Right
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 2401;
        }
        if self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 2430;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::North
        {
            return 2414;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
        {
            return 2433;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 2434;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 2413;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 2409;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 2445;
        }
        if self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Left
        {
            return 2453;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2415;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
        {
            return 2449;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 2405;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 2439;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 2417;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 2410;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 2432;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
        {
            return 2435;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::West
        {
            return 2438;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 2448;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
        {
            return 2422;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 2456;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::South
        {
            return 2423;
        }
        if self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Center
        {
            return 2403;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 2418;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2447;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 2399;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 2416;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::West
        {
            return 2446;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
        {
            return 2424;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Center
        {
            return 2420;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
        {
            return 2459;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
        {
            return 2437;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 2411;
        }
        if self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
        {
            return 2462;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 2450;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Left
        {
            return 2461;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Center
            && self.r#powered == false
        {
            return 2443;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
        {
            return 2406;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 2440;
        }
        if self.r#waterlogged == true
            && self.r#powered == false
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::South
        {
            return 2429;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 2454;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 2460;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2407;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 2427;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 2425;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::North
        {
            return 2408;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 2421;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2402 {
            return Some(AcaciaShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2444 {
            return Some(AcaciaShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2400 {
            return Some(AcaciaShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2441 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2419 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2457 {
            return Some(AcaciaShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 2404 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2451 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2458 {
            return Some(AcaciaShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2428 {
            return Some(AcaciaShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2431 {
            return Some(AcaciaShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2442 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 2452 {
            return Some(AcaciaShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
            });
        }
        if state_id == 2455 {
            return Some(AcaciaShelf {
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
            });
        }
        if state_id == 2412 {
            return Some(AcaciaShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2426 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2436 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2401 {
            return Some(AcaciaShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2430 {
            return Some(AcaciaShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2414 {
            return Some(AcaciaShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 2433 {
            return Some(AcaciaShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2434 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 2413 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 2409 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2445 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2453 {
            return Some(AcaciaShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2415 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2449 {
            return Some(AcaciaShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2405 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2439 {
            return Some(AcaciaShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 2417 {
            return Some(AcaciaShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 2410 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2432 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 2435 {
            return Some(AcaciaShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2438 {
            return Some(AcaciaShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 2448 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2422 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2456 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2423 {
            return Some(AcaciaShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
            });
        }
        if state_id == 2403 {
            return Some(AcaciaShelf {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2418 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2447 {
            return Some(AcaciaShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2399 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 2416 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2446 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2424 {
            return Some(AcaciaShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2420 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2459 {
            return Some(AcaciaShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2437 {
            return Some(AcaciaShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2411 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 2462 {
            return Some(AcaciaShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2450 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2461 {
            return Some(AcaciaShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2443 {
            return Some(AcaciaShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2406 {
            return Some(AcaciaShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2440 {
            return Some(AcaciaShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2429 {
            return Some(AcaciaShelf {
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 2454 {
            return Some(AcaciaShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2460 {
            return Some(AcaciaShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2407 {
            return Some(AcaciaShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2427 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2425 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2408 {
            return Some(AcaciaShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2421 {
            return Some(AcaciaShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
