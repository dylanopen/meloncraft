use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crafter {
    pub r#orientation: Orientation,
    pub crafting: bool,
    pub triggered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
    EastUp,
    NorthUp,
    SouthUp,
}

impl BlockState for Crafter {
    fn to_id(self) -> i32 {
        if block_state.r#orientation == Orientation::DownNorth && block_state.r#triggered == true && block_state.r#crafting == true { return 29409; }
        if block_state.r#triggered == false && block_state.r#crafting == true && block_state.r#orientation == Orientation::DownWest { return 29414; }
        if block_state.r#orientation == Orientation::UpSouth && block_state.r#crafting == true && block_state.r#triggered == false { return 29420; }
        if block_state.r#crafting == true && block_state.r#orientation == Orientation::SouthUp && block_state.r#triggered == false { return 29430; }
        if block_state.r#crafting == true && block_state.r#orientation == Orientation::UpNorth && block_state.r#triggered == false { return 29418; }
        if block_state.r#triggered == false && block_state.r#orientation == Orientation::DownSouth && block_state.r#crafting == true { return 29412; }
        if block_state.r#orientation == Orientation::DownSouth && block_state.r#triggered == false && block_state.r#crafting == false { return 29436; }
        if block_state.r#orientation == Orientation::UpNorth && block_state.r#crafting == false && block_state.r#triggered == true { return 29441; }
        if block_state.r#orientation == Orientation::EastUp && block_state.r#triggered == true && block_state.r#crafting == false { return 29449; }
        if block_state.r#crafting == false && block_state.r#orientation == Orientation::DownEast && block_state.r#triggered == true { return 29431; }
        if block_state.r#crafting == false && block_state.r#orientation == Orientation::SouthUp && block_state.r#triggered == true { return 29453; }
        if block_state.r#triggered == true && block_state.r#orientation == Orientation::EastUp && block_state.r#crafting == true { return 29425; }
        if block_state.r#crafting == false && block_state.r#orientation == Orientation::DownWest && block_state.r#triggered == true { return 29437; }
        if block_state.r#triggered == true && block_state.r#crafting == true && block_state.r#orientation == Orientation::WestUp { return 29423; }
        if block_state.r#crafting == false && block_state.r#orientation == Orientation::EastUp && block_state.r#triggered == false { return 29450; }
        if block_state.r#triggered == true && block_state.r#crafting == true && block_state.r#orientation == Orientation::DownEast { return 29407; }
        if block_state.r#triggered == true && block_state.r#crafting == false && block_state.r#orientation == Orientation::UpSouth { return 29443; }
        if block_state.r#crafting == false && block_state.r#triggered == true && block_state.r#orientation == Orientation::DownSouth { return 29435; }
        if block_state.r#triggered == true && block_state.r#crafting == false && block_state.r#orientation == Orientation::UpEast { return 29439; }
        if block_state.r#orientation == Orientation::DownEast && block_state.r#triggered == false && block_state.r#crafting == false { return 29432; }
        if block_state.r#triggered == false && block_state.r#crafting == true && block_state.r#orientation == Orientation::DownEast { return 29408; }
        if block_state.r#orientation == Orientation::DownWest && block_state.r#crafting == true && block_state.r#triggered == true { return 29413; }
        if block_state.r#orientation == Orientation::UpWest && block_state.r#triggered == true && block_state.r#crafting == true { return 29421; }
        if block_state.r#triggered == false && block_state.r#orientation == Orientation::NorthUp && block_state.r#crafting == false { return 29452; }
        if block_state.r#orientation == Orientation::DownNorth && block_state.r#triggered == false && block_state.r#crafting == false { return 29434; }
        if block_state.r#crafting == false && block_state.r#orientation == Orientation::UpWest && block_state.r#triggered == false { return 29446; }
        if block_state.r#orientation == Orientation::NorthUp && block_state.r#crafting == false && block_state.r#triggered == true { return 29451; }
        if block_state.r#orientation == Orientation::DownNorth && block_state.r#triggered == true && block_state.r#crafting == false { return 29433; }
        if block_state.r#triggered == true && block_state.r#orientation == Orientation::UpSouth && block_state.r#crafting == true { return 29419; }
        if block_state.r#crafting == true && block_state.r#triggered == true && block_state.r#orientation == Orientation::NorthUp { return 29427; }
        if block_state.r#crafting == true && block_state.r#triggered == false && block_state.r#orientation == Orientation::UpWest { return 29422; }
        if block_state.r#triggered == false && block_state.r#crafting == true && block_state.r#orientation == Orientation::EastUp { return 29426; }
        if block_state.r#orientation == Orientation::UpNorth && block_state.r#triggered == false && block_state.r#crafting == false { return 29442; }
        if block_state.r#crafting == true && block_state.r#triggered == false && block_state.r#orientation == Orientation::WestUp { return 29424; }
        if block_state.r#crafting == true && block_state.r#orientation == Orientation::UpNorth && block_state.r#triggered == true { return 29417; }
        if block_state.r#orientation == Orientation::WestUp && block_state.r#triggered == false && block_state.r#crafting == false { return 29448; }
        if block_state.r#crafting == false && block_state.r#orientation == Orientation::SouthUp && block_state.r#triggered == false { return 29454; }
        if block_state.r#crafting == true && block_state.r#orientation == Orientation::UpEast && block_state.r#triggered == false { return 29416; }
        if block_state.r#crafting == true && block_state.r#orientation == Orientation::NorthUp && block_state.r#triggered == false { return 29428; }
        if block_state.r#triggered == false && block_state.r#orientation == Orientation::DownNorth && block_state.r#crafting == true { return 29410; }
        if block_state.r#orientation == Orientation::UpEast && block_state.r#triggered == true && block_state.r#crafting == true { return 29415; }
        if block_state.r#triggered == false && block_state.r#orientation == Orientation::DownWest && block_state.r#crafting == false { return 29438; }
        if block_state.r#triggered == false && block_state.r#crafting == false && block_state.r#orientation == Orientation::UpEast { return 29440; }
        if block_state.r#crafting == false && block_state.r#orientation == Orientation::UpWest && block_state.r#triggered == true { return 29445; }
        if block_state.r#crafting == true && block_state.r#orientation == Orientation::SouthUp && block_state.r#triggered == true { return 29429; }
        if block_state.r#triggered == false && block_state.r#orientation == Orientation::UpSouth && block_state.r#crafting == false { return 29444; }
        if block_state.r#crafting == true && block_state.r#triggered == true && block_state.r#orientation == Orientation::DownSouth { return 29411; }
        if block_state.r#crafting == false && block_state.r#triggered == true && block_state.r#orientation == Orientation::WestUp { return 29447; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29409 {
            return Some(Crafter {
                r#orientation: Orientation::DownNorth,
                r#triggered: true,
                r#crafting: true,
            });
        }
        if state_id == 29414 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: true,
                r#orientation: Orientation::DownWest,
            });
        }
        if state_id == 29420 {
            return Some(Crafter {
                r#orientation: Orientation::UpSouth,
                r#crafting: true,
                r#triggered: false,
            });
        }
        if state_id == 29430 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::SouthUp,
                r#triggered: false,
            });
        }
        if state_id == 29418 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::UpNorth,
                r#triggered: false,
            });
        }
        if state_id == 29412 {
            return Some(Crafter {
                r#triggered: false,
                r#orientation: Orientation::DownSouth,
                r#crafting: true,
            });
        }
        if state_id == 29436 {
            return Some(Crafter {
                r#orientation: Orientation::DownSouth,
                r#triggered: false,
                r#crafting: false,
            });
        }
        if state_id == 29441 {
            return Some(Crafter {
                r#orientation: Orientation::UpNorth,
                r#crafting: false,
                r#triggered: true,
            });
        }
        if state_id == 29449 {
            return Some(Crafter {
                r#orientation: Orientation::EastUp,
                r#triggered: true,
                r#crafting: false,
            });
        }
        if state_id == 29431 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::DownEast,
                r#triggered: true,
            });
        }
        if state_id == 29453 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::SouthUp,
                r#triggered: true,
            });
        }
        if state_id == 29425 {
            return Some(Crafter {
                r#triggered: true,
                r#orientation: Orientation::EastUp,
                r#crafting: true,
            });
        }
        if state_id == 29437 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::DownWest,
                r#triggered: true,
            });
        }
        if state_id == 29423 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: true,
                r#orientation: Orientation::WestUp,
            });
        }
        if state_id == 29450 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::EastUp,
                r#triggered: false,
            });
        }
        if state_id == 29407 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: true,
                r#orientation: Orientation::DownEast,
            });
        }
        if state_id == 29443 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: false,
                r#orientation: Orientation::UpSouth,
            });
        }
        if state_id == 29435 {
            return Some(Crafter {
                r#crafting: false,
                r#triggered: true,
                r#orientation: Orientation::DownSouth,
            });
        }
        if state_id == 29439 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: false,
                r#orientation: Orientation::UpEast,
            });
        }
        if state_id == 29432 {
            return Some(Crafter {
                r#orientation: Orientation::DownEast,
                r#triggered: false,
                r#crafting: false,
            });
        }
        if state_id == 29408 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: true,
                r#orientation: Orientation::DownEast,
            });
        }
        if state_id == 29413 {
            return Some(Crafter {
                r#orientation: Orientation::DownWest,
                r#crafting: true,
                r#triggered: true,
            });
        }
        if state_id == 29421 {
            return Some(Crafter {
                r#orientation: Orientation::UpWest,
                r#triggered: true,
                r#crafting: true,
            });
        }
        if state_id == 29452 {
            return Some(Crafter {
                r#triggered: false,
                r#orientation: Orientation::NorthUp,
                r#crafting: false,
            });
        }
        if state_id == 29434 {
            return Some(Crafter {
                r#orientation: Orientation::DownNorth,
                r#triggered: false,
                r#crafting: false,
            });
        }
        if state_id == 29446 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::UpWest,
                r#triggered: false,
            });
        }
        if state_id == 29451 {
            return Some(Crafter {
                r#orientation: Orientation::NorthUp,
                r#crafting: false,
                r#triggered: true,
            });
        }
        if state_id == 29433 {
            return Some(Crafter {
                r#orientation: Orientation::DownNorth,
                r#triggered: true,
                r#crafting: false,
            });
        }
        if state_id == 29419 {
            return Some(Crafter {
                r#triggered: true,
                r#orientation: Orientation::UpSouth,
                r#crafting: true,
            });
        }
        if state_id == 29427 {
            return Some(Crafter {
                r#crafting: true,
                r#triggered: true,
                r#orientation: Orientation::NorthUp,
            });
        }
        if state_id == 29422 {
            return Some(Crafter {
                r#crafting: true,
                r#triggered: false,
                r#orientation: Orientation::UpWest,
            });
        }
        if state_id == 29426 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: true,
                r#orientation: Orientation::EastUp,
            });
        }
        if state_id == 29442 {
            return Some(Crafter {
                r#orientation: Orientation::UpNorth,
                r#triggered: false,
                r#crafting: false,
            });
        }
        if state_id == 29424 {
            return Some(Crafter {
                r#crafting: true,
                r#triggered: false,
                r#orientation: Orientation::WestUp,
            });
        }
        if state_id == 29417 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::UpNorth,
                r#triggered: true,
            });
        }
        if state_id == 29448 {
            return Some(Crafter {
                r#orientation: Orientation::WestUp,
                r#triggered: false,
                r#crafting: false,
            });
        }
        if state_id == 29454 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::SouthUp,
                r#triggered: false,
            });
        }
        if state_id == 29416 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::UpEast,
                r#triggered: false,
            });
        }
        if state_id == 29428 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::NorthUp,
                r#triggered: false,
            });
        }
        if state_id == 29410 {
            return Some(Crafter {
                r#triggered: false,
                r#orientation: Orientation::DownNorth,
                r#crafting: true,
            });
        }
        if state_id == 29415 {
            return Some(Crafter {
                r#orientation: Orientation::UpEast,
                r#triggered: true,
                r#crafting: true,
            });
        }
        if state_id == 29438 {
            return Some(Crafter {
                r#triggered: false,
                r#orientation: Orientation::DownWest,
                r#crafting: false,
            });
        }
        if state_id == 29440 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: false,
                r#orientation: Orientation::UpEast,
            });
        }
        if state_id == 29445 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::UpWest,
                r#triggered: true,
            });
        }
        if state_id == 29429 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::SouthUp,
                r#triggered: true,
            });
        }
        if state_id == 29444 {
            return Some(Crafter {
                r#triggered: false,
                r#orientation: Orientation::UpSouth,
                r#crafting: false,
            });
        }
        if state_id == 29411 {
            return Some(Crafter {
                r#crafting: true,
                r#triggered: true,
                r#orientation: Orientation::DownSouth,
            });
        }
        if state_id == 29447 {
            return Some(Crafter {
                r#crafting: false,
                r#triggered: true,
                r#orientation: Orientation::WestUp,
            });
        }
        return None;
    }
}

