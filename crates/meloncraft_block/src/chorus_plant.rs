use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChorusPlant {
    pub north: bool,
    pub up: bool,
    pub west: bool,
    pub east: bool,
    pub south: bool,
    pub down: bool,
}


impl BlockState for ChorusPlant {
    fn to_id(self) -> i32 {
        if block_state.r#down == true && block_state.r#west == false && block_state.r#up == false && block_state.r#north == true && block_state.r#east == true && block_state.r#south == false { return 14447; }
        if block_state.r#north == true && block_state.r#down == false && block_state.r#east == true && block_state.r#up == false && block_state.r#west == false && block_state.r#south == false { return 14479; }
        if block_state.r#west == false && block_state.r#down == false && block_state.r#north == true && block_state.r#up == true && block_state.r#south == true && block_state.r#east == false { return 14489; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#west == false && block_state.r#down == false && block_state.r#up == false && block_state.r#east == false { return 14491; }
        if block_state.r#north == true && block_state.r#up == false && block_state.r#east == false && block_state.r#south == false && block_state.r#down == false && block_state.r#west == false { return 14495; }
        if block_state.r#north == false && block_state.r#down == false && block_state.r#east == true && block_state.r#south == false && block_state.r#up == false && block_state.r#west == false { return 14487; }
        if block_state.r#down == true && block_state.r#east == false && block_state.r#south == false && block_state.r#north == true && block_state.r#up == true && block_state.r#west == true { return 14460; }
        if block_state.r#up == false && block_state.r#south == false && block_state.r#north == true && block_state.r#west == false && block_state.r#down == true && block_state.r#east == false { return 14463; }
        if block_state.r#west == true && block_state.r#down == false && block_state.r#up == true && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false { return 14492; }
        if block_state.r#down == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == true && block_state.r#east == true { return 14441; }
        if block_state.r#up == false && block_state.r#west == false && block_state.r#north == false && block_state.r#east == false && block_state.r#down == true && block_state.r#south == true { return 14467; }
        if block_state.r#east == true && block_state.r#down == true && block_state.r#north == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true { return 14440; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#west == false && block_state.r#east == true && block_state.r#up == false && block_state.r#down == true { return 14451; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#down == true { return 14452; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#down == true && block_state.r#west == false && block_state.r#up == true && block_state.r#south == true { return 14457; }
        if block_state.r#down == false && block_state.r#east == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#up == false { return 14499; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#down == false { return 14483; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == false && block_state.r#down == true { return 14449; }
        if block_state.r#north == true && block_state.r#down == true && block_state.r#east == false && block_state.r#up == false && block_state.r#west == false && block_state.r#south == true { return 14459; }
        if block_state.r#east == false && block_state.r#up == true && block_state.r#south == false && block_state.r#north == true && block_state.r#west == false && block_state.r#down == true { return 14461; }
        if block_state.r#east == true && block_state.r#down == true && block_state.r#south == false && block_state.r#west == true && block_state.r#north == true && block_state.r#up == false { return 14446; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#west == false && block_state.r#up == true && block_state.r#down == true { return 14465; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == false && block_state.r#north == false && block_state.r#down == false { return 14501; }
        if block_state.r#west == true && block_state.r#down == false && block_state.r#south == false && block_state.r#east == true && block_state.r#north == true && block_state.r#up == true { return 14476; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#west == true && block_state.r#down == false && block_state.r#east == true && block_state.r#up == false { return 14478; }
        if block_state.r#up == false && block_state.r#north == false && block_state.r#west == true && block_state.r#down == false && block_state.r#east == true && block_state.r#south == true { return 14482; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#down == false && block_state.r#up == false && block_state.r#north == true && block_state.r#south == true { return 14474; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#up == false && block_state.r#down == false && block_state.r#north == false && block_state.r#east == false { return 14502; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#up == true && block_state.r#east == true && block_state.r#down == true && block_state.r#west == false { return 14445; }
        if block_state.r#north == false && block_state.r#down == true && block_state.r#east == true && block_state.r#south == false && block_state.r#west == true && block_state.r#up == false { return 14454; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#north == false && block_state.r#down == false && block_state.r#east == false && block_state.r#south == true { return 14496; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#down == false && block_state.r#north == false && block_state.r#east == true && block_state.r#up == true { return 14484; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#south == false && block_state.r#down == false && block_state.r#north == false && block_state.r#up == false { return 14486; }
        if block_state.r#east == true && block_state.r#up == false && block_state.r#west == true && block_state.r#north == true && block_state.r#down == true && block_state.r#south == true { return 14442; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#down == false && block_state.r#south == false && block_state.r#east == true && block_state.r#up == true { return 14485; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#up == false && block_state.r#west == true && block_state.r#down == false { return 14494; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#east == true && block_state.r#down == true && block_state.r#up == false && block_state.r#south == true { return 14443; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#down == true && block_state.r#up == true { return 14469; }
        if block_state.r#south == false && block_state.r#north == false && block_state.r#east == true && block_state.r#down == true && block_state.r#up == true && block_state.r#west == false { return 14453; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#down == false && block_state.r#up == true && block_state.r#east == false && block_state.r#south == true { return 14488; }
        if block_state.r#north == false && block_state.r#up == true && block_state.r#east == true && block_state.r#down == false && block_state.r#west == false && block_state.r#south == true { return 14481; }
        if block_state.r#south == false && block_state.r#up == true && block_state.r#down == false && block_state.r#east == false && block_state.r#north == true && block_state.r#west == false { return 14493; }
        if block_state.r#down == true && block_state.r#up == false && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#west == true { return 14458; }
        if block_state.r#down == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false { return 14448; }
        if block_state.r#down == true && block_state.r#north == true && block_state.r#west == true && block_state.r#up == false && block_state.r#south == false && block_state.r#east == false { return 14462; }
        if block_state.r#west == true && block_state.r#down == false && block_state.r#up == true && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true { return 14472; }
        if block_state.r#west == true && block_state.r#down == true && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#up == false { return 14470; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#west == true && block_state.r#east == false && block_state.r#down == false && block_state.r#north == false { return 14498; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#down == false && block_state.r#north == false && block_state.r#south == true && block_state.r#up == true { return 14480; }
        if block_state.r#up == false && block_state.r#down == false && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#west == true { return 14490; }
        if block_state.r#down == true && block_state.r#east == true && block_state.r#south == false && block_state.r#up == true && block_state.r#north == true && block_state.r#west == true { return 14444; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#north == false && block_state.r#up == true && block_state.r#down == false && block_state.r#south == false { return 14500; }
        if block_state.r#down == true && block_state.r#east == false && block_state.r#south == false && block_state.r#north == false && block_state.r#up == false && block_state.r#west == false { return 14471; }
        if block_state.r#south == true && block_state.r#down == false && block_state.r#east == false && block_state.r#up == true && block_state.r#north == false && block_state.r#west == false { return 14497; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#up == true && block_state.r#north == false && block_state.r#west == true && block_state.r#down == true { return 14464; }
        if block_state.r#north == false && block_state.r#up == false && block_state.r#down == true && block_state.r#south == false && block_state.r#west == false && block_state.r#east == true { return 14455; }
        if block_state.r#south == true && block_state.r#down == true && block_state.r#up == false && block_state.r#west == true && block_state.r#north == false && block_state.r#east == false { return 14466; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#up == true && block_state.r#down == true && block_state.r#west == true && block_state.r#north == false { return 14468; }
        if block_state.r#south == false && block_state.r#down == false && block_state.r#up == false && block_state.r#west == false && block_state.r#north == false && block_state.r#east == false { return 14503; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#east == true && block_state.r#down == true && block_state.r#up == false && block_state.r#south == true { return 14450; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#up == false && block_state.r#down == false && block_state.r#west == false && block_state.r#north == true { return 14475; }
        if block_state.r#up == true && block_state.r#north == true && block_state.r#down == false && block_state.r#east == true && block_state.r#west == false && block_state.r#south == false { return 14477; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true && block_state.r#north == true && block_state.r#down == true { return 14456; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#east == true && block_state.r#up == true && block_state.r#down == false && block_state.r#north == true { return 14473; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14447 {
            return Some(ChorusPlant {
                r#down: true,
                r#west: false,
                r#up: false,
                r#north: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 14479 {
            return Some(ChorusPlant {
                r#north: true,
                r#down: false,
                r#east: true,
                r#up: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 14489 {
            return Some(ChorusPlant {
                r#west: false,
                r#down: false,
                r#north: true,
                r#up: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 14491 {
            return Some(ChorusPlant {
                r#south: true,
                r#north: true,
                r#west: false,
                r#down: false,
                r#up: false,
                r#east: false,
            });
        }
        if state_id == 14495 {
            return Some(ChorusPlant {
                r#north: true,
                r#up: false,
                r#east: false,
                r#south: false,
                r#down: false,
                r#west: false,
            });
        }
        if state_id == 14487 {
            return Some(ChorusPlant {
                r#north: false,
                r#down: false,
                r#east: true,
                r#south: false,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 14460 {
            return Some(ChorusPlant {
                r#down: true,
                r#east: false,
                r#south: false,
                r#north: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 14463 {
            return Some(ChorusPlant {
                r#up: false,
                r#south: false,
                r#north: true,
                r#west: false,
                r#down: true,
                r#east: false,
            });
        }
        if state_id == 14492 {
            return Some(ChorusPlant {
                r#west: true,
                r#down: false,
                r#up: true,
                r#east: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 14441 {
            return Some(ChorusPlant {
                r#down: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 14467 {
            return Some(ChorusPlant {
                r#up: false,
                r#west: false,
                r#north: false,
                r#east: false,
                r#down: true,
                r#south: true,
            });
        }
        if state_id == 14440 {
            return Some(ChorusPlant {
                r#east: true,
                r#down: true,
                r#north: true,
                r#south: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 14451 {
            return Some(ChorusPlant {
                r#south: true,
                r#north: false,
                r#west: false,
                r#east: true,
                r#up: false,
                r#down: true,
            });
        }
        if state_id == 14452 {
            return Some(ChorusPlant {
                r#up: true,
                r#west: true,
                r#north: false,
                r#south: false,
                r#east: true,
                r#down: true,
            });
        }
        if state_id == 14457 {
            return Some(ChorusPlant {
                r#east: false,
                r#north: true,
                r#down: true,
                r#west: false,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 14499 {
            return Some(ChorusPlant {
                r#down: false,
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 14483 {
            return Some(ChorusPlant {
                r#south: true,
                r#up: false,
                r#west: false,
                r#east: true,
                r#north: false,
                r#down: false,
            });
        }
        if state_id == 14449 {
            return Some(ChorusPlant {
                r#east: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#north: false,
                r#down: true,
            });
        }
        if state_id == 14459 {
            return Some(ChorusPlant {
                r#north: true,
                r#down: true,
                r#east: false,
                r#up: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 14461 {
            return Some(ChorusPlant {
                r#east: false,
                r#up: true,
                r#south: false,
                r#north: true,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 14446 {
            return Some(ChorusPlant {
                r#east: true,
                r#down: true,
                r#south: false,
                r#west: true,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 14465 {
            return Some(ChorusPlant {
                r#east: false,
                r#north: false,
                r#south: true,
                r#west: false,
                r#up: true,
                r#down: true,
            });
        }
        if state_id == 14501 {
            return Some(ChorusPlant {
                r#up: true,
                r#west: false,
                r#east: false,
                r#south: false,
                r#north: false,
                r#down: false,
            });
        }
        if state_id == 14476 {
            return Some(ChorusPlant {
                r#west: true,
                r#down: false,
                r#south: false,
                r#east: true,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 14478 {
            return Some(ChorusPlant {
                r#south: false,
                r#north: true,
                r#west: true,
                r#down: false,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 14482 {
            return Some(ChorusPlant {
                r#up: false,
                r#north: false,
                r#west: true,
                r#down: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 14474 {
            return Some(ChorusPlant {
                r#west: true,
                r#east: true,
                r#down: false,
                r#up: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 14502 {
            return Some(ChorusPlant {
                r#west: true,
                r#south: false,
                r#up: false,
                r#down: false,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 14445 {
            return Some(ChorusPlant {
                r#south: false,
                r#north: true,
                r#up: true,
                r#east: true,
                r#down: true,
                r#west: false,
            });
        }
        if state_id == 14454 {
            return Some(ChorusPlant {
                r#north: false,
                r#down: true,
                r#east: true,
                r#south: false,
                r#west: true,
                r#up: false,
            });
        }
        if state_id == 14496 {
            return Some(ChorusPlant {
                r#up: true,
                r#west: true,
                r#north: false,
                r#down: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 14484 {
            return Some(ChorusPlant {
                r#west: true,
                r#south: false,
                r#down: false,
                r#north: false,
                r#east: true,
                r#up: true,
            });
        }
        if state_id == 14486 {
            return Some(ChorusPlant {
                r#west: true,
                r#east: true,
                r#south: false,
                r#down: false,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 14442 {
            return Some(ChorusPlant {
                r#east: true,
                r#up: false,
                r#west: true,
                r#north: true,
                r#down: true,
                r#south: true,
            });
        }
        if state_id == 14485 {
            return Some(ChorusPlant {
                r#west: false,
                r#north: false,
                r#down: false,
                r#south: false,
                r#east: true,
                r#up: true,
            });
        }
        if state_id == 14494 {
            return Some(ChorusPlant {
                r#east: false,
                r#north: true,
                r#south: false,
                r#up: false,
                r#west: true,
                r#down: false,
            });
        }
        if state_id == 14443 {
            return Some(ChorusPlant {
                r#west: false,
                r#north: true,
                r#east: true,
                r#down: true,
                r#up: false,
                r#south: true,
            });
        }
        if state_id == 14469 {
            return Some(ChorusPlant {
                r#west: false,
                r#east: false,
                r#north: false,
                r#south: false,
                r#down: true,
                r#up: true,
            });
        }
        if state_id == 14453 {
            return Some(ChorusPlant {
                r#south: false,
                r#north: false,
                r#east: true,
                r#down: true,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 14488 {
            return Some(ChorusPlant {
                r#west: true,
                r#north: true,
                r#down: false,
                r#up: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 14481 {
            return Some(ChorusPlant {
                r#north: false,
                r#up: true,
                r#east: true,
                r#down: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 14493 {
            return Some(ChorusPlant {
                r#south: false,
                r#up: true,
                r#down: false,
                r#east: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 14458 {
            return Some(ChorusPlant {
                r#down: true,
                r#up: false,
                r#north: true,
                r#east: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 14448 {
            return Some(ChorusPlant {
                r#down: true,
                r#south: true,
                r#up: true,
                r#west: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 14462 {
            return Some(ChorusPlant {
                r#down: true,
                r#north: true,
                r#west: true,
                r#up: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 14472 {
            return Some(ChorusPlant {
                r#west: true,
                r#down: false,
                r#up: true,
                r#east: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 14470 {
            return Some(ChorusPlant {
                r#west: true,
                r#down: true,
                r#east: false,
                r#north: false,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 14498 {
            return Some(ChorusPlant {
                r#south: true,
                r#up: false,
                r#west: true,
                r#east: false,
                r#down: false,
                r#north: false,
            });
        }
        if state_id == 14480 {
            return Some(ChorusPlant {
                r#east: true,
                r#west: true,
                r#down: false,
                r#north: false,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 14490 {
            return Some(ChorusPlant {
                r#up: false,
                r#down: false,
                r#north: true,
                r#east: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 14444 {
            return Some(ChorusPlant {
                r#down: true,
                r#east: true,
                r#south: false,
                r#up: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 14500 {
            return Some(ChorusPlant {
                r#west: true,
                r#east: false,
                r#north: false,
                r#up: true,
                r#down: false,
                r#south: false,
            });
        }
        if state_id == 14471 {
            return Some(ChorusPlant {
                r#down: true,
                r#east: false,
                r#south: false,
                r#north: false,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 14497 {
            return Some(ChorusPlant {
                r#south: true,
                r#down: false,
                r#east: false,
                r#up: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 14464 {
            return Some(ChorusPlant {
                r#east: false,
                r#south: true,
                r#up: true,
                r#north: false,
                r#west: true,
                r#down: true,
            });
        }
        if state_id == 14455 {
            return Some(ChorusPlant {
                r#north: false,
                r#up: false,
                r#down: true,
                r#south: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 14466 {
            return Some(ChorusPlant {
                r#south: true,
                r#down: true,
                r#up: false,
                r#west: true,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 14468 {
            return Some(ChorusPlant {
                r#east: false,
                r#south: false,
                r#up: true,
                r#down: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 14503 {
            return Some(ChorusPlant {
                r#south: false,
                r#down: false,
                r#up: false,
                r#west: false,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 14450 {
            return Some(ChorusPlant {
                r#west: true,
                r#north: false,
                r#east: true,
                r#down: true,
                r#up: false,
                r#south: true,
            });
        }
        if state_id == 14475 {
            return Some(ChorusPlant {
                r#south: true,
                r#east: true,
                r#up: false,
                r#down: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 14477 {
            return Some(ChorusPlant {
                r#up: true,
                r#north: true,
                r#down: false,
                r#east: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 14456 {
            return Some(ChorusPlant {
                r#east: false,
                r#south: true,
                r#up: true,
                r#west: true,
                r#north: true,
                r#down: true,
            });
        }
        if state_id == 14473 {
            return Some(ChorusPlant {
                r#south: true,
                r#west: false,
                r#east: true,
                r#up: true,
                r#down: false,
                r#north: true,
            });
        }
        return None;
    }
}

