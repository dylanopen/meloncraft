use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChorusPlant {
    pub up: bool,
    pub east: bool,
    pub west: bool,
    pub down: bool,
    pub north: bool,
    pub south: bool,
}

impl BlockState for ChorusPlant {
    fn to_id(&self) -> i32 {
        if self.r#west == true
            && self.r#north == false
            && self.r#down == true
            && self.r#east == true
            && self.r#south == false
            && self.r#up == true
        {
            return 14452;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#down == false
            && self.r#up == false
            && self.r#south == false
            && self.r#west == true
        {
            return 14486;
        }
        if self.r#north == true
            && self.r#down == false
            && self.r#up == true
            && self.r#south == false
            && self.r#west == true
            && self.r#east == false
        {
            return 14492;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#down == false
            && self.r#south == false
            && self.r#up == false
            && self.r#west == false
        {
            return 14495;
        }
        if self.r#down == false
            && self.r#west == true
            && self.r#east == false
            && self.r#up == false
            && self.r#south == true
            && self.r#north == true
        {
            return 14490;
        }
        if self.r#west == true
            && self.r#south == true
            && self.r#north == false
            && self.r#east == false
            && self.r#down == true
            && self.r#up == false
        {
            return 14466;
        }
        if self.r#down == true
            && self.r#north == true
            && self.r#up == true
            && self.r#south == true
            && self.r#west == true
            && self.r#east == true
        {
            return 14440;
        }
        if self.r#east == true
            && self.r#down == true
            && self.r#south == true
            && self.r#up == false
            && self.r#west == false
            && self.r#north == true
        {
            return 14443;
        }
        if self.r#down == true
            && self.r#east == true
            && self.r#south == false
            && self.r#up == false
            && self.r#west == true
            && self.r#north == false
        {
            return 14454;
        }
        if self.r#up == true
            && self.r#east == false
            && self.r#down == false
            && self.r#north == false
            && self.r#south == true
            && self.r#west == true
        {
            return 14496;
        }
        if self.r#down == false
            && self.r#east == true
            && self.r#south == false
            && self.r#west == true
            && self.r#north == true
            && self.r#up == true
        {
            return 14476;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#up == false
            && self.r#south == true
            && self.r#west == true
            && self.r#down == true
        {
            return 14450;
        }
        if self.r#north == true
            && self.r#down == true
            && self.r#south == false
            && self.r#up == true
            && self.r#west == false
            && self.r#east == false
        {
            return 14461;
        }
        if self.r#west == true
            && self.r#down == false
            && self.r#east == true
            && self.r#north == true
            && self.r#south == true
            && self.r#up == true
        {
            return 14472;
        }
        if self.r#south == false
            && self.r#north == true
            && self.r#west == true
            && self.r#east == true
            && self.r#up == true
            && self.r#down == true
        {
            return 14444;
        }
        if self.r#south == true
            && self.r#up == true
            && self.r#east == false
            && self.r#north == true
            && self.r#down == false
            && self.r#west == true
        {
            return 14488;
        }
        if self.r#down == true
            && self.r#west == false
            && self.r#south == true
            && self.r#east == false
            && self.r#north == true
            && self.r#up == false
        {
            return 14459;
        }
        if self.r#north == false
            && self.r#south == false
            && self.r#west == true
            && self.r#up == true
            && self.r#down == false
            && self.r#east == true
        {
            return 14484;
        }
        if self.r#down == false
            && self.r#up == true
            && self.r#east == false
            && self.r#south == true
            && self.r#west == false
            && self.r#north == true
        {
            return 14489;
        }
        if self.r#down == false
            && self.r#east == false
            && self.r#north == false
            && self.r#south == false
            && self.r#up == true
            && self.r#west == false
        {
            return 14501;
        }
        if self.r#up == false
            && self.r#east == false
            && self.r#west == true
            && self.r#north == true
            && self.r#south == false
            && self.r#down == false
        {
            return 14494;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#up == false
            && self.r#down == false
            && self.r#west == false
            && self.r#north == false
        {
            return 14499;
        }
        if self.r#east == false
            && self.r#up == true
            && self.r#south == false
            && self.r#down == false
            && self.r#west == false
            && self.r#north == true
        {
            return 14493;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#north == true
            && self.r#up == false
            && self.r#west == true
            && self.r#down == false
        {
            return 14478;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#down == false
            && self.r#up == false
            && self.r#east == false
            && self.r#west == false
        {
            return 14491;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#south == false
            && self.r#down == true
            && self.r#north == true
            && self.r#up == false
        {
            return 14462;
        }
        if self.r#up == false
            && self.r#west == false
            && self.r#south == true
            && self.r#down == false
            && self.r#east == true
            && self.r#north == true
        {
            return 14475;
        }
        if self.r#north == true
            && self.r#east == false
            && self.r#down == true
            && self.r#up == false
            && self.r#west == true
            && self.r#south == true
        {
            return 14458;
        }
        if self.r#east == false
            && self.r#up == false
            && self.r#west == false
            && self.r#north == false
            && self.r#south == false
            && self.r#down == true
        {
            return 14471;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#down == true
            && self.r#north == false
            && self.r#south == false
            && self.r#up == true
        {
            return 14468;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#down == false
            && self.r#east == true
            && self.r#south == false
            && self.r#up == true
        {
            return 14485;
        }
        if self.r#down == false
            && self.r#up == false
            && self.r#east == true
            && self.r#north == false
            && self.r#west == true
            && self.r#south == true
        {
            return 14482;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#up == false
            && self.r#south == true
            && self.r#west == false
            && self.r#down == true
        {
            return 14451;
        }
        if self.r#west == true
            && self.r#down == true
            && self.r#north == true
            && self.r#south == true
            && self.r#east == true
            && self.r#up == false
        {
            return 14442;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#south == false
            && self.r#north == false
            && self.r#down == true
            && self.r#up == true
        {
            return 14453;
        }
        if self.r#down == false
            && self.r#south == true
            && self.r#west == false
            && self.r#up == false
            && self.r#north == false
            && self.r#east == true
        {
            return 14483;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#north == false
            && self.r#down == false
            && self.r#east == false
            && self.r#up == true
        {
            return 14500;
        }
        if self.r#up == false
            && self.r#east == true
            && self.r#west == false
            && self.r#down == true
            && self.r#south == false
            && self.r#north == false
        {
            return 14455;
        }
        if self.r#up == true
            && self.r#west == false
            && self.r#east == true
            && self.r#south == true
            && self.r#north == false
            && self.r#down == false
        {
            return 14481;
        }
        if self.r#east == false
            && self.r#down == false
            && self.r#north == false
            && self.r#south == false
            && self.r#up == false
            && self.r#west == false
        {
            return 14503;
        }
        if self.r#south == true
            && self.r#up == true
            && self.r#down == true
            && self.r#west == true
            && self.r#east == false
            && self.r#north == true
        {
            return 14456;
        }
        if self.r#down == true
            && self.r#west == true
            && self.r#east == false
            && self.r#south == true
            && self.r#up == true
            && self.r#north == false
        {
            return 14464;
        }
        if self.r#down == true
            && self.r#up == false
            && self.r#north == true
            && self.r#west == true
            && self.r#south == false
            && self.r#east == true
        {
            return 14446;
        }
        if self.r#east == true
            && self.r#up == false
            && self.r#north == true
            && self.r#south == true
            && self.r#west == true
            && self.r#down == false
        {
            return 14474;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#up == true
            && self.r#south == true
            && self.r#down == false
            && self.r#west == true
        {
            return 14480;
        }
        if self.r#west == false
            && self.r#north == true
            && self.r#south == false
            && self.r#up == false
            && self.r#down == false
            && self.r#east == true
        {
            return 14479;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#down == false
            && self.r#north == false
            && self.r#east == true
            && self.r#up == false
        {
            return 14487;
        }
        if self.r#north == true
            && self.r#down == true
            && self.r#up == false
            && self.r#west == false
            && self.r#south == false
            && self.r#east == false
        {
            return 14463;
        }
        if self.r#east == false
            && self.r#west == false
            && self.r#up == true
            && self.r#down == false
            && self.r#north == false
            && self.r#south == true
        {
            return 14497;
        }
        if self.r#west == false
            && self.r#north == true
            && self.r#up == false
            && self.r#down == true
            && self.r#south == false
            && self.r#east == true
        {
            return 14447;
        }
        if self.r#west == false
            && self.r#south == true
            && self.r#east == false
            && self.r#down == true
            && self.r#up == true
            && self.r#north == true
        {
            return 14457;
        }
        if self.r#south == false
            && self.r#north == true
            && self.r#west == true
            && self.r#down == true
            && self.r#east == false
            && self.r#up == true
        {
            return 14460;
        }
        if self.r#north == false
            && self.r#down == false
            && self.r#south == true
            && self.r#east == false
            && self.r#up == false
            && self.r#west == true
        {
            return 14498;
        }
        if self.r#down == true
            && self.r#west == false
            && self.r#east == true
            && self.r#north == false
            && self.r#up == true
            && self.r#south == true
        {
            return 14449;
        }
        if self.r#up == true
            && self.r#down == true
            && self.r#south == false
            && self.r#east == false
            && self.r#north == false
            && self.r#west == false
        {
            return 14469;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#up == true
            && self.r#down == false
            && self.r#north == true
            && self.r#west == false
        {
            return 14473;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == true
            && self.r#down == true
            && self.r#west == false
            && self.r#up == true
        {
            return 14441;
        }
        if self.r#west == true
            && self.r#down == true
            && self.r#east == true
            && self.r#north == false
            && self.r#south == true
            && self.r#up == true
        {
            return 14448;
        }
        if self.r#down == true
            && self.r#north == false
            && self.r#south == true
            && self.r#west == false
            && self.r#east == false
            && self.r#up == true
        {
            return 14465;
        }
        if self.r#east == false
            && self.r#up == false
            && self.r#west == true
            && self.r#south == false
            && self.r#north == false
            && self.r#down == true
        {
            return 14470;
        }
        if self.r#east == false
            && self.r#down == false
            && self.r#north == false
            && self.r#up == false
            && self.r#west == true
            && self.r#south == false
        {
            return 14502;
        }
        if self.r#up == true
            && self.r#north == true
            && self.r#south == false
            && self.r#west == false
            && self.r#down == false
            && self.r#east == true
        {
            return 14477;
        }
        if self.r#east == false
            && self.r#up == false
            && self.r#north == false
            && self.r#west == false
            && self.r#down == true
            && self.r#south == true
        {
            return 14467;
        }
        if self.r#east == true
            && self.r#down == true
            && self.r#north == true
            && self.r#west == false
            && self.r#up == true
            && self.r#south == false
        {
            return 14445;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14452 {
            return Some(ChorusPlant {
                r#west: true,
                r#north: false,
                r#down: true,
                r#east: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 14486 {
            return Some(ChorusPlant {
                r#east: true,
                r#north: false,
                r#down: false,
                r#up: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 14492 {
            return Some(ChorusPlant {
                r#north: true,
                r#down: false,
                r#up: true,
                r#south: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 14495 {
            return Some(ChorusPlant {
                r#east: false,
                r#north: true,
                r#down: false,
                r#south: false,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 14490 {
            return Some(ChorusPlant {
                r#down: false,
                r#west: true,
                r#east: false,
                r#up: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 14466 {
            return Some(ChorusPlant {
                r#west: true,
                r#south: true,
                r#north: false,
                r#east: false,
                r#down: true,
                r#up: false,
            });
        }
        if state_id == 14440 {
            return Some(ChorusPlant {
                r#down: true,
                r#north: true,
                r#up: true,
                r#south: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 14443 {
            return Some(ChorusPlant {
                r#east: true,
                r#down: true,
                r#south: true,
                r#up: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 14454 {
            return Some(ChorusPlant {
                r#down: true,
                r#east: true,
                r#south: false,
                r#up: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 14496 {
            return Some(ChorusPlant {
                r#up: true,
                r#east: false,
                r#down: false,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 14476 {
            return Some(ChorusPlant {
                r#down: false,
                r#east: true,
                r#south: false,
                r#west: true,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 14450 {
            return Some(ChorusPlant {
                r#north: false,
                r#east: true,
                r#up: false,
                r#south: true,
                r#west: true,
                r#down: true,
            });
        }
        if state_id == 14461 {
            return Some(ChorusPlant {
                r#north: true,
                r#down: true,
                r#south: false,
                r#up: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 14472 {
            return Some(ChorusPlant {
                r#west: true,
                r#down: false,
                r#east: true,
                r#north: true,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 14444 {
            return Some(ChorusPlant {
                r#south: false,
                r#north: true,
                r#west: true,
                r#east: true,
                r#up: true,
                r#down: true,
            });
        }
        if state_id == 14488 {
            return Some(ChorusPlant {
                r#south: true,
                r#up: true,
                r#east: false,
                r#north: true,
                r#down: false,
                r#west: true,
            });
        }
        if state_id == 14459 {
            return Some(ChorusPlant {
                r#down: true,
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 14484 {
            return Some(ChorusPlant {
                r#north: false,
                r#south: false,
                r#west: true,
                r#up: true,
                r#down: false,
                r#east: true,
            });
        }
        if state_id == 14489 {
            return Some(ChorusPlant {
                r#down: false,
                r#up: true,
                r#east: false,
                r#south: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 14501 {
            return Some(ChorusPlant {
                r#down: false,
                r#east: false,
                r#north: false,
                r#south: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 14494 {
            return Some(ChorusPlant {
                r#up: false,
                r#east: false,
                r#west: true,
                r#north: true,
                r#south: false,
                r#down: false,
            });
        }
        if state_id == 14499 {
            return Some(ChorusPlant {
                r#east: false,
                r#south: true,
                r#up: false,
                r#down: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 14493 {
            return Some(ChorusPlant {
                r#east: false,
                r#up: true,
                r#south: false,
                r#down: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 14478 {
            return Some(ChorusPlant {
                r#east: true,
                r#south: false,
                r#north: true,
                r#up: false,
                r#west: true,
                r#down: false,
            });
        }
        if state_id == 14491 {
            return Some(ChorusPlant {
                r#north: true,
                r#south: true,
                r#down: false,
                r#up: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 14462 {
            return Some(ChorusPlant {
                r#west: true,
                r#east: false,
                r#south: false,
                r#down: true,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 14475 {
            return Some(ChorusPlant {
                r#up: false,
                r#west: false,
                r#south: true,
                r#down: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 14458 {
            return Some(ChorusPlant {
                r#north: true,
                r#east: false,
                r#down: true,
                r#up: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 14471 {
            return Some(ChorusPlant {
                r#east: false,
                r#up: false,
                r#west: false,
                r#north: false,
                r#south: false,
                r#down: true,
            });
        }
        if state_id == 14468 {
            return Some(ChorusPlant {
                r#west: true,
                r#east: false,
                r#down: true,
                r#north: false,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 14485 {
            return Some(ChorusPlant {
                r#west: false,
                r#north: false,
                r#down: false,
                r#east: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 14482 {
            return Some(ChorusPlant {
                r#down: false,
                r#up: false,
                r#east: true,
                r#north: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 14451 {
            return Some(ChorusPlant {
                r#north: false,
                r#east: true,
                r#up: false,
                r#south: true,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 14442 {
            return Some(ChorusPlant {
                r#west: true,
                r#down: true,
                r#north: true,
                r#south: true,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 14453 {
            return Some(ChorusPlant {
                r#west: false,
                r#east: true,
                r#south: false,
                r#north: false,
                r#down: true,
                r#up: true,
            });
        }
        if state_id == 14483 {
            return Some(ChorusPlant {
                r#down: false,
                r#south: true,
                r#west: false,
                r#up: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 14500 {
            return Some(ChorusPlant {
                r#south: false,
                r#west: true,
                r#north: false,
                r#down: false,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 14455 {
            return Some(ChorusPlant {
                r#up: false,
                r#east: true,
                r#west: false,
                r#down: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 14481 {
            return Some(ChorusPlant {
                r#up: true,
                r#west: false,
                r#east: true,
                r#south: true,
                r#north: false,
                r#down: false,
            });
        }
        if state_id == 14503 {
            return Some(ChorusPlant {
                r#east: false,
                r#down: false,
                r#north: false,
                r#south: false,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 14456 {
            return Some(ChorusPlant {
                r#south: true,
                r#up: true,
                r#down: true,
                r#west: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 14464 {
            return Some(ChorusPlant {
                r#down: true,
                r#west: true,
                r#east: false,
                r#south: true,
                r#up: true,
                r#north: false,
            });
        }
        if state_id == 14446 {
            return Some(ChorusPlant {
                r#down: true,
                r#up: false,
                r#north: true,
                r#west: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 14474 {
            return Some(ChorusPlant {
                r#east: true,
                r#up: false,
                r#north: true,
                r#south: true,
                r#west: true,
                r#down: false,
            });
        }
        if state_id == 14480 {
            return Some(ChorusPlant {
                r#east: true,
                r#north: false,
                r#up: true,
                r#south: true,
                r#down: false,
                r#west: true,
            });
        }
        if state_id == 14479 {
            return Some(ChorusPlant {
                r#west: false,
                r#north: true,
                r#south: false,
                r#up: false,
                r#down: false,
                r#east: true,
            });
        }
        if state_id == 14487 {
            return Some(ChorusPlant {
                r#west: false,
                r#south: false,
                r#down: false,
                r#north: false,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 14463 {
            return Some(ChorusPlant {
                r#north: true,
                r#down: true,
                r#up: false,
                r#west: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 14497 {
            return Some(ChorusPlant {
                r#east: false,
                r#west: false,
                r#up: true,
                r#down: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 14447 {
            return Some(ChorusPlant {
                r#west: false,
                r#north: true,
                r#up: false,
                r#down: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 14457 {
            return Some(ChorusPlant {
                r#west: false,
                r#south: true,
                r#east: false,
                r#down: true,
                r#up: true,
                r#north: true,
            });
        }
        if state_id == 14460 {
            return Some(ChorusPlant {
                r#south: false,
                r#north: true,
                r#west: true,
                r#down: true,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 14498 {
            return Some(ChorusPlant {
                r#north: false,
                r#down: false,
                r#south: true,
                r#east: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 14449 {
            return Some(ChorusPlant {
                r#down: true,
                r#west: false,
                r#east: true,
                r#north: false,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 14469 {
            return Some(ChorusPlant {
                r#up: true,
                r#down: true,
                r#south: false,
                r#east: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 14473 {
            return Some(ChorusPlant {
                r#south: true,
                r#east: true,
                r#up: true,
                r#down: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 14441 {
            return Some(ChorusPlant {
                r#north: true,
                r#south: true,
                r#east: true,
                r#down: true,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 14448 {
            return Some(ChorusPlant {
                r#west: true,
                r#down: true,
                r#east: true,
                r#north: false,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 14465 {
            return Some(ChorusPlant {
                r#down: true,
                r#north: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 14470 {
            return Some(ChorusPlant {
                r#east: false,
                r#up: false,
                r#west: true,
                r#south: false,
                r#north: false,
                r#down: true,
            });
        }
        if state_id == 14502 {
            return Some(ChorusPlant {
                r#east: false,
                r#down: false,
                r#north: false,
                r#up: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 14477 {
            return Some(ChorusPlant {
                r#up: true,
                r#north: true,
                r#south: false,
                r#west: false,
                r#down: false,
                r#east: true,
            });
        }
        if state_id == 14467 {
            return Some(ChorusPlant {
                r#east: false,
                r#up: false,
                r#north: false,
                r#west: false,
                r#down: true,
                r#south: true,
            });
        }
        if state_id == 14445 {
            return Some(ChorusPlant {
                r#east: true,
                r#down: true,
                r#north: true,
                r#west: false,
                r#up: true,
                r#south: false,
            });
        }
        return None;
    }
}
