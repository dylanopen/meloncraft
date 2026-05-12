use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CobblestoneWall {
    pub r#west: West,
    pub r#north: North,
    pub r#south: South,
    pub r#east: East,
    pub waterlogged: bool,
    pub up: bool,
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

impl BlockState for CobblestoneWall {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low { return 9933; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 9865; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true { return 9912; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 9906; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Low { return 9941; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false { return 9943; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 9992; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None { return 9804; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Low { return 9944; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false { return 9971; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 9897; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false { return 10088; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 9916; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 9955; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 9967; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 10009; }
        if block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true { return 10023; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 10050; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true { return 9782; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 9786; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 9805; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low { return 9954; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false { return 9802; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true { return 9940; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false { return 9789; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 10011; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 10012; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 10103; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 9986; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 10004; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Low { return 9889; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false { return 9788; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 9808; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None { return 9784; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 9976; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false { return 9787; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None { return 9846; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true { return 9888; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true { return 9985; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 9813; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None { return 9919; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Low { return 9826; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 9956; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 10007; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 9872; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 10016; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall { return 9953; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 9951; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Low { return 9909; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 9974; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 10030; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 9875; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 10033; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 10070; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 9950; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 9814; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None { return 9792; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::None { return 9837; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false { return 10075; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 10094; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false { return 9978; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 10095; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Low { return 9833; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 9832; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 10066; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 10056; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None { return 9800; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 9812; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::None { return 9903; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::None { return 9797; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 9983; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 9918; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true { return 9961; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 10054; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 10101; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::None { return 9866; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None { return 9861; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 10090; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 9839; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 10013; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None { return 9855; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 9920; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::Low { return 9892; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::None { return 10021; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == true { return 9780; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall { return 10047; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::None { return 9823; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::None { return 10014; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None { return 10029; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 9844; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 9796; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low { return 9869; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::None { return 9915; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 10040; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 10055; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 10078; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 10084; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 10099; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::None { return 9900; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 9987; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false { return 9850; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None { return 9791; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None { return 9891; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Low { return 9926; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 10096; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low { return 10017; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 10003; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false { return 10018; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 9979; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 9932; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false { return 9958; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == false { return 9982; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 9859; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 9980; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false { return 9910; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 9972; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 10024; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 10081; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None { return 9798; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false { return 9899; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 9817; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 10085; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None { return 9783; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low { return 9781; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None { return 9873; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false { return 9896; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall { return 9908; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 10006; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall { return 10020; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::None { return 9930; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 10010; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 10058; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true { return 9799; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None { return 9824; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 10097; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 9853; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Low { return 9905; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall { return 10037; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None { return 10053; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 9810; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 9825; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 9995; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::None { return 10001; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false { return 9848; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low { return 10039; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 10069; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 9952; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 10064; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Low { return 10065; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true { return 9925; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 10035; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::None { return 9936; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 9947; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 10059; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None { return 9981; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low { return 9849; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 9843; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false { return 10028; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 9937; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false { return 10077; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None { return 9858; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 9847; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 9860; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 9959; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall { return 9999; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low { return 9795; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == true { return 9965; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low { return 10051; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 9977; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 9931; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 9984; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 9819; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == true { return 9876; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 9975; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None { return 10032; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 10057; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::None { return 9831; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 9871; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false { return 10052; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None { return 9870; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 9921; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 9880; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::None { return 9840; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 10036; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false { return 10098; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 10089; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 9962; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 9809; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 9806; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None { return 9852; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false { return 9879; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None { return 10071; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 9818; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false { return 9836; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 10048; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 10061; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::None { return 9867; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == true { return 9924; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Low { return 9898; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == true { return 9829; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false { return 10015; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 9830; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 9989; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 10043; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Tall { return 9917; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 9945; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false { return 9838; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall { return 10082; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 10060; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Low { return 9939; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Low { return 9946; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false { return 9886; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 9887; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None { return 9964; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Low { return 9835; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 9841; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 10044; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 9801; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None { return 10002; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 10079; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 10093; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall { return 9877; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == true { return 9988; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 9969; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 10102; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 10100; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Low { return 9970; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::None { return 9890; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 9998; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 9851; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true { return 9794; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None { return 9807; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 9902; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 9966; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true { return 10008; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 10041; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 10091; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::Low { return 9929; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 10087; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall { return 10083; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true { return 9834; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::None { return 9793; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 10076; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 9815; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None { return 9862; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None { return 9883; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None { return 9923; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 9845; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low { return 9973; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == false { return 9895; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 9934; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == false { return 9991; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 9997; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::None { return 10019; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 9882; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == false { return 9790; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None { return 9878; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Low { return 10067; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 9821; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low { return 9948; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None { return 9927; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 9996; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None { return 9803; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 9881; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 9822; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall { return 9884; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 9811; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 9960; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 10074; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 9942; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None { return 10092; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Low { return 10063; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 9935; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None { return 9894; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Low { return 9868; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false { return 10031; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == true { return 9828; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false { return 9907; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 10025; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 10073; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::None { return 9901; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 10045; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall { return 10080; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true { return 9864; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 9993; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 9922; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 9857; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::Low { return 9904; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true { return 9928; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None { return 9963; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 9994; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true { return 10068; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::None { return 9827; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 9854; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 10026; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 10049; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None { return 9856; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None { return 9885; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#north == North::None { return 9893; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low { return 9957; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None { return 9990; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 10038; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 9911; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 9968; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 9938; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false { return 10086; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None { return 9785; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::None { return 10005; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true { return 10046; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 10072; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::None { return 9820; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 9913; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 10042; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 10062; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 9874; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true { return 10034; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 9949; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 9842; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None { return 9863; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 9914; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None { return 10000; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::None { return 9816; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true { return 10022; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false { return 10027; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9933 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 9865 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 9912 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 9906 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9941 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 9943 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 9992 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 9804 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 9944 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 9971 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 9897 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 10088 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 9916 {
            return Some(CobblestoneWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 9955 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 9967 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10009 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10023 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 10050 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9782 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 9786 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9805 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 9954 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 9802 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 9940 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 9789 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 10011 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 10012 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 10103 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 9986 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10004 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 9889 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 9788 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 9808 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 9784 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 9976 {
            return Some(CobblestoneWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 9787 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 9846 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 9888 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 9985 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 9813 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 9919 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 9826 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 9956 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10007 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 9872 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 10016 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9953 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 9951 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 9909 {
            return Some(CobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 9974 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 10030 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 9875 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10033 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 10070 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9950 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 9814 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 9792 {
            return Some(CobblestoneWall {
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 9837 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 10075 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 10094 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9978 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 10095 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 9833 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 9832 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10066 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 10056 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 9800 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 9812 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 9903 {
            return Some(CobblestoneWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 9797 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 9983 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 9918 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 9961 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 10054 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10101 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 9866 {
            return Some(CobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 9861 {
            return Some(CobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 10090 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 9839 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10013 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9855 {
            return Some(CobblestoneWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 9920 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 9892 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 10021 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 9780 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 10047 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 9823 {
            return Some(CobblestoneWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 10014 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 10029 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 9844 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 9796 {
            return Some(CobblestoneWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 9869 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 9915 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 10040 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10055 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 10078 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10084 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 10099 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 9900 {
            return Some(CobblestoneWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 9987 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 9850 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 9791 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 9891 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 9926 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 10096 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 10017 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 10003 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10018 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 9979 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 9932 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9958 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 9982 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 9859 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 9980 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 9910 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 9972 {
            return Some(CobblestoneWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10024 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 10081 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 9798 {
            return Some(CobblestoneWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 9899 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 9817 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 10085 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 9783 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 9781 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 9873 {
            return Some(CobblestoneWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 9896 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 9908 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 10006 {
            return Some(CobblestoneWall {
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10020 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 9930 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 10010 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10058 {
            return Some(CobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 9799 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9824 {
            return Some(CobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 10097 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 9853 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 9905 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 10037 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 10053 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 9810 {
            return Some(CobblestoneWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 9825 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 9995 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10001 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 9848 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 10039 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 10069 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 9952 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 10064 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 10065 {
            return Some(CobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 9925 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 10035 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 9936 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 9947 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 10059 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 9981 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 9849 {
            return Some(CobblestoneWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 9843 {
            return Some(CobblestoneWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 10028 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 9937 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 10077 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 9858 {
            return Some(CobblestoneWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 9847 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 9860 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9959 {
            return Some(CobblestoneWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9999 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 9795 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 9965 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 10051 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 9977 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 9931 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 9984 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9819 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 9876 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 9975 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 10032 {
            return Some(CobblestoneWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 10057 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 9831 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 9871 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10052 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 9870 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 9921 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 9880 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 9840 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 10036 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 10098 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 10089 {
            return Some(CobblestoneWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 9962 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9809 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 9806 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 9852 {
            return Some(CobblestoneWall {
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 9879 {
            return Some(CobblestoneWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10071 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 9818 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9836 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 10048 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10061 {
            return Some(CobblestoneWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 9867 {
            return Some(CobblestoneWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 9924 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 9898 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 9829 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10015 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 9830 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 9989 {
            return Some(CobblestoneWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 10043 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 9917 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 9945 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 9838 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 10082 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 10060 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 9939 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 9946 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 9886 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 9887 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 9964 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 9835 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 9841 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 10044 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 9801 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10002 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 10079 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 10093 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 9877 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 9988 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 9969 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10102 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 10100 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9970 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 9890 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 9998 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 9851 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 9794 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 9807 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 9902 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 9966 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10008 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 10041 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10091 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 9929 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 10087 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10083 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 9834 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9793 {
            return Some(CobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 10076 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 9815 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 9862 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 9883 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 9923 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 9845 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 9973 {
            return Some(CobblestoneWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 9895 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 9934 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 9991 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 9997 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10019 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 9882 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9790 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9878 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 10067 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 9821 {
            return Some(CobblestoneWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 9948 {
            return Some(CobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 9927 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 9996 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 9803 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 9881 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 9822 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9884 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9811 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 9960 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 10074 {
            return Some(CobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 9942 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10092 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 10063 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 9935 {
            return Some(CobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 9894 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 9868 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 10031 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 9828 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 9907 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 10025 {
            return Some(CobblestoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 10073 {
            return Some(CobblestoneWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9901 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 10045 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10080 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 9864 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 9993 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 9922 {
            return Some(CobblestoneWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 9857 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 9904 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 9928 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 9963 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 9994 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10068 {
            return Some(CobblestoneWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 9827 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 9854 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 10026 {
            return Some(CobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 10049 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9856 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 9885 {
            return Some(CobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 9893 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 9957 {
            return Some(CobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 9990 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 10038 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 9911 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 9968 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 9938 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10086 {
            return Some(CobblestoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 9785 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 10005 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 10046 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 10072 {
            return Some(CobblestoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 9820 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 9913 {
            return Some(CobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10042 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10062 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 9874 {
            return Some(CobblestoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10034 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 9949 {
            return Some(CobblestoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 9842 {
            return Some(CobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 9863 {
            return Some(CobblestoneWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 9914 {
            return Some(CobblestoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10000 {
            return Some(CobblestoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 9816 {
            return Some(CobblestoneWall {
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 10022 {
            return Some(CobblestoneWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10027 {
            return Some(CobblestoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        return None;
    }
}

