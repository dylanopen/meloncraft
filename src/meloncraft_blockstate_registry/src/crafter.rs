use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crafter {
    pub crafting: bool,
    pub r#orientation: Orientation,
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
    fn to_id(&self) -> i32 {
        if self.r#orientation == Orientation::UpNorth
            && self.r#crafting == false
            && self.r#triggered == true
        {
            return 29441;
        }
        if self.r#triggered == true
            && self.r#crafting == false
            && self.r#orientation == Orientation::UpWest
        {
            return 29445;
        }
        if self.r#triggered == true
            && self.r#orientation == Orientation::NorthUp
            && self.r#crafting == true
        {
            return 29427;
        }
        if self.r#orientation == Orientation::WestUp
            && self.r#crafting == true
            && self.r#triggered == true
        {
            return 29423;
        }
        if self.r#orientation == Orientation::DownSouth
            && self.r#triggered == true
            && self.r#crafting == false
        {
            return 29435;
        }
        if self.r#crafting == false
            && self.r#triggered == false
            && self.r#orientation == Orientation::UpNorth
        {
            return 29442;
        }
        if self.r#crafting == false
            && self.r#triggered == false
            && self.r#orientation == Orientation::DownWest
        {
            return 29438;
        }
        if self.r#orientation == Orientation::UpEast
            && self.r#crafting == true
            && self.r#triggered == true
        {
            return 29415;
        }
        if self.r#crafting == true
            && self.r#orientation == Orientation::DownSouth
            && self.r#triggered == false
        {
            return 29412;
        }
        if self.r#crafting == false
            && self.r#triggered == true
            && self.r#orientation == Orientation::EastUp
        {
            return 29449;
        }
        if self.r#crafting == false
            && self.r#triggered == false
            && self.r#orientation == Orientation::EastUp
        {
            return 29450;
        }
        if self.r#crafting == true
            && self.r#orientation == Orientation::UpNorth
            && self.r#triggered == false
        {
            return 29418;
        }
        if self.r#crafting == false
            && self.r#orientation == Orientation::NorthUp
            && self.r#triggered == true
        {
            return 29451;
        }
        if self.r#orientation == Orientation::WestUp
            && self.r#triggered == true
            && self.r#crafting == false
        {
            return 29447;
        }
        if self.r#orientation == Orientation::NorthUp
            && self.r#triggered == false
            && self.r#crafting == false
        {
            return 29452;
        }
        if self.r#crafting == false
            && self.r#orientation == Orientation::DownEast
            && self.r#triggered == true
        {
            return 29431;
        }
        if self.r#triggered == false
            && self.r#crafting == true
            && self.r#orientation == Orientation::DownNorth
        {
            return 29410;
        }
        if self.r#triggered == false
            && self.r#crafting == true
            && self.r#orientation == Orientation::EastUp
        {
            return 29426;
        }
        if self.r#orientation == Orientation::DownNorth
            && self.r#crafting == false
            && self.r#triggered == true
        {
            return 29433;
        }
        if self.r#orientation == Orientation::SouthUp
            && self.r#triggered == false
            && self.r#crafting == true
        {
            return 29430;
        }
        if self.r#triggered == false
            && self.r#crafting == false
            && self.r#orientation == Orientation::UpEast
        {
            return 29440;
        }
        if self.r#orientation == Orientation::SouthUp
            && self.r#crafting == false
            && self.r#triggered == true
        {
            return 29453;
        }
        if self.r#triggered == true
            && self.r#crafting == true
            && self.r#orientation == Orientation::DownSouth
        {
            return 29411;
        }
        if self.r#triggered == false
            && self.r#crafting == true
            && self.r#orientation == Orientation::UpWest
        {
            return 29422;
        }
        if self.r#crafting == false
            && self.r#triggered == false
            && self.r#orientation == Orientation::WestUp
        {
            return 29448;
        }
        if self.r#triggered == true
            && self.r#crafting == true
            && self.r#orientation == Orientation::EastUp
        {
            return 29425;
        }
        if self.r#triggered == true
            && self.r#orientation == Orientation::DownWest
            && self.r#crafting == false
        {
            return 29437;
        }
        if self.r#orientation == Orientation::DownEast
            && self.r#triggered == false
            && self.r#crafting == false
        {
            return 29432;
        }
        if self.r#triggered == true
            && self.r#crafting == true
            && self.r#orientation == Orientation::DownNorth
        {
            return 29409;
        }
        if self.r#crafting == false
            && self.r#triggered == true
            && self.r#orientation == Orientation::UpSouth
        {
            return 29443;
        }
        if self.r#orientation == Orientation::SouthUp
            && self.r#triggered == false
            && self.r#crafting == false
        {
            return 29454;
        }
        if self.r#orientation == Orientation::DownEast
            && self.r#crafting == true
            && self.r#triggered == true
        {
            return 29407;
        }
        if self.r#triggered == false
            && self.r#crafting == false
            && self.r#orientation == Orientation::UpSouth
        {
            return 29444;
        }
        if self.r#triggered == false
            && self.r#crafting == true
            && self.r#orientation == Orientation::UpSouth
        {
            return 29420;
        }
        if self.r#crafting == false
            && self.r#orientation == Orientation::DownSouth
            && self.r#triggered == false
        {
            return 29436;
        }
        if self.r#triggered == true
            && self.r#crafting == true
            && self.r#orientation == Orientation::UpNorth
        {
            return 29417;
        }
        if self.r#triggered == true
            && self.r#crafting == true
            && self.r#orientation == Orientation::SouthUp
        {
            return 29429;
        }
        if self.r#crafting == false
            && self.r#orientation == Orientation::UpEast
            && self.r#triggered == true
        {
            return 29439;
        }
        if self.r#orientation == Orientation::UpWest
            && self.r#triggered == false
            && self.r#crafting == false
        {
            return 29446;
        }
        if self.r#orientation == Orientation::NorthUp
            && self.r#triggered == false
            && self.r#crafting == true
        {
            return 29428;
        }
        if self.r#crafting == true
            && self.r#orientation == Orientation::UpWest
            && self.r#triggered == true
        {
            return 29421;
        }
        if self.r#crafting == false
            && self.r#orientation == Orientation::DownNorth
            && self.r#triggered == false
        {
            return 29434;
        }
        if self.r#crafting == true
            && self.r#orientation == Orientation::DownEast
            && self.r#triggered == false
        {
            return 29408;
        }
        if self.r#orientation == Orientation::DownWest
            && self.r#crafting == true
            && self.r#triggered == true
        {
            return 29413;
        }
        if self.r#orientation == Orientation::UpEast
            && self.r#triggered == false
            && self.r#crafting == true
        {
            return 29416;
        }
        if self.r#crafting == true
            && self.r#orientation == Orientation::WestUp
            && self.r#triggered == false
        {
            return 29424;
        }
        if self.r#crafting == true
            && self.r#triggered == false
            && self.r#orientation == Orientation::DownWest
        {
            return 29414;
        }
        if self.r#crafting == true
            && self.r#orientation == Orientation::UpSouth
            && self.r#triggered == true
        {
            return 29419;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29441 {
            return Some(Crafter {
                r#orientation: Orientation::UpNorth,
                r#crafting: false,
                r#triggered: true,
            });
        }
        if state_id == 29445 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: false,
                r#orientation: Orientation::UpWest,
            });
        }
        if state_id == 29427 {
            return Some(Crafter {
                r#triggered: true,
                r#orientation: Orientation::NorthUp,
                r#crafting: true,
            });
        }
        if state_id == 29423 {
            return Some(Crafter {
                r#orientation: Orientation::WestUp,
                r#crafting: true,
                r#triggered: true,
            });
        }
        if state_id == 29435 {
            return Some(Crafter {
                r#orientation: Orientation::DownSouth,
                r#triggered: true,
                r#crafting: false,
            });
        }
        if state_id == 29442 {
            return Some(Crafter {
                r#crafting: false,
                r#triggered: false,
                r#orientation: Orientation::UpNorth,
            });
        }
        if state_id == 29438 {
            return Some(Crafter {
                r#crafting: false,
                r#triggered: false,
                r#orientation: Orientation::DownWest,
            });
        }
        if state_id == 29415 {
            return Some(Crafter {
                r#orientation: Orientation::UpEast,
                r#crafting: true,
                r#triggered: true,
            });
        }
        if state_id == 29412 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::DownSouth,
                r#triggered: false,
            });
        }
        if state_id == 29449 {
            return Some(Crafter {
                r#crafting: false,
                r#triggered: true,
                r#orientation: Orientation::EastUp,
            });
        }
        if state_id == 29450 {
            return Some(Crafter {
                r#crafting: false,
                r#triggered: false,
                r#orientation: Orientation::EastUp,
            });
        }
        if state_id == 29418 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::UpNorth,
                r#triggered: false,
            });
        }
        if state_id == 29451 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::NorthUp,
                r#triggered: true,
            });
        }
        if state_id == 29447 {
            return Some(Crafter {
                r#orientation: Orientation::WestUp,
                r#triggered: true,
                r#crafting: false,
            });
        }
        if state_id == 29452 {
            return Some(Crafter {
                r#orientation: Orientation::NorthUp,
                r#triggered: false,
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
        if state_id == 29410 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: true,
                r#orientation: Orientation::DownNorth,
            });
        }
        if state_id == 29426 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: true,
                r#orientation: Orientation::EastUp,
            });
        }
        if state_id == 29433 {
            return Some(Crafter {
                r#orientation: Orientation::DownNorth,
                r#crafting: false,
                r#triggered: true,
            });
        }
        if state_id == 29430 {
            return Some(Crafter {
                r#orientation: Orientation::SouthUp,
                r#triggered: false,
                r#crafting: true,
            });
        }
        if state_id == 29440 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: false,
                r#orientation: Orientation::UpEast,
            });
        }
        if state_id == 29453 {
            return Some(Crafter {
                r#orientation: Orientation::SouthUp,
                r#crafting: false,
                r#triggered: true,
            });
        }
        if state_id == 29411 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: true,
                r#orientation: Orientation::DownSouth,
            });
        }
        if state_id == 29422 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: true,
                r#orientation: Orientation::UpWest,
            });
        }
        if state_id == 29448 {
            return Some(Crafter {
                r#crafting: false,
                r#triggered: false,
                r#orientation: Orientation::WestUp,
            });
        }
        if state_id == 29425 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: true,
                r#orientation: Orientation::EastUp,
            });
        }
        if state_id == 29437 {
            return Some(Crafter {
                r#triggered: true,
                r#orientation: Orientation::DownWest,
                r#crafting: false,
            });
        }
        if state_id == 29432 {
            return Some(Crafter {
                r#orientation: Orientation::DownEast,
                r#triggered: false,
                r#crafting: false,
            });
        }
        if state_id == 29409 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: true,
                r#orientation: Orientation::DownNorth,
            });
        }
        if state_id == 29443 {
            return Some(Crafter {
                r#crafting: false,
                r#triggered: true,
                r#orientation: Orientation::UpSouth,
            });
        }
        if state_id == 29454 {
            return Some(Crafter {
                r#orientation: Orientation::SouthUp,
                r#triggered: false,
                r#crafting: false,
            });
        }
        if state_id == 29407 {
            return Some(Crafter {
                r#orientation: Orientation::DownEast,
                r#crafting: true,
                r#triggered: true,
            });
        }
        if state_id == 29444 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: false,
                r#orientation: Orientation::UpSouth,
            });
        }
        if state_id == 29420 {
            return Some(Crafter {
                r#triggered: false,
                r#crafting: true,
                r#orientation: Orientation::UpSouth,
            });
        }
        if state_id == 29436 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::DownSouth,
                r#triggered: false,
            });
        }
        if state_id == 29417 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: true,
                r#orientation: Orientation::UpNorth,
            });
        }
        if state_id == 29429 {
            return Some(Crafter {
                r#triggered: true,
                r#crafting: true,
                r#orientation: Orientation::SouthUp,
            });
        }
        if state_id == 29439 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::UpEast,
                r#triggered: true,
            });
        }
        if state_id == 29446 {
            return Some(Crafter {
                r#orientation: Orientation::UpWest,
                r#triggered: false,
                r#crafting: false,
            });
        }
        if state_id == 29428 {
            return Some(Crafter {
                r#orientation: Orientation::NorthUp,
                r#triggered: false,
                r#crafting: true,
            });
        }
        if state_id == 29421 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::UpWest,
                r#triggered: true,
            });
        }
        if state_id == 29434 {
            return Some(Crafter {
                r#crafting: false,
                r#orientation: Orientation::DownNorth,
                r#triggered: false,
            });
        }
        if state_id == 29408 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::DownEast,
                r#triggered: false,
            });
        }
        if state_id == 29413 {
            return Some(Crafter {
                r#orientation: Orientation::DownWest,
                r#crafting: true,
                r#triggered: true,
            });
        }
        if state_id == 29416 {
            return Some(Crafter {
                r#orientation: Orientation::UpEast,
                r#triggered: false,
                r#crafting: true,
            });
        }
        if state_id == 29424 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::WestUp,
                r#triggered: false,
            });
        }
        if state_id == 29414 {
            return Some(Crafter {
                r#crafting: true,
                r#triggered: false,
                r#orientation: Orientation::DownWest,
            });
        }
        if state_id == 29419 {
            return Some(Crafter {
                r#crafting: true,
                r#orientation: Orientation::UpSouth,
                r#triggered: true,
            });
        }
        return None;
    }
}
