use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResinBrickWall {
    pub r#south: South,
    pub waterlogged: bool,
    pub r#east: East,
    pub up: bool,
    pub r#west: West,
    pub r#north: North,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    None,
    Low,
    Tall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum East {
    None,
    Low,
    Tall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum West {
    None,
    Low,
    Tall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum North {
    None,
    Low,
    Tall,
}

impl BlockState for ResinBrickWall {
    fn to_id(self) -> i32 {
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false { return 8915; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall { return 9099; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 8880; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 9005; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None { return 8815; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#west == West::Low { return 8902; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false { return 9035; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 8943; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall { return 9072; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 8859; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 8855; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true { return 8932; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 8939; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None { return 9029; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Low { return 8863; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false { return 9045; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 8998; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 9096; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None { return 8872; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None { return 8931; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 8914; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None { return 8925; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Tall { return 8940; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false { return 8962; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 8996; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None { return 9030; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 9031; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 9065; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false { return 8841; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 8979; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true { return 8809; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Low { return 8861; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 8907; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 8837; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false { return 8839; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 9015; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true { return 9074; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 9093; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 9126; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::None { return 8851; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 8946; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 9023; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None { return 8883; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true { return 9012; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 9046; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::None { return 8887; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true { return 8920; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == true { return 8847; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low { return 8901; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None { return 8889; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Low { return 8858; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 8816; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 8865; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::None { return 8923; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 9013; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false { return 9020; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall { return 9059; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Tall { return 9081; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low { return 9094; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 9051; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 8993; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false { return 9070; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Low { return 8898; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == true { return 8992; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None { return 8896; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None { return 8830; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 8971; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == true { return 9077; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true { return 8933; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low { return 9076; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Low { return 8961; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None { return 9067; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::None { return 8813; }
        if block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 8879; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::None { return 8904; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Tall { return 8927; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 8912; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Tall { return 9062; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall { return 9056; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 9018; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 8842; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None { return 8817; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall { return 8906; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 8834; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 9002; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true { return 9124; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 9129; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None { return 9028; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 9084; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 9055; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true { return 8836; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false { return 9091; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 8959; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false { return 8867; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 9097; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Low { return 8824; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::None { return 9043; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 9047; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 9086; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Tall { return 9120; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 9122; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == true { return 8808; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall { return 8882; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 8810; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low { return 8934; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 8975; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 8890; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 8980; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Tall { return 8982; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 8822; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall { return 9071; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == true { return 8825; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None { return 8831; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 8976; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Tall { return 8888; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::None { return 9038; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None { return 8832; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 9042; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low { return 8983; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Low { return 8866; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 8819; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 9089; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false { return 9103; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 9104; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 9078; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 9000; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 9090; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true { return 9037; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None { return 8921; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None { return 9048; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 8908; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false { return 8970; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true { return 9026; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 8972; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 8827; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low { return 8966; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false { return 9027; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 9044; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall { return 9039; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false { return 9022; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 8881; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Tall { return 8954; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall { return 8951; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low { return 8986; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low { return 8900; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 8938; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 9114; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::None { return 8835; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 9004; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false { return 8843; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 8903; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low { return 9061; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 9082; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 8947; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 9079; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 8977; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 8894; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 9088; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false { return 8949; }
        if block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall { return 9116; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 9064; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None { return 8911; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 8941; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 8997; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 9075; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 9085; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 8905; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false { return 9069; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 9100; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Tall { return 9058; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 8995; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == true { return 8868; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false { return 9107; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Low { return 9010; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 8973; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 9019; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 9083; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 8891; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true { return 8929; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 9121; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 8892; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::None { return 8919; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 8967; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 8893; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false { return 9111; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 8857; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true { return 8936; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true { return 9063; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true { return 9098; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == true { return 8852; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::None { return 9060; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 9130; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 8862; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 8871; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 8821; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 9118; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false { return 8884; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 9050; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == false { return 8960; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 9128; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 9108; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None { return 8922; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::None { return 8937; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 8952; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::None { return 9053; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None { return 8826; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 9008; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low { return 8958; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Low { return 8955; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 9115; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None { return 8848; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall { return 8877; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::Low { return 8956; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 8928; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false { return 9052; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None { return 9054; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 9057; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true { return 8957; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None { return 9041; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false { return 9009; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 9066; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 8988; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 9080; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == false { return 8811; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 8845; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Low { return 8864; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None { return 8909; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 8885; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 8860; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None { return 8953; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 8981; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 8853; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Tall { return 8985; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 8987; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None { return 8913; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall { return 8945; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 9011; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 9014; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 9117; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 9087; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 8930; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None { return 9123; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 8812; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 8978; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 8963; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::None { return 9025; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 8989; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 8874; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None { return 9032; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall { return 8873; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 9006; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Tall { return 8870; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::None { return 8924; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 9106; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true { return 8895; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low { return 8926; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true { return 8844; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall { return 8897; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 8869; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::None { return 8899; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true { return 9017; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low { return 8965; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 9034; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true { return 8875; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false { return 8818; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 9001; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None { return 9101; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 8990; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 9113; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::None { return 8850; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 8964; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 9127; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 9105; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 9131; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low { return 8984; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low { return 9016; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 8918; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 8935; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None { return 8910; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::None { return 8820; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == false { return 8999; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 9007; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 9073; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 8856; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 8942; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 9125; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true { return 8917; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 8829; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == true { return 8916; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 8969; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 9102; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall { return 9119; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 9095; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false { return 8886; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 9021; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None { return 9024; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 9049; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Low { return 9040; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None { return 8854; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::None { return 8944; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Low { return 8974; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false { return 8823; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall { return 8838; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None { return 8846; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Low { return 9003; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false { return 8878; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 9036; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None { return 8833; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None { return 8814; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::None { return 8991; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 9109; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low { return 8968; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 9110; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 8849; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Low { return 8948; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low { return 8876; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 9092; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::Low { return 8828; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 9068; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall { return 9112; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 8840; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::None { return 8994; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 8950; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 9033; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8915 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 9099 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 8880 {
            return Some(ResinBrickWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 9005 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 8815 {
            return Some(ResinBrickWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 8902 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 9035 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 8943 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 9072 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 8859 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8855 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 8932 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 8939 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 9029 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 8863 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 9045 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 8998 {
            return Some(ResinBrickWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 9096 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8872 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 8931 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 8914 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 8925 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 8940 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 8962 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 8996 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 9030 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 9031 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 9065 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 8841 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 8979 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 8809 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 8861 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8907 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 8837 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 8839 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 9015 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 9074 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 9093 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 9126 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 8851 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 8946 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 9023 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 8883 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 9012 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 9046 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 8887 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 8920 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 8847 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 8901 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 8889 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 8858 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 8816 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 8865 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 8923 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 9013 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 9020 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 9059 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9081 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 9094 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 9051 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 8993 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 9070 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8898 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 8992 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 8896 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 8830 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 8971 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 9077 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 8933 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 9076 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 8961 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 9067 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 8813 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 8879 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8904 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 8927 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 8912 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 9062 {
            return Some(ResinBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 9056 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 9018 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 8842 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 8817 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 8906 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 8834 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 9002 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 9124 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 9129 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 9028 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 9084 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 9055 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 8836 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 9091 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 8959 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 8867 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 9097 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 8824 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 9043 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 9047 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 9086 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 9120 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 9122 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 8808 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 8882 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 8810 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 8934 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 8975 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 8890 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 8980 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 8982 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 8822 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 9071 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 8825 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 8831 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 8976 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 8888 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 9038 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 8832 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 9042 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8983 {
            return Some(ResinBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 8866 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 8819 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9089 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 9103 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 9104 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 9078 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 9000 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 9090 {
            return Some(ResinBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9037 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 8921 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 9048 {
            return Some(ResinBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 8908 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 8970 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 9026 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 8972 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 8827 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8966 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 9027 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9044 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 9039 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 9022 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 8881 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 8954 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 8951 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 8986 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 8900 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 8938 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 9114 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 8835 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 9004 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8843 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 8903 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 9061 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 9082 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8947 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 9079 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 8977 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8894 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 9088 {
            return Some(ResinBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 8949 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9116 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 9064 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 8911 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 8941 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 8997 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 9075 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 9085 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 8905 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 9069 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9100 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 9058 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 8995 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 8868 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 9107 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9010 {
            return Some(ResinBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 8973 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 9019 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 9083 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 8891 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8929 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 9121 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 8892 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 8919 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 8967 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 8893 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 9111 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8857 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 8936 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 9063 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 9098 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 8852 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 9060 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 9130 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 8862 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 8871 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 8821 {
            return Some(ResinBrickWall {
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 9118 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 8884 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9050 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 8960 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 9128 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9108 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 8922 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 8937 {
            return Some(ResinBrickWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 8952 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 9053 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 8826 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 9008 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 8958 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8955 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 9115 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 8848 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 8877 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 8956 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 8928 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 9052 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9054 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 9057 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 8957 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 9041 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 9009 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 9066 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 8988 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9080 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 8811 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 8845 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8864 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 8909 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 8885 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 8860 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8953 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 8981 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8853 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 8985 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 8987 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8913 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 8945 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 9011 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9014 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 9117 {
            return Some(ResinBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 9087 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 8930 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 9123 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 8812 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 8978 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 8963 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 9025 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 8989 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 8874 {
            return Some(ResinBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 9032 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 8873 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 9006 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 8870 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 8924 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 9106 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 8895 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 8926 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 8844 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 8897 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 8869 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 8899 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 9017 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 8965 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 9034 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 8875 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 8818 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9001 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 9101 {
            return Some(ResinBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 8990 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 9113 {
            return Some(ResinBrickWall {
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 8850 {
            return Some(ResinBrickWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 8964 {
            return Some(ResinBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 9127 {
            return Some(ResinBrickWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 9105 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 9131 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 8984 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 9016 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 8918 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 8935 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 8910 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 8820 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 8999 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 9007 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 9073 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8856 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 8942 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 9125 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8917 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 8829 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 8916 {
            return Some(ResinBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 8969 {
            return Some(ResinBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 9102 {
            return Some(ResinBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9119 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 9095 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 8886 {
            return Some(ResinBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 9021 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 9024 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 9049 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 9040 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 8854 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 8944 {
            return Some(ResinBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 8974 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 8823 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8838 {
            return Some(ResinBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 8846 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 9003 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 8878 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 9036 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 8833 {
            return Some(ResinBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 8814 {
            return Some(ResinBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 8991 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 9109 {
            return Some(ResinBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 8968 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 9110 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 8849 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 8948 {
            return Some(ResinBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 8876 {
            return Some(ResinBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 9092 {
            return Some(ResinBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 8828 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 9068 {
            return Some(ResinBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 9112 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 8840 {
            return Some(ResinBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 8994 {
            return Some(ResinBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 8950 {
            return Some(ResinBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 9033 {
            return Some(ResinBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        return None;
    }
}

