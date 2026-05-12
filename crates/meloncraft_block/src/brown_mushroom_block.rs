use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownMushroomBlock {
    pub south: bool,
    pub down: bool,
    pub east: bool,
    pub west: bool,
    pub north: bool,
    pub up: bool,
}


impl BlockState for BrownMushroomBlock {
    fn to_id(self) -> i32 {
        if block_state.r#down == false && block_state.r#north == false && block_state.r#south == true && block_state.r#east == true && block_state.r#up == true && block_state.r#west == true { return 7605; }
        if block_state.r#up == false && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#down == false { return 7628; }
        if block_state.r#north == false && block_state.r#up == true && block_state.r#south == false && block_state.r#down == true && block_state.r#east == false && block_state.r#west == true { return 7593; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#north == false && block_state.r#east == true && block_state.r#down == true && block_state.r#up == false { return 7576; }
        if block_state.r#north == true && block_state.r#up == true && block_state.r#east == true && block_state.r#down == false && block_state.r#south == true && block_state.r#west == false { return 7598; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#down == false && block_state.r#west == false { return 7604; }
        if block_state.r#down == false && block_state.r#south == false && block_state.r#up == true && block_state.r#north == true && block_state.r#east == false && block_state.r#west == false { return 7618; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#south == false && block_state.r#up == false && block_state.r#west == false && block_state.r#down == true { return 7580; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#down == false { return 7615; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#north == true && block_state.r#down == true && block_state.r#east == true && block_state.r#up == false { return 7572; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#down == true && block_state.r#west == false && block_state.r#north == true && block_state.r#east == false { return 7584; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#down == true && block_state.r#up == false && block_state.r#west == true && block_state.r#south == false { return 7587; }
        if block_state.r#up == false && block_state.r#west == true && block_state.r#north == true && block_state.r#down == false && block_state.r#east == false && block_state.r#south == false { return 7619; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#east == false && block_state.r#west == false && block_state.r#down == false && block_state.r#south == false { return 7620; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#down == true && block_state.r#east == false && block_state.r#up == false && block_state.r#west == true { return 7591; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#east == false && block_state.r#up == false && block_state.r#down == true && block_state.r#west == false { return 7592; }
        if block_state.r#up == false && block_state.r#south == false && block_state.r#east == true && block_state.r#west == true && block_state.r#down == true && block_state.r#north == false { return 7579; }
        if block_state.r#west == true && block_state.r#up == false && block_state.r#east == true && block_state.r#north == false && block_state.r#down == false && block_state.r#south == true { return 7607; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#east == false && block_state.r#north == false && block_state.r#down == false && block_state.r#south == true { return 7622; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#up == true && block_state.r#down == false && block_state.r#east == true && block_state.r#west == true { return 7597; }
        if block_state.r#down == true && block_state.r#east == false && block_state.r#up == false && block_state.r#west == true && block_state.r#south == true && block_state.r#north == true { return 7583; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#east == true && block_state.r#up == false && block_state.r#down == false && block_state.r#north == false { return 7611; }
        if block_state.r#down == false && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true && block_state.r#east == false && block_state.r#north == false { return 7625; }
        if block_state.r#down == true && block_state.r#up == true && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true { return 7569; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#down == false && block_state.r#up == false && block_state.r#west == true && block_state.r#north == true { return 7599; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#up == true && block_state.r#west == false && block_state.r#down == true { return 7594; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#south == true && block_state.r#up == false && block_state.r#north == false && block_state.r#down == false { return 7608; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#down == true && block_state.r#up == false { return 7567; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true && block_state.r#up == false && block_state.r#down == false { return 7600; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#down == false && block_state.r#north == false { return 7606; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#down == true && block_state.r#east == false { return 7588; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#south == true && block_state.r#up == true && block_state.r#north == true && block_state.r#down == true { return 7566; }
        if block_state.r#down == false && block_state.r#south == false && block_state.r#north == false && block_state.r#west == true && block_state.r#east == false && block_state.r#up == false { return 7627; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#down == true && block_state.r#south == true && block_state.r#up == false && block_state.r#west == true { return 7575; }
        if block_state.r#down == false && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true { return 7601; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#east == true && block_state.r#south == true && block_state.r#up == true && block_state.r#down == true { return 7573; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#north == false && block_state.r#east == false && block_state.r#down == false && block_state.r#up == false { return 7623; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#north == false && block_state.r#up == true && block_state.r#down == false && block_state.r#south == true { return 7621; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#north == true && block_state.r#down == true && block_state.r#south == false && block_state.r#east == true { return 7570; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#east == false && block_state.r#down == true && block_state.r#south == true && block_state.r#up == true { return 7590; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#east == true && block_state.r#south == false && block_state.r#down == false && block_state.r#north == true { return 7602; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#down == false && block_state.r#up == true && block_state.r#west == false && block_state.r#east == true { return 7610; }
        if block_state.r#up == true && block_state.r#east == false && block_state.r#west == true && block_state.r#down == true && block_state.r#south == false && block_state.r#north == true { return 7585; }
        if block_state.r#west == false && block_state.r#down == false && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#up == false { return 7624; }
        if block_state.r#down == true && block_state.r#west == false && block_state.r#south == true && block_state.r#up == true && block_state.r#north == false && block_state.r#east == true { return 7574; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#north == false && block_state.r#west == false && block_state.r#down == true && block_state.r#up == true { return 7578; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#up == false && block_state.r#east == true && block_state.r#down == true && block_state.r#south == false { return 7571; }
        if block_state.r#down == false && block_state.r#east == true && block_state.r#west == true && block_state.r#south == false && block_state.r#up == true && block_state.r#north == false { return 7609; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#north == false && block_state.r#east == false && block_state.r#down == true && block_state.r#up == true { return 7589; }
        if block_state.r#down == true && block_state.r#west == true && block_state.r#east == true && block_state.r#up == true && block_state.r#north == true && block_state.r#south == true { return 7565; }
        if block_state.r#north == true && block_state.r#up == true && block_state.r#east == false && block_state.r#down == true && block_state.r#south == true && block_state.r#west == true { return 7581; }
        if block_state.r#down == false && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#up == false && block_state.r#west == true { return 7603; }
        if block_state.r#west == false && block_state.r#down == true && block_state.r#up == true && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true { return 7582; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#west == false && block_state.r#down == false && block_state.r#north == true && block_state.r#up == true { return 7614; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#down == true && block_state.r#south == true && block_state.r#up == false && block_state.r#west == false { return 7568; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#up == false && block_state.r#west == false && block_state.r#down == false && block_state.r#south == false { return 7612; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#up == false && block_state.r#down == true && block_state.r#west == true { return 7595; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#up == false && block_state.r#down == true && block_state.r#north == false && block_state.r#south == false { return 7596; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true && block_state.r#north == true && block_state.r#down == false { return 7613; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#up == false && block_state.r#down == false && block_state.r#south == true && block_state.r#north == true { return 7616; }
        if block_state.r#south == false && block_state.r#down == true && block_state.r#west == false && block_state.r#north == true && block_state.r#east == false && block_state.r#up == true { return 7586; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#down == true && block_state.r#up == true && block_state.r#west == true && block_state.r#south == false { return 7577; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#north == false && block_state.r#down == false && block_state.r#south == false && block_state.r#east == false { return 7626; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#up == true && block_state.r#south == false && block_state.r#down == false && block_state.r#north == true { return 7617; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7605 {
            return Some(BrownMushroomBlock {
                r#down: false,
                r#north: false,
                r#south: true,
                r#east: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 7628 {
            return Some(BrownMushroomBlock {
                r#up: false,
                r#east: false,
                r#north: false,
                r#south: false,
                r#west: false,
                r#down: false,
            });
        }
        if state_id == 7593 {
            return Some(BrownMushroomBlock {
                r#north: false,
                r#up: true,
                r#south: false,
                r#down: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 7576 {
            return Some(BrownMushroomBlock {
                r#south: true,
                r#west: false,
                r#north: false,
                r#east: true,
                r#down: true,
                r#up: false,
            });
        }
        if state_id == 7598 {
            return Some(BrownMushroomBlock {
                r#north: true,
                r#up: true,
                r#east: true,
                r#down: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7604 {
            return Some(BrownMushroomBlock {
                r#up: false,
                r#north: true,
                r#east: true,
                r#south: false,
                r#down: false,
                r#west: false,
            });
        }
        if state_id == 7618 {
            return Some(BrownMushroomBlock {
                r#down: false,
                r#south: false,
                r#up: true,
                r#north: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7580 {
            return Some(BrownMushroomBlock {
                r#north: false,
                r#east: true,
                r#south: false,
                r#up: false,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 7615 {
            return Some(BrownMushroomBlock {
                r#up: false,
                r#north: true,
                r#south: true,
                r#east: false,
                r#west: true,
                r#down: false,
            });
        }
        if state_id == 7572 {
            return Some(BrownMushroomBlock {
                r#south: false,
                r#west: false,
                r#north: true,
                r#down: true,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 7584 {
            return Some(BrownMushroomBlock {
                r#south: true,
                r#up: false,
                r#down: true,
                r#west: false,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 7587 {
            return Some(BrownMushroomBlock {
                r#east: false,
                r#north: true,
                r#down: true,
                r#up: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 7619 {
            return Some(BrownMushroomBlock {
                r#up: false,
                r#west: true,
                r#north: true,
                r#down: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 7620 {
            return Some(BrownMushroomBlock {
                r#up: false,
                r#north: true,
                r#east: false,
                r#west: false,
                r#down: false,
                r#south: false,
            });
        }
        if state_id == 7591 {
            return Some(BrownMushroomBlock {
                r#south: true,
                r#north: false,
                r#down: true,
                r#east: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 7592 {
            return Some(BrownMushroomBlock {
                r#north: false,
                r#south: true,
                r#east: false,
                r#up: false,
                r#down: true,
                r#west: false,
            });
        }
        if state_id == 7579 {
            return Some(BrownMushroomBlock {
                r#up: false,
                r#south: false,
                r#east: true,
                r#west: true,
                r#down: true,
                r#north: false,
            });
        }
        if state_id == 7607 {
            return Some(BrownMushroomBlock {
                r#west: true,
                r#up: false,
                r#east: true,
                r#north: false,
                r#down: false,
                r#south: true,
            });
        }
        if state_id == 7622 {
            return Some(BrownMushroomBlock {
                r#up: true,
                r#west: false,
                r#east: false,
                r#north: false,
                r#down: false,
                r#south: true,
            });
        }
        if state_id == 7597 {
            return Some(BrownMushroomBlock {
                r#south: true,
                r#north: true,
                r#up: true,
                r#down: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 7583 {
            return Some(BrownMushroomBlock {
                r#down: true,
                r#east: false,
                r#up: false,
                r#west: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 7611 {
            return Some(BrownMushroomBlock {
                r#south: false,
                r#west: true,
                r#east: true,
                r#up: false,
                r#down: false,
                r#north: false,
            });
        }
        if state_id == 7625 {
            return Some(BrownMushroomBlock {
                r#down: false,
                r#south: false,
                r#up: true,
                r#west: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 7569 {
            return Some(BrownMushroomBlock {
                r#down: true,
                r#up: true,
                r#east: true,
                r#north: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 7599 {
            return Some(BrownMushroomBlock {
                r#east: true,
                r#south: true,
                r#down: false,
                r#up: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7594 {
            return Some(BrownMushroomBlock {
                r#east: false,
                r#north: false,
                r#south: false,
                r#up: true,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 7608 {
            return Some(BrownMushroomBlock {
                r#west: false,
                r#east: true,
                r#south: true,
                r#up: false,
                r#north: false,
                r#down: false,
            });
        }
        if state_id == 7567 {
            return Some(BrownMushroomBlock {
                r#west: true,
                r#south: true,
                r#east: true,
                r#north: true,
                r#down: true,
                r#up: false,
            });
        }
        if state_id == 7600 {
            return Some(BrownMushroomBlock {
                r#south: true,
                r#west: false,
                r#east: true,
                r#north: true,
                r#up: false,
                r#down: false,
            });
        }
        if state_id == 7606 {
            return Some(BrownMushroomBlock {
                r#east: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#down: false,
                r#north: false,
            });
        }
        if state_id == 7588 {
            return Some(BrownMushroomBlock {
                r#up: false,
                r#north: true,
                r#south: false,
                r#west: false,
                r#down: true,
                r#east: false,
            });
        }
        if state_id == 7566 {
            return Some(BrownMushroomBlock {
                r#east: true,
                r#west: false,
                r#south: true,
                r#up: true,
                r#north: true,
                r#down: true,
            });
        }
        if state_id == 7627 {
            return Some(BrownMushroomBlock {
                r#down: false,
                r#south: false,
                r#north: false,
                r#west: true,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 7575 {
            return Some(BrownMushroomBlock {
                r#north: false,
                r#east: true,
                r#down: true,
                r#south: true,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 7601 {
            return Some(BrownMushroomBlock {
                r#down: false,
                r#east: true,
                r#north: true,
                r#south: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 7573 {
            return Some(BrownMushroomBlock {
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: true,
                r#up: true,
                r#down: true,
            });
        }
        if state_id == 7623 {
            return Some(BrownMushroomBlock {
                r#west: true,
                r#south: true,
                r#north: false,
                r#east: false,
                r#down: false,
                r#up: false,
            });
        }
        if state_id == 7621 {
            return Some(BrownMushroomBlock {
                r#west: true,
                r#east: false,
                r#north: false,
                r#up: true,
                r#down: false,
                r#south: true,
            });
        }
        if state_id == 7570 {
            return Some(BrownMushroomBlock {
                r#up: true,
                r#west: false,
                r#north: true,
                r#down: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 7590 {
            return Some(BrownMushroomBlock {
                r#north: false,
                r#west: false,
                r#east: false,
                r#down: true,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 7602 {
            return Some(BrownMushroomBlock {
                r#up: true,
                r#west: false,
                r#east: true,
                r#south: false,
                r#down: false,
                r#north: true,
            });
        }
        if state_id == 7610 {
            return Some(BrownMushroomBlock {
                r#north: false,
                r#south: false,
                r#down: false,
                r#up: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 7585 {
            return Some(BrownMushroomBlock {
                r#up: true,
                r#east: false,
                r#west: true,
                r#down: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 7624 {
            return Some(BrownMushroomBlock {
                r#west: false,
                r#down: false,
                r#east: false,
                r#north: false,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 7574 {
            return Some(BrownMushroomBlock {
                r#down: true,
                r#west: false,
                r#south: true,
                r#up: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 7578 {
            return Some(BrownMushroomBlock {
                r#south: false,
                r#east: true,
                r#north: false,
                r#west: false,
                r#down: true,
                r#up: true,
            });
        }
        if state_id == 7571 {
            return Some(BrownMushroomBlock {
                r#north: true,
                r#west: true,
                r#up: false,
                r#east: true,
                r#down: true,
                r#south: false,
            });
        }
        if state_id == 7609 {
            return Some(BrownMushroomBlock {
                r#down: false,
                r#east: true,
                r#west: true,
                r#south: false,
                r#up: true,
                r#north: false,
            });
        }
        if state_id == 7589 {
            return Some(BrownMushroomBlock {
                r#south: true,
                r#west: true,
                r#north: false,
                r#east: false,
                r#down: true,
                r#up: true,
            });
        }
        if state_id == 7565 {
            return Some(BrownMushroomBlock {
                r#down: true,
                r#west: true,
                r#east: true,
                r#up: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 7581 {
            return Some(BrownMushroomBlock {
                r#north: true,
                r#up: true,
                r#east: false,
                r#down: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 7603 {
            return Some(BrownMushroomBlock {
                r#down: false,
                r#east: true,
                r#north: true,
                r#south: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 7582 {
            return Some(BrownMushroomBlock {
                r#west: false,
                r#down: true,
                r#up: true,
                r#east: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 7614 {
            return Some(BrownMushroomBlock {
                r#east: false,
                r#south: true,
                r#west: false,
                r#down: false,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 7568 {
            return Some(BrownMushroomBlock {
                r#north: true,
                r#east: true,
                r#down: true,
                r#south: true,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 7612 {
            return Some(BrownMushroomBlock {
                r#east: true,
                r#north: false,
                r#up: false,
                r#west: false,
                r#down: false,
                r#south: false,
            });
        }
        if state_id == 7595 {
            return Some(BrownMushroomBlock {
                r#east: false,
                r#north: false,
                r#south: false,
                r#up: false,
                r#down: true,
                r#west: true,
            });
        }
        if state_id == 7596 {
            return Some(BrownMushroomBlock {
                r#west: false,
                r#east: false,
                r#up: false,
                r#down: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 7613 {
            return Some(BrownMushroomBlock {
                r#east: false,
                r#south: true,
                r#up: true,
                r#west: true,
                r#north: true,
                r#down: false,
            });
        }
        if state_id == 7616 {
            return Some(BrownMushroomBlock {
                r#west: false,
                r#east: false,
                r#up: false,
                r#down: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 7586 {
            return Some(BrownMushroomBlock {
                r#south: false,
                r#down: true,
                r#west: false,
                r#north: true,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 7577 {
            return Some(BrownMushroomBlock {
                r#north: false,
                r#east: true,
                r#down: true,
                r#up: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 7626 {
            return Some(BrownMushroomBlock {
                r#up: true,
                r#west: false,
                r#north: false,
                r#down: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 7617 {
            return Some(BrownMushroomBlock {
                r#east: false,
                r#west: true,
                r#up: true,
                r#south: false,
                r#down: false,
                r#north: true,
            });
        }
        return None;
    }
}

