use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedstoneWire {
    pub r#north: North,
    pub r#south: South,
    pub power: i32,
    pub r#east: East,
    pub r#west: West,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum North {
    Up,
    Side,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    Up,
    Side,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum East {
    Up,
    Side,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum West {
    Up,
    Side,
    None,
}

impl BlockState for RedstoneWire {
    fn to_id(self) -> i32 {
        if block_state.r#power == 10 && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#west == West::Side { return 4333; }
        if block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#power == 11 { return 4198; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#power == 10 { return 4338; }
        if block_state.r#power == 1 && block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#west == West::Up { return 3966; }
        if block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#power == 8 && block_state.r#east == East::None { return 4898; }
        if block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#power == 6 { return 3866; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 12 && block_state.r#east == East::Side && block_state.r#north == North::Up { return 4356; }
        if block_state.r#power == 9 && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#west == West::Side { return 4762; }
        if block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 5 && block_state.r#south == South::Side && block_state.r#east == East::Side { return 4434; }
        if block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#power == 3 && block_state.r#south == South::None && block_state.r#east == East::Up { return 4131; }
        if block_state.r#north == North::None && block_state.r#power == 8 && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None { return 5042; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 15 && block_state.r#south == South::Up { return 4667; }
        if block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#power == 9 && block_state.r#south == South::Up { return 3892; }
        if block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#power == 13 && block_state.r#south == South::Side && block_state.r#west == West::Side { return 3931; }
        if block_state.r#east == East::Up && block_state.r#power == 9 && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#north == North::Side { return 4037; }
        if block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#power == 10 && block_state.r#north == North::Side { return 4908; }
        if block_state.r#west == West::Side && block_state.r#power == 7 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::None { return 4024; }
        if block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#power == 14 && block_state.r#east == East::Up { return 4084; }
        if block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#power == 5 && block_state.r#south == South::Up { return 3856; }
        if block_state.r#north == North::Side && block_state.r#power == 5 && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#east == East::Up { return 4005; }
        if block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#power == 6 { return 4159; }
        if block_state.r#north == North::None && block_state.r#power == 2 && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#west == West::Side { return 4555; }
        if block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 14 { return 4371; }
        if block_state.r#power == 15 && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::Up { return 4384; }
        if block_state.r#north == North::None && block_state.r#power == 2 && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#south == South::None { return 4124; }
        if block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 10 && block_state.r#north == North::Up { return 3904; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 7 && block_state.r#south == South::Side && block_state.r#west == West::None { return 4598; }
        if block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#power == 3 && block_state.r#east == East::Side { return 4564; }
        if block_state.r#power == 0 && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4242; }
        if block_state.r#power == 2 && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4408; }
        if block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#power == 11 { return 3916; }
        if block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#power == 2 && block_state.r#east == East::None && block_state.r#south == South::None { return 4987; }
        if block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 7 { return 3879; }
        if block_state.r#power == 12 && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#east == East::Up { return 4207; }
        if block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#power == 14 && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4801; }
        if block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#power == 9 { return 4899; }
        if block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 3 && block_state.r#south == South::Side { return 3986; }
        if block_state.r#west == West::Up && block_state.r#south == South::Side && block_state.r#power == 1 && block_state.r#north == North::Side && block_state.r#east == East::None { return 4830; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#power == 11 && block_state.r#east == East::Up { return 4200; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#power == 6 { return 4304; }
        if block_state.r#east == East::Side && block_state.r#power == 2 && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::Side { return 4264; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 7 && block_state.r#west == West::Up && block_state.r#south == South::Up { return 4305; }
        if block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#power == 13 && block_state.r#south == South::Side { return 4651; }
        if block_state.r#east == East::Up && block_state.r#power == 15 && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::None { return 4091; }
        if block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 5 && block_state.r#north == North::Up { return 4723; }
        if block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#power == 0 { return 4098; }
        if block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#power == 0 { return 4963; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 14 { return 5094; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#power == 2 { return 4406; }
        if block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 4 { return 3849; }
        if block_state.r#power == 11 && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Up { return 5061; }
        if block_state.r#north == North::None && block_state.r#power == 15 && block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#south == South::Up { return 5098; }
        if block_state.r#west == West::Up && block_state.r#power == 14 && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Up { return 4512; }
        if block_state.r#power == 12 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Side { return 5074; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 6 && block_state.r#north == North::Side { return 4873; }
        if block_state.r#east == East::None && block_state.r#power == 12 && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#south == South::None { return 5077; }
        if block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 3 { return 4845; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#power == 14 && block_state.r#south == South::None { return 4231; }
        if block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#power == 4 && block_state.r#south == South::None && block_state.r#west == West::Side { return 4285; }
        if block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#power == 2 { return 3833; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#west == West::Side && block_state.r#south == South::Side { return 4111; }
        if block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#power == 5 { return 4439; }
        if block_state.r#north == North::None && block_state.r#power == 6 && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#west == West::None { return 4154; }
        if block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 14 { return 3936; }
        if block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#power == 11 && block_state.r#west == West::None && block_state.r#east == East::Up { return 3911; }
        if block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#power == 6 { return 4734; }
        if block_state.r#north == North::None && block_state.r#power == 11 && block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#south == South::Up { return 5062; }
        if block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#power == 1 && block_state.r#east == East::Side && block_state.r#south == South::Side { return 4256; }
        if block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#power == 15 && block_state.r#west == West::None { return 4235; }
        if block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 6 { return 4879; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 15 && block_state.r#south == South::Side && block_state.r#west == West::None { return 4382; }
        if block_state.r#power == 11 && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::Up { return 4779; }
        if block_state.r#north == North::None && block_state.r#power == 0 && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4537; }
        if block_state.r#north == North::None && block_state.r#power == 10 && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#south == South::Side { return 4624; }
        if block_state.r#east == East::None && block_state.r#power == 2 && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4981; }
        if block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 7 && block_state.r#west == West::None && block_state.r#south == South::None { return 4025; }
        if block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#power == 15 && block_state.r#east == East::Up { return 4238; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#power == 10 && block_state.r#west == West::Side { return 5059; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#power == 13 && block_state.r#west == West::Up { return 3927; }
        if block_state.r#power == 13 && block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#west == West::Side { return 4075; }
        if block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#power == 14 && block_state.r#east == East::Up && block_state.r#south == South::Side { return 4083; }
        if block_state.r#east == East::Side && block_state.r#power == 2 && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#south == South::None { return 4411; }
        if block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#west == West::Up && block_state.r#east == East::None { return 4974; }
        if block_state.r#north == North::Side && block_state.r#power == 13 && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4504; }
        if block_state.r#west == West::Up && block_state.r#power == 9 && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#east == East::Side { return 4614; }
        if block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 7 && block_state.r#south == South::Side { return 4022; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#power == 8 { return 4178; }
        if block_state.r#north == North::None && block_state.r#power == 10 && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4623; }
        if block_state.r#power == 13 && block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#north == North::None { return 4220; }
        if block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#power == 3 && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4702; }
        if block_state.r#power == 15 && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#south == South::Up { return 4233; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 4 && block_state.r#south == South::None && block_state.r#west == West::None { return 4862; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 5 && block_state.r#north == North::Side && block_state.r#east == East::None { return 4864; }
        if block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#west == West::None { return 4547; }
        if block_state.r#south == South::Up && block_state.r#power == 2 && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#east == East::Up { return 3973; }
        if block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#power == 6 { return 4731; }
        if block_state.r#north == North::Up && block_state.r#power == 5 && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#east == East::Up { return 3855; }
        if block_state.r#power == 11 && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::None { return 4774; }
        if block_state.r#south == South::None && block_state.r#power == 5 && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#east == East::Up { return 4007; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 6 && block_state.r#north == North::Side { return 4880; }
        if block_state.r#power == 9 && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#west == West::Side { return 4330; }
        if block_state.r#power == 0 && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#north == North::None { return 4530; }
        if block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#power == 0 && block_state.r#west == West::None { return 4676; }
        if block_state.r#west == West::None && block_state.r#power == 9 && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Side { return 4907; }
        if block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 9 { return 4905; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 3 && block_state.r#south == South::Side && block_state.r#north == North::None { return 4994; }
        if block_state.r#power == 13 && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#north == North::Up { return 4362; }
        if block_state.r#power == 4 && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::None { return 4571; }
        if block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 12 && block_state.r#north == North::None && block_state.r#west == West::Up { return 5073; }
        if block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#power == 7 { return 3880; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#power == 2 && block_state.r#west == West::Up { return 4260; }
        if block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#power == 3 { return 4273; }
        if block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 15 && block_state.r#west == West::None && block_state.r#north == North::Side { return 4958; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#power == 0 && block_state.r#west == West::Side && block_state.r#north == North::Up { return 4675; }
        if block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#power == 6 && block_state.r#north == North::Up && block_state.r#east == East::Up { return 3867; }
        if block_state.r#power == 8 && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#south == South::Side { return 4749; }
        if block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#power == 4 && block_state.r#west == West::Up { return 4854; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#west == West::Side { return 5071; }
        if block_state.r#power == 5 && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#south == South::None { return 4150; }
        if block_state.r#north == North::Side && block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#east == East::Up { return 4062; }
        if block_state.r#power == 12 && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#south == South::Side { return 4930; }
        if block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#power == 5 { return 5008; }
        if block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#power == 2 && block_state.r#west == West::None && block_state.r#north == North::None { return 4553; }
        if block_state.r#west == West::Side && block_state.r#south == South::Side && block_state.r#power == 1 && block_state.r#north == North::Up && block_state.r#east == East::None { return 4687; }
        if block_state.r#power == 14 && block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4368; }
        if block_state.r#west == West::None && block_state.r#power == 6 && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#south == South::Up { return 4730; }
        if block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#power == 14 { return 4945; }
        if block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#power == 4 && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4713; }
        if block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#power == 12 && block_state.r#west == West::Side { return 3922; }
        if block_state.r#power == 14 && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::Up { return 3938; }
        if block_state.r#north == North::None && block_state.r#power == 4 && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Side { return 5003; }
        if block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#power == 7 && block_state.r#east == East::Side { return 4594; }
        if block_state.r#power == 5 && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#north == North::Side { return 4865; }
        if block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#power == 2 { return 4267; }
        if block_state.r#south == South::None && block_state.r#power == 13 && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#east == East::None { return 5085; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#power == 8 && block_state.r#south == South::Up { return 4316; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 10 { return 4196; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#power == 15 && block_state.r#east == East::Side && block_state.r#west == West::None { return 4379; }
        if block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 7 && block_state.r#south == South::Side { return 4166; }
        if block_state.r#power == 15 && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4380; }
        if block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::None { return 4496; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#power == 6 { return 4585; }
        if block_state.r#power == 5 && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::Side { return 4868; }
        if block_state.r#power == 0 && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#east == East::Side { return 4390; }
        if block_state.r#north == North::Up && block_state.r#power == 13 && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#south == South::None { return 3935; }
        if block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 3 && block_state.r#west == West::Up { return 3984; }
        if block_state.r#east == East::Side && block_state.r#power == 5 && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#north == North::None { return 4575; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 5 && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4432; }
        if block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 7 && block_state.r#south == South::Side { return 4742; }
        if block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#south == South::Side && block_state.r#power == 1 && block_state.r#north == North::Up { return 4686; }
        if block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#power == 11 { return 4631; }
        if block_state.r#power == 7 && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#south == South::None { return 4743; }
        if block_state.r#power == 1 && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::None { return 4110; }
        if block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#power == 12 && block_state.r#east == East::Up { return 3921; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#power == 13 && block_state.r#north == North::Up && block_state.r#south == South::Up { return 4361; }
        if block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 10 && block_state.r#west == West::None { return 4913; }
        if block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 11 { return 5068; }
        if block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#power == 9 && block_state.r#north == North::Up && block_state.r#west == West::Up { return 3894; }
        if block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#power == 7 && block_state.r#west == West::None && block_state.r#east == East::Up { return 3881; }
        if block_state.r#power == 11 && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#west == West::Up { return 4053; }
        if block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 7 && block_state.r#south == South::Side && block_state.r#north == North::Up { return 3876; }
        if block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 8 && block_state.r#south == South::Up { return 4027; }
        if block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#power == 0 && block_state.r#south == South::Up { return 4100; }
        if block_state.r#east == East::Up && block_state.r#power == 4 && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#north == North::None { return 4135; }
        if block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#power == 13 && block_state.r#west == West::Up { return 4215; }
        if block_state.r#power == 14 && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#north == North::None { return 4225; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#power == 11 && block_state.r#west == West::None && block_state.r#east == East::Side { return 4343; }
        if block_state.r#south == South::None && block_state.r#power == 1 && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#north == North::Up { return 4257; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#power == 1 { return 4255; }
        if block_state.r#power == 5 && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Up { return 5013; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 9 { return 5043; }
        if block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#power == 3 { return 3842; }
        if block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#power == 2 && block_state.r#south == South::None && block_state.r#west == West::Up { return 4266; }
        if block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 11 { return 4920; }
        if block_state.r#west == West::Side && block_state.r#power == 13 && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#east == East::None { return 4795; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 11 && block_state.r#north == North::Up && block_state.r#west == West::None { return 4781; }
        if block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#power == 13 && block_state.r#east == East::Up { return 4076; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#power == 15 { return 4377; }
        if block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 6 && block_state.r#south == South::None && block_state.r#west == West::Up { return 4158; }
        if block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#power == 7 && block_state.r#south == South::Side && block_state.r#north == North::None { return 4597; }
        if block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 14 && block_state.r#north == North::None { return 5093; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#power == 3 && block_state.r#east == East::Side && block_state.r#west == West::None { return 4565; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 3 && block_state.r#east == East::None && block_state.r#north == North::None { return 4995; }
        if block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#power == 15 { return 3950; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 2 { return 4556; }
        if block_state.r#north == North::Up && block_state.r#power == 13 && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::None { return 4799; }
        if block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 14 { return 5089; }
        if block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 3 && block_state.r#west == West::None { return 4709; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 4 { return 4567; }
        if block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#power == 6 { return 4729; }
        if block_state.r#north == North::Side && block_state.r#power == 15 && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#west == West::None { return 4529; }
        if block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#power == 8 && block_state.r#west == West::None && block_state.r#east == East::Side { return 4463; }
        if block_state.r#east == East::Side && block_state.r#power == 15 && block_state.r#west == West::Up && block_state.r#south == South::Side && block_state.r#north == North::Side { return 4524; }
        if block_state.r#east == East::Side && block_state.r#power == 3 && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Up { return 4559; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#power == 4 { return 4998; }
        if block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 2 && block_state.r#west == West::Side && block_state.r#north == North::Side { return 4840; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 15 && block_state.r#north == North::Side && block_state.r#east == East::None { return 4959; }
        if block_state.r#power == 0 && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::Up { return 4386; }
        if block_state.r#power == 8 && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#west == West::Up { return 4170; }
        if block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#power == 14 && block_state.r#north == North::None { return 4658; }
        if block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 15 && block_state.r#north == North::None { return 4669; }
        if block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#power == 2 && block_state.r#north == North::Side { return 4836; }
        if block_state.r#east == East::Side && block_state.r#power == 14 && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::None { return 4664; }
        if block_state.r#south == South::Side && block_state.r#power == 10 && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#west == West::Side { return 4912; }
        if block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#power == 11 && block_state.r#east == East::Up && block_state.r#west == West::Side { return 4201; }
        if block_state.r#north == North::Up && block_state.r#power == 15 && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#south == South::None { return 4383; }
        if block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 1 && block_state.r#west == West::Side { return 4540; }
        if block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#power == 12 && block_state.r#west == West::Up { return 3918; }
        if block_state.r#power == 3 && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Up { return 4563; }
        if block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 2 && block_state.r#east == East::Up && block_state.r#north == North::Side { return 3972; }
        if block_state.r#north == North::Up && block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#west == West::None { return 4259; }
        if block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#power == 14 && block_state.r#west == West::None && block_state.r#north == North::Side { return 4520; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#power == 15 && block_state.r#south == South::Up && block_state.r#east == East::None { return 5099; }
        if block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 0 && block_state.r#west == West::None { return 4391; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 3 && block_state.r#south == South::Up { return 4413; }
        if block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#power == 13 && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4363; }
        if block_state.r#north == North::Up && block_state.r#power == 4 && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#south == South::None { return 3852; }
        if block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#power == 11 && block_state.r#north == North::Side { return 4061; }
        if block_state.r#east == East::Up && block_state.r#power == 0 && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#south == South::Up { return 3812; }
        if block_state.r#power == 1 && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::Side { return 4834; }
        if block_state.r#west == West::None && block_state.r#power == 3 && block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#north == North::None { return 4130; }
        if block_state.r#west == West::None && block_state.r#power == 9 && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::None { return 4187; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#power == 2 && block_state.r#west == West::Side { return 3835; }
        if block_state.r#west == West::None && block_state.r#power == 6 && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Up { return 4160; }
        if block_state.r#north == North::Side && block_state.r#power == 9 && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#south == South::Side { return 4902; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#power == 15 && block_state.r#north == North::None { return 4672; }
        if block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 12 && block_state.r#north == North::Up { return 3924; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#power == 15 && block_state.r#west == West::Up && block_state.r#east == East::None { return 4956; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 1 && block_state.r#north == North::None { return 4976; }
        if block_state.r#south == South::Up && block_state.r#power == 3 && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#north == North::None { return 4989; }
        if block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#power == 5 { return 4295; }
        if block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 5 { return 4869; }
        if block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#power == 7 { return 4739; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 15 { return 4095; }
        if block_state.r#west == West::Up && block_state.r#power == 2 && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#east == East::None { return 4983; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 12 && block_state.r#north == North::Up { return 4783; }
        if block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#power == 3 { return 3843; }
        if block_state.r#power == 10 && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::None { return 4769; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#power == 4 && block_state.r#north == North::Up { return 4280; }
        if block_state.r#north == North::Up && block_state.r#power == 1 && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4252; }
        if block_state.r#south == South::Up && block_state.r#power == 12 && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::None { return 4640; }
        if block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#power == 4 { return 4279; }
        if block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#power == 5 { return 3862; }
        if block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#power == 7 { return 5028; }
        if block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#power == 1 && block_state.r#west == West::Up && block_state.r#north == North::Up { return 3822; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#power == 3 { return 3845; }
        if block_state.r#power == 14 && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#south == South::Side { return 5091; }
        if block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#power == 12 && block_state.r#south == South::Up { return 4782; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 3 && block_state.r#south == South::None { return 4420; }
        if block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#power == 9 && block_state.r#west == West::Side { return 4474; }
        if block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 5 && block_state.r#south == South::None { return 4870; }
        if block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#power == 5 && block_state.r#south == South::Side { return 4722; }
        if block_state.r#power == 3 && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#north == North::Side { return 3987; }
        if block_state.r#power == 7 && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#north == North::Up { return 4308; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#power == 8 { return 4604; }
        if block_state.r#east == East::Up && block_state.r#power == 13 && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#south == South::Side { return 3930; }
        if block_state.r#east == East::Side && block_state.r#power == 11 && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4633; }
        if block_state.r#power == 3 && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#south == South::Up { return 4701; }
        if block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#power == 4 && block_state.r#east == East::None { return 4856; }
        if block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#power == 7 && block_state.r#south == South::Up { return 4019; }
        if block_state.r#power == 11 && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::None { return 4204; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 3 && block_state.r#north == North::Up { return 4269; }
        if block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#power == 12 && block_state.r#north == North::Side { return 4928; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 8 { return 4174; }
        if block_state.r#power == 5 && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::None { return 4292; }
        if block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#power == 9 { return 4757; }
        if block_state.r#power == 14 && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::Up { return 4082; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#west == West::Side { return 4258; }
        if block_state.r#east == East::None && block_state.r#power == 2 && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::None { return 4697; }
        if block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 5 && block_state.r#west == West::Up && block_state.r#south == South::Up { return 4143; }
        if block_state.r#north == North::Up && block_state.r#power == 4 && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#east == East::Side { return 4283; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#power == 13 && block_state.r#east == East::Side && block_state.r#west == West::Side { return 4510; }
        if block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#power == 14 { return 4657; }
        if block_state.r#power == 13 && block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#south == South::Side { return 5082; }
        if block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 10 && block_state.r#west == West::Side && block_state.r#south == South::Up { return 4621; }
        if block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 10 { return 4481; }
        if block_state.r#west == West::None && block_state.r#power == 1 && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Side { return 4400; }
        if block_state.r#east == East::Side && block_state.r#power == 12 && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#north == North::Side { return 4501; }
        if block_state.r#power == 13 && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::None { return 4511; }
        if block_state.r#power == 15 && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#south == South::None { return 4239; }
        if block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#power == 2 && block_state.r#south == South::Up && block_state.r#east == East::None { return 4694; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 8 && block_state.r#south == South::Side { return 4893; }
        if block_state.r#power == 3 && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4849; }
        if block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#power == 13 && block_state.r#north == North::Up && block_state.r#south == South::Up { return 3928; }
        if block_state.r#east == East::Up && block_state.r#power == 9 && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::Side { return 4040; }
        if block_state.r#power == 15 && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::None { return 4961; }
        if block_state.r#south == South::None && block_state.r#power == 11 && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#north == North::Up { return 4780; }
        if block_state.r#power == 1 && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::None { return 3821; }
        if block_state.r#west == West::None && block_state.r#power == 15 && block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#east == East::Up { return 4094; }
        if block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 2 { return 4984; }
        if block_state.r#north == North::Side && block_state.r#power == 8 && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#west == West::None { return 4466; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 8 && block_state.r#west == West::Up && block_state.r#south == South::Up { return 4458; }
        if block_state.r#power == 5 && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Side { return 4003; }
        if block_state.r#north == North::Side && block_state.r#power == 4 && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#south == South::Side { return 3993; }
        if block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#power == 3 { return 4704; }
        if block_state.r#power == 3 && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::Side { return 4993; }
        if block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 4 { return 5004; }
        if block_state.r#east == East::Up && block_state.r#power == 9 && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Side { return 4186; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#power == 13 && block_state.r#north == North::Side && block_state.r#east == East::Up { return 4079; }
        if block_state.r#power == 4 && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#south == South::Side { return 4139; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#power == 0 { return 4536; }
        if block_state.r#north == North::Up && block_state.r#power == 15 && block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#south == South::Up { return 4810; }
        if block_state.r#power == 4 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::None { return 3995; }
        if block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#power == 13 && block_state.r#east == East::Up { return 4216; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 15 { return 3949; }
        if block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#power == 0 && block_state.r#west == West::Side && block_state.r#north == North::Up { return 4243; }
        if block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#power == 8 && block_state.r#north == North::Up { return 4319; }
        if block_state.r#power == 13 && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Side { return 5086; }
        if block_state.r#north == North::None && block_state.r#power == 15 && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::None { return 5101; }
        if block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#power == 3 && block_state.r#north == North::Up && block_state.r#east == East::Up { return 3844; }
        if block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#power == 4 { return 3854; }
        if block_state.r#west == West::Up && block_state.r#power == 0 && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#south == South::Side { return 4101; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#power == 5 && block_state.r#east == East::Up && block_state.r#south == South::Side { return 4148; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 0 { return 4394; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 10 { return 4766; }
        if block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#power == 8 && block_state.r#south == South::None { return 4752; }
        if block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 3 { return 4417; }
        if block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#power == 7 { return 4310; }
        if block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 10 { return 4768; }
        if block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 15 && block_state.r#east == East::None && block_state.r#south == South::None { return 4815; }
        if block_state.r#power == 3 && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4846; }
        if block_state.r#power == 0 && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#west == West::Up { return 4392; }
        if block_state.r#east == East::Up && block_state.r#power == 14 && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#south == South::Up { return 4224; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 5 { return 4863; }
        if block_state.r#west == West::None && block_state.r#power == 9 && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#east == East::None { return 4901; }
        if block_state.r#west == West::Up && block_state.r#power == 13 && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#north == North::Side { return 4938; }
        if block_state.r#east == East::Up && block_state.r#power == 1 && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#north == North::None { return 4108; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#power == 6 && block_state.r#south == South::Side { return 4156; }
        if block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#power == 8 && block_state.r#north == North::None { return 4608; }
        if block_state.r#power == 12 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#west == West::None { return 4931; }
        if block_state.r#east == East::Side && block_state.r#power == 6 && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::Side { return 4588; }
        if block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#power == 6 { return 4589; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 12 && block_state.r#east == East::None && block_state.r#north == North::Up { return 4788; }
        if block_state.r#east == East::None && block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::Side { return 4927; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#power == 13 { return 4653; }
        if block_state.r#power == 4 && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Side { return 4573; }
        if block_state.r#south == South::Side && block_state.r#power == 3 && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#west == West::None { return 4706; }
        if block_state.r#power == 9 && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4900; }
        if block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 0 && block_state.r#north == North::None && block_state.r#south == South::None { return 4969; }
        if block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#power == 4 && block_state.r#south == South::Up { return 4712; }
        if block_state.r#north == North::Side && block_state.r#power == 12 && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4500; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 14 && block_state.r#south == South::Side { return 4660; }
        if block_state.r#north == North::Up && block_state.r#power == 5 && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#south == South::Side { return 4291; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#power == 6 && block_state.r#east == East::None && block_state.r#west == West::Up { return 5019; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#power == 14 && block_state.r#south == South::Side && block_state.r#west == West::Side { return 3940; }
        if block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#power == 5 { return 4435; }
        if block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#power == 2 && block_state.r#east == East::Side && block_state.r#south == South::None { return 4412; }
        if block_state.r#east == East::Up && block_state.r#power == 12 && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4210; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#power == 4 && block_state.r#east == East::Side { return 4425; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#power == 7 { return 4453; }
        if block_state.r#power == 13 && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::Side { return 4650; }
        if block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 6 && block_state.r#west == West::None && block_state.r#north == North::None { return 5021; }
        if block_state.r#power == 7 && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#south == South::None { return 5031; }
        if block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#power == 0 && block_state.r#west == West::Up && block_state.r#south == South::Up { return 3810; }
        if block_state.r#south == South::Up && block_state.r#power == 4 && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#east == East::Up { return 3990; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#power == 13 && block_state.r#east == East::Side && block_state.r#west == West::Up { return 4359; }
        if block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#power == 9 && block_state.r#west == West::None { return 4619; }
        if block_state.r#power == 8 && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#west == West::None { return 4322; }
        if block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#power == 6 { return 5017; }
        if block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#power == 2 { return 4265; }
        if block_state.r#power == 9 && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#north == North::None { return 4617; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#power == 4 && block_state.r#east == East::Side && block_state.r#north == North::None { return 4574; }
        if block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#power == 13 && block_state.r#north == North::None && block_state.r#south == South::Side { return 4652; }
        if block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 0 && block_state.r#south == South::Side { return 4821; }
        if block_state.r#south == South::Side && block_state.r#power == 9 && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::Side { return 4471; }
        if block_state.r#power == 3 && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#east == East::None { return 4991; }
        if block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 9 { return 4041; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 11 { return 4489; }
        if block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#power == 12 { return 4358; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#power == 7 { return 4018; }
        if block_state.r#power == 5 && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#west == West::Up { return 4287; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#power == 10 { return 3902; }
        if block_state.r#north == North::Side && block_state.r#power == 9 && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#south == South::Up { return 4036; }
        if block_state.r#east == East::Side && block_state.r#power == 10 && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4332; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#power == 2 { return 4693; }
        if block_state.r#east == East::None && block_state.r#power == 2 && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#south == South::Side { return 4695; }
        if block_state.r#power == 15 && block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#north == North::Up { return 4816; }
        if block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#power == 1 && block_state.r#south == South::Up { return 4828; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#power == 3 { return 4415; }
        if block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 6 && block_state.r#west == West::None { return 4877; }
        if block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 11 { return 4924; }
        if block_state.r#north == North::None && block_state.r#power == 12 && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#west == West::Up { return 4641; }
        if block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 7 && block_state.r#east == East::None { return 4881; }
        if block_state.r#power == 2 && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#north == North::None { return 4985; }
        if block_state.r#east == East::None && block_state.r#power == 3 && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None { return 4997; }
        if block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 8 { return 5035; }
        if block_state.r#power == 13 && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#south == South::Up { return 4072; }
        if block_state.r#north == North::Side && block_state.r#power == 14 && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#south == South::None { return 4519; }
        if block_state.r#south == South::Up && block_state.r#power == 2 && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::Side { return 4405; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 13 && block_state.r#east == East::None { return 5083; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 14 && block_state.r#south == South::None && block_state.r#west == West::Up { return 4230; }
        if block_state.r#power == 11 && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Up { return 5063; }
        if block_state.r#west == West::Side && block_state.r#power == 6 && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#south == South::Side { return 4732; }
        if block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#power == 4 && block_state.r#north == North::Side && block_state.r#south == South::Side { return 4857; }
        if block_state.r#power == 7 && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None { return 4745; }
        if block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#power == 14 && block_state.r#south == South::Side && block_state.r#east == East::None { return 4805; }
        if block_state.r#north == North::Side && block_state.r#power == 13 && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Side { return 4942; }
        if block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 10 && block_state.r#west == West::None { return 4334; }
        if block_state.r#power == 10 && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#west == West::Up { return 3900; }
        if block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#power == 2 { return 3977; }
        if block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#power == 8 && block_state.r#east == East::None && block_state.r#north == North::Side { return 4892; }
        if block_state.r#north == North::Side && block_state.r#power == 6 && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::Side { return 4015; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 8 && block_state.r#south == South::None && block_state.r#west == West::Up { return 4464; }
        if block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#power == 13 { return 4798; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#power == 2 { return 4986; }
        if block_state.r#south == South::Up && block_state.r#power == 10 && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::None { return 4765; }
        if block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 4 && block_state.r#north == North::None { return 5005; }
        if block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#power == 0 { return 4105; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 14 && block_state.r#south == South::Up && block_state.r#north == North::Side { return 4946; }
        if block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#power == 7 { return 4312; }
        if block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#power == 9 { return 4468; }
        if block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#power == 11 && block_state.r#west == West::Side { return 4057; }
        if block_state.r#north == North::Side && block_state.r#power == 3 && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::None { return 4418; }
        if block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#south == South::Side && block_state.r#power == 8 && block_state.r#east == East::None { return 4750; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 15 && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4953; }
        if block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#power == 2 && block_state.r#west == West::Up { return 4692; }
        if block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#power == 7 && block_state.r#south == South::Up && block_state.r#west == West::Side { return 3874; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#power == 5 && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4579; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#power == 0 { return 4535; }
        if block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#power == 13 && block_state.r#east == East::None && block_state.r#north == North::Up { return 4793; }
        if block_state.r#power == 12 && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Up { return 5072; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#power == 15 && block_state.r#south == South::Side { return 5100; }
        if block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#power == 9 && block_state.r#west == West::None { return 5045; }
        if block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 12 && block_state.r#west == West::Side { return 4933; }
        if block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 7 && block_state.r#south == South::Side { return 5029; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#power == 14 { return 4661; }
        if block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#power == 6 && block_state.r#east == East::None && block_state.r#north == North::None { return 5016; }
        if block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 5 { return 5007; }
        if block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 3 { return 4561; }
        if block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#power == 4 { return 4718; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 3 && block_state.r#north == North::Side { return 4853; }
        if block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#power == 4 { return 3850; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#power == 8 && block_state.r#east == East::Up && block_state.r#west == West::None { return 4034; }
        if block_state.r#power == 8 && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::None { return 3887; }
        if block_state.r#east == East::None && block_state.r#power == 0 && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Up { return 4968; }
        if block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 15 && block_state.r#south == South::None { return 4241; }
        if block_state.r#power == 9 && block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#south == South::None { return 3899; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 12 && block_state.r#south == South::None && block_state.r#west == West::Side { return 4357; }
        if block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#power == 1 && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4398; }
        if block_state.r#power == 10 && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#south == South::Up { return 4764; }
        if block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#power == 10 && block_state.r#west == West::Side { return 3907; }
        if block_state.r#power == 2 && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#south == South::Up { return 4404; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 6 && block_state.r#south == South::Up { return 4872; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#power == 7 && block_state.r#south == South::Up { return 5026; }
        if block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#power == 15 && block_state.r#east == East::None { return 5104; }
        if block_state.r#north == North::None && block_state.r#power == 0 && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::None { return 4532; }
        if block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#power == 8 && block_state.r#south == South::Side && block_state.r#east == East::Side { return 4605; }
        if block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#power == 4 && block_state.r#north == North::None { return 4142; }
        if block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#west == West::Up { return 3969; }
        if block_state.r#power == 3 && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4416; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 10 && block_state.r#south == South::None && block_state.r#west == West::Up { return 4626; }
        if block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#power == 5 && block_state.r#east == East::Up && block_state.r#west == West::Up { return 4002; }
        if block_state.r#east == East::Up && block_state.r#power == 0 && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::None { return 4103; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 11 && block_state.r#south == South::Up { return 4199; }
        if block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#power == 1 && block_state.r#east == East::Side { return 4253; }
        if block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#power == 3 && block_state.r#north == North::Up { return 3839; }
        if block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#power == 13 && block_state.r#north == North::Up && block_state.r#south == South::Side { return 3932; }
        if block_state.r#east == East::Up && block_state.r#power == 10 && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::None { return 4193; }
        if block_state.r#north == North::Up && block_state.r#power == 7 && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Side { return 4744; }
        if block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#power == 6 { return 4155; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#power == 6 && block_state.r#west == West::Side { return 5020; }
        if block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#power == 13 && block_state.r#east == East::Up && block_state.r#west == West::None { return 4217; }
        if block_state.r#north == North::None && block_state.r#power == 4 && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4569; }
        if block_state.r#north == North::Up && block_state.r#power == 9 && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#south == South::Side { return 4758; }
        if block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#power == 2 && block_state.r#south == South::Up && block_state.r#east == East::Up { return 3830; }
        if block_state.r#north == North::Up && block_state.r#power == 15 && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#south == South::None { return 3951; }
        if block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 9 && block_state.r#south == South::None { return 4761; }
        if block_state.r#south == South::Side && block_state.r#power == 9 && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#east == East::Side { return 4327; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 1 && block_state.r#south == South::Side && block_state.r#west == West::None { return 3968; }
        if block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#power == 4 && block_state.r#south == South::Up && block_state.r#north == North::Up { return 4278; }
        if block_state.r#power == 6 && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::Up { return 4298; }
        if block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#north == North::Side { return 4402; }
        if block_state.r#power == 5 && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::None { return 4583; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#power == 11 && block_state.r#north == North::Side && block_state.r#west == West::Up { return 4923; }
        if block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#power == 0 { return 3957; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#power == 5 { return 4721; }
        if block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#power == 6 && block_state.r#east == East::None && block_state.r#south == South::Up { return 4728; }
        if block_state.r#west == West::Up && block_state.r#power == 8 && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#north == North::Side { return 4032; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#power == 8 { return 4029; }
        if block_state.r#power == 4 && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4423; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 5 { return 4867; }
        if block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 5 && block_state.r#south == South::Side { return 3859; }
        if block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#power == 12 && block_state.r#south == South::None { return 4790; }
        if block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#power == 15 && block_state.r#south == South::Up && block_state.r#north == North::Side { return 4089; }
        if block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 2 && block_state.r#south == South::Up { return 3974; }
        if block_state.r#south == South::Up && block_state.r#power == 6 && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#north == North::Up { return 4296; }
        if block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#power == 4 && block_state.r#south == South::Up && block_state.r#north == North::Side { return 4424; }
        if block_state.r#power == 11 && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#east == East::None { return 4777; }
        if block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 3 { return 3840; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 3 && block_state.r#north == North::None { return 4126; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#power == 12 && block_state.r#north == North::Side { return 4502; }
        if block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#power == 10 && block_state.r#east == East::None && block_state.r#west == West::Side { return 4909; }
        if block_state.r#east == East::None && block_state.r#power == 4 && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::Up { return 4716; }
        if block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 2 { return 4980; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 11 && block_state.r#east == East::Up { return 3910; }
        if block_state.r#power == 2 && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::Up { return 3980; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#power == 2 && block_state.r#west == West::Side { return 4261; }
        if block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#power == 2 && block_state.r#south == South::Side && block_state.r#east == East::Side { return 4407; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#power == 1 && block_state.r#west == West::Up { return 4539; }
        if block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#power == 11 && block_state.r#south == South::Up { return 4917; }
        if block_state.r#power == 12 && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::None { return 4934; }
        if block_state.r#north == North::Side && block_state.r#power == 14 && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::None { return 4088; }
        if block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 11 { return 4922; }
        if block_state.r#power == 10 && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#east == East::Side { return 4335; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#west == West::None { return 3827; }
        if block_state.r#power == 5 && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#south == South::Side { return 4436; }
        if block_state.r#power == 10 && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::None { return 4340; }
        if block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#power == 15 && block_state.r#west == West::Side { return 4522; }
        if block_state.r#power == 6 && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::None { return 5018; }
        if block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#south == South::Side && block_state.r#power == 15 && block_state.r#north == North::Side { return 4525; }
        if block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#power == 10 && block_state.r#east == East::Up { return 4190; }
        if block_state.r#power == 3 && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#south == South::Up { return 3838; }
        if block_state.r#power == 5 && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#west == West::Up { return 4293; }
        if block_state.r#north == North::None && block_state.r#power == 4 && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4572; }
        if block_state.r#power == 11 && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#south == South::Side { return 3913; }
        if block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 4 && block_state.r#north == North::Up && block_state.r#south == South::Up { return 3847; }
        if block_state.r#west == West::Side && block_state.r#power == 1 && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#south == South::Side { return 4399; }
        if block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#power == 3 { return 4851; }
        if block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#power == 13 && block_state.r#west == West::Side && block_state.r#north == North::Side { return 4507; }
        if block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#power == 12 && block_state.r#south == South::Up { return 3919; }
        if block_state.r#south == South::Side && block_state.r#power == 10 && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::None { return 4625; }
        if block_state.r#power == 7 && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#north == North::Up { return 4741; }
        if block_state.r#west == West::Up && block_state.r#power == 14 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::None { return 4086; }
        if block_state.r#east == East::Up && block_state.r#power == 10 && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4188; }
        if block_state.r#west == West::Side && block_state.r#power == 0 && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#east == East::Side { return 4249; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#power == 9 { return 4472; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 0 { return 4680; }
        if block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 7 && block_state.r#north == North::None && block_state.r#east == East::Up { return 4168; }
        if block_state.r#west == West::Up && block_state.r#power == 9 && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#east == East::Up { return 3897; }
        if block_state.r#west == West::None && block_state.r#power == 12 && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#east == East::Up { return 3923; }
        if block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#power == 8 && block_state.r#north == North::Up && block_state.r#west == West::Side { return 4318; }
        if block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#power == 14 && block_state.r#east == East::Up && block_state.r#west == West::Side { return 4087; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#power == 15 && block_state.r#east == East::Side { return 4385; }
        if block_state.r#east == East::Side && block_state.r#power == 8 && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::Up { return 4460; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#power == 3 && block_state.r#south == South::Side && block_state.r#north == North::None { return 4562; }
        if block_state.r#power == 10 && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#south == South::Up { return 4622; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#power == 1 { return 4978; }
        if block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#power == 8 && block_state.r#west == West::None { return 4028; }
        if block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 1 { return 3967; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 13 { return 4648; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 5 { return 4288; }
        if block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#power == 1 && block_state.r#east == East::Up && block_state.r#west == West::Side { return 3970; }
        if block_state.r#west == West::Up && block_state.r#power == 14 && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#south == South::Side { return 3939; }
        if block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 8 && block_state.r#south == South::None { return 4176; }
        if block_state.r#power == 5 && block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4437; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#power == 1 { return 4972; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#power == 15 && block_state.r#south == South::Up { return 4090; }
        if block_state.r#power == 2 && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#north == North::Side { return 4838; }
        if block_state.r#power == 1 && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None { return 4691; }
        if block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#power == 5 && block_state.r#east == East::None && block_state.r#south == South::None { return 5014; }
        if block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 7 && block_state.r#north == North::None { return 5030; }
        if block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 4 { return 4141; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#power == 8 && block_state.r#south == South::Up { return 3882; }
        if block_state.r#east == East::None && block_state.r#power == 9 && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::None { return 5051; }
        if block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 10 && block_state.r#west == West::Side { return 4189; }
        if block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#power == 5 && block_state.r#south == South::None { return 4726; }
        if block_state.r#power == 14 && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::None { return 4232; }
        if block_state.r#east == East::None && block_state.r#power == 7 && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::Side { return 4882; }
        if block_state.r#south == South::Side && block_state.r#power == 10 && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::Up { return 4337; }
        if block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#power == 12 && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4498; }
        if block_state.r#power == 0 && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::None { return 4962; }
        if block_state.r#west == West::None && block_state.r#power == 9 && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::Up { return 4613; }
        if block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#power == 15 && block_state.r#east == East::Up && block_state.r#west == West::None { return 3953; }
        if block_state.r#power == 4 && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#west == West::Up { return 4284; }
        if block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 12 && block_state.r#north == North::Up && block_state.r#south == South::Side { return 4786; }
        if block_state.r#east == East::Up && block_state.r#power == 6 && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#north == North::Up { return 3865; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#west == West::None { return 4403; }
        if block_state.r#power == 15 && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Side { return 3946; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 13 && block_state.r#south == South::None { return 4509; }
        if block_state.r#power == 15 && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#north == North::Side { return 4096; }
        if block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#power == 3 && block_state.r#east == East::Up { return 4125; }
        if block_state.r#east == East::Side && block_state.r#power == 3 && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::None { return 4274; }
        if block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#power == 6 { return 4441; }
        if block_state.r#power == 0 && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::None { return 4533; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 3 && block_state.r#south == South::Up { return 4558; }
        if block_state.r#south == South::Up && block_state.r#power == 11 && block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#north == North::None { return 4630; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#power == 8 && block_state.r#north == North::Up && block_state.r#west == West::Up { return 4746; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 15 && block_state.r#south == South::Side { return 4670; }
        if block_state.r#west == West::None && block_state.r#power == 7 && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::Side { return 4886; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#power == 14 && block_state.r#east == East::None { return 5096; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 5 && block_state.r#south == South::Up { return 3999; }
        if block_state.r#south == South::Side && block_state.r#power == 9 && block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#north == North::None { return 4615; }
        if block_state.r#power == 8 && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::Side { return 5038; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#power == 13 { return 4654; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#power == 13 && block_state.r#north == North::None && block_state.r#south == South::None { return 5087; }
        if block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#power == 4 { return 4281; }
        if block_state.r#south == South::Up && block_state.r#power == 3 && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#west == West::None { return 4703; }
        if block_state.r#west == West::None && block_state.r#power == 6 && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#east == East::Up { return 4157; }
        if block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 7 && block_state.r#south == South::None && block_state.r#east == East::Up { return 4023; }
        if block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 9 && block_state.r#south == South::Up { return 4180; }
        if block_state.r#power == 5 && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4290; }
        if block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#power == 6 && block_state.r#south == South::Side && block_state.r#west == West::None { return 4301; }
        if block_state.r#west == West::Up && block_state.r#power == 11 && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#south == South::Side { return 4776; }
        if block_state.r#south == South::Side && block_state.r#power == 0 && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None { return 4967; }
        if block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 9 && block_state.r#south == South::Side && block_state.r#north == North::Up { return 3895; }
        if block_state.r#power == 10 && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::Side { return 4192; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#power == 1 { return 4543; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#power == 0 && block_state.r#east == East::Up { return 3811; }
        if block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#power == 7 && block_state.r#south == South::Up && block_state.r#north == North::None { return 4593; }
        if block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#power == 5 && block_state.r#west == West::Up { return 4578; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#power == 7 && block_state.r#east == East::Side { return 4601; }
        if block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#power == 6 && block_state.r#west == West::Up { return 4440; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#power == 1 { return 4544; }
        if block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#power == 4 { return 3992; }
        if block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 4 { return 4859; }
        if block_state.r#power == 11 && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#west == West::Up { return 4773; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#power == 15 && block_state.r#east == East::Up && block_state.r#west == West::None { return 3947; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#power == 6 && block_state.r#west == West::Up { return 4446; }
        if block_state.r#west == West::Up && block_state.r#power == 13 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::None { return 4077; }
        if block_state.r#power == 13 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#south == South::None { return 4078; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 11 { return 4925; }
        if block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#power == 11 { return 4347; }
        if block_state.r#west == West::Side && block_state.r#power == 8 && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::Side { return 4459; }
        if block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#power == 10 && block_state.r#south == South::None { return 4770; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#power == 4 { return 4715; }
        if block_state.r#west == West::None && block_state.r#power == 0 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Side { return 3959; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#power == 8 && block_state.r#north == North::None { return 5040; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#power == 10 { return 5054; }
        if block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 13 && block_state.r#south == South::Side { return 4939; }
        if block_state.r#north == North::None && block_state.r#power == 9 && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#east == East::Side { return 4618; }
        if block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#power == 9 && block_state.r#south == South::None { return 4331; }
        if block_state.r#south == South::Up && block_state.r#power == 15 && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#west == West::None { return 4811; }
        if block_state.r#west == West::None && block_state.r#power == 4 && block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#north == North::Up { return 3851; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#power == 12 && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4645; }
        if block_state.r#power == 6 && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Side { return 4448; }
        if block_state.r#power == 14 && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::Up { return 3944; }
        if block_state.r#west == West::Side && block_state.r#power == 2 && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#south == South::Side { return 3976; }
        if block_state.r#north == North::Side && block_state.r#power == 1 && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::None { return 4829; }
        if block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#power == 15 { return 3945; }
        if block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#power == 10 { return 4478; }
        if block_state.r#east == East::None && block_state.r#power == 7 && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::Side { return 4888; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 5 && block_state.r#south == South::Up && block_state.r#west == West::None { return 4289; }
        if block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#power == 11 { return 4634; }
        if block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 13 { return 4941; }
        if block_state.r#power == 3 && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#east == East::Up { return 4128; }
        if block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#power == 10 && block_state.r#north == North::Side { return 4482; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#power == 6 && block_state.r#south == South::None && block_state.r#west == West::Side { return 5023; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#power == 0 { return 3813; }
        if block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#power == 8 { return 3884; }
        if block_state.r#south == South::Up && block_state.r#power == 2 && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::None { return 4117; }
        if block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 9 { return 3898; }
        if block_state.r#power == 5 && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#east == East::Up { return 4144; }
        if block_state.r#west == West::None && block_state.r#power == 5 && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#south == South::Side { return 3860; }
        if block_state.r#power == 1 && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4401; }
        if block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 15 && block_state.r#west == West::Up { return 4668; }
        if block_state.r#power == 15 && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#north == North::Up { return 4813; }
        if block_state.r#west == West::Up && block_state.r#power == 1 && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#south == South::None { return 4689; }
        if block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#power == 14 { return 4518; }
        if block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#power == 12 { return 4069; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4494; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#power == 12 && block_state.r#north == North::None { return 4643; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 5 { return 4719; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#power == 11 && block_state.r#west == West::None { return 4493; }
        if block_state.r#north == North::Side && block_state.r#power == 11 && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Up { return 4919; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#power == 9 && block_state.r#west == West::Side { return 4039; }
        if block_state.r#power == 1 && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4546; }
        if block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 15 && block_state.r#south == South::None { return 4097; }
        if block_state.r#east == East::Up && block_state.r#power == 14 && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::None { return 4085; }
        if block_state.r#east == East::Up && block_state.r#power == 0 && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Up { return 4104; }
        if block_state.r#power == 4 && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#north == North::None { return 4138; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#power == 9 && block_state.r#north == North::Side && block_state.r#south == South::Up { return 4469; }
        if block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#power == 14 && block_state.r#south == South::Up && block_state.r#west == West::None { return 4802; }
        if block_state.r#west == West::Side && block_state.r#power == 3 && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#south == South::Side { return 4705; }
        if block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#power == 6 { return 4009; }
        if block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#power == 8 { return 4026; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 0 && block_state.r#west == West::Up { return 4824; }
        if block_state.r#power == 13 && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4218; }
        if block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 10 && block_state.r#south == South::Side { return 5055; }
        if block_state.r#power == 12 && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4354; }
        if block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 7 && block_state.r#east == East::Side { return 4449; }
        if block_state.r#south == South::None && block_state.r#power == 7 && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#north == North::Up { return 4311; }
        if block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#power == 9 { return 5048; }
        if block_state.r#north == North::None && block_state.r#power == 12 && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#south == South::Up { return 4638; }
        if block_state.r#power == 10 && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::None { return 4910; }
        if block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 4 { return 4137; }
        if block_state.r#south == South::None && block_state.r#power == 15 && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#west == West::Up { return 4671; }
        if block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#power == 0 { return 3818; }
        if block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#power == 8 && block_state.r#south == South::Side && block_state.r#north == North::None { return 5037; }
        if block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#power == 9 && block_state.r#west == West::Up && block_state.r#east == East::Up { return 4179; }
        if block_state.r#north == North::Up && block_state.r#power == 9 && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#east == East::None { return 4760; }
        if block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4107; }
        if block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#power == 12 && block_state.r#west == West::Side && block_state.r#south == South::Up { return 4495; }
        if block_state.r#south == South::None && block_state.r#power == 14 && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#north == North::Up { return 4807; }
        if block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#power == 7 && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4306; }
        if block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#power == 3 && block_state.r#east == East::None { return 4707; }
        if block_state.r#east == East::Side && block_state.r#power == 9 && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4611; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#power == 0 && block_state.r#south == South::Up { return 4964; }
        if block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#power == 4 && block_state.r#west == West::Side { return 3991; }
        if block_state.r#north == North::Side && block_state.r#power == 14 && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#south == South::Up { return 4081; }
        if block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#power == 5 && block_state.r#east == East::Up { return 4149; }
        if block_state.r#west == West::None && block_state.r#power == 9 && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::Up { return 4181; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#power == 7 { return 4450; }
        if block_state.r#power == 9 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Side { return 5047; }
        if block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 6 && block_state.r#west == West::Side && block_state.r#south == South::Side { return 4012; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#power == 14 { return 4375; }
        if block_state.r#power == 13 && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::Up { return 5079; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#power == 4 { return 4860; }
        if block_state.r#power == 6 && block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#south == South::None { return 4303; }
        if block_state.r#power == 8 && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#north == North::None { return 4172; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#power == 12 && block_state.r#west == West::Up { return 4497; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#power == 7 { return 5025; }
        if block_state.r#power == 10 && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::Side { return 4051; }
        if block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#power == 6 { return 3872; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 0 { return 3960; }
        if block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 7 { return 4600; }
        if block_state.r#power == 3 && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#east == East::Up { return 3985; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#power == 9 && block_state.r#north == North::Up { return 4763; }
        if block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#power == 10 { return 4052; }
        if block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#power == 12 && block_state.r#north == North::None { return 4644; }
        if block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 10 && block_state.r#south == South::Side { return 4048; }
        if block_state.r#power == 12 && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::Side { return 4499; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 13 && block_state.r#south == South::Up { return 4649; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 4 { return 3997; }
        if block_state.r#power == 8 && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#north == North::None { return 4175; }
        if block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#power == 11 && block_state.r#east == East::None && block_state.r#west == West::Side { return 4921; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#power == 0 && block_state.r#south == South::None { return 4970; }
        if block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#power == 15 && block_state.r#north == North::None && block_state.r#west == West::None { return 5102; }
        if block_state.r#east == East::None && block_state.r#power == 6 && block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#west == West::Up { return 4875; }
        if block_state.r#power == 15 && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#south == South::None { return 4240; }
        if block_state.r#power == 0 && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::Up { return 3817; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 13 { return 4797; }
        if block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#power == 6 { return 3864; }
        if block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#power == 7 && block_state.r#east == East::None { return 4889; }
        if block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#power == 9 && block_state.r#west == West::None { return 4328; }
        if block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 4 && block_state.r#north == North::Up && block_state.r#east == East::None { return 4717; }
        if block_state.r#north == North::Up && block_state.r#power == 14 && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#south == South::Up { return 4800; }
        if block_state.r#north == North::None && block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#east == East::None { return 5070; }
        if block_state.r#east == East::Up && block_state.r#power == 14 && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::Up { return 3943; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 15 && block_state.r#south == South::Up { return 4523; }
        if block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#power == 7 && block_state.r#east == East::Up { return 4161; }
        if block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#power == 2 && block_state.r#south == South::None && block_state.r#north == North::Up { return 3836; }
        if block_state.r#west == West::Side && block_state.r#power == 12 && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#north == North::Up { return 3925; }
        if block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#power == 14 && block_state.r#south == South::None { return 3942; }
        if block_state.r#power == 1 && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::Up { return 4251; }
        if block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#power == 3 && block_state.r#north == North::Side { return 4852; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 11 && block_state.r#south == South::None && block_state.r#west == West::None { return 5069; }
        if block_state.r#power == 2 && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#west == West::Up { return 4554; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 14 { return 4376; }
        if block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 13 && block_state.r#south == South::Up && block_state.r#east == East::Side { return 4503; }
        if block_state.r#south == South::Side && block_state.r#power == 10 && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4336; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#power == 13 && block_state.r#west == West::Side { return 4936; }
        if block_state.r#power == 14 && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::None { return 5090; }
        if block_state.r#west == West::Side && block_state.r#power == 9 && block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#north == North::None { return 4183; }
        if block_state.r#west == West::None && block_state.r#power == 15 && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None { return 5105; }
        if block_state.r#north == North::Side && block_state.r#power == 3 && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#east == East::Up { return 3981; }
        if block_state.r#power == 13 && block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4360; }
        if block_state.r#power == 8 && block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#south == South::Side { return 4606; }
        if block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#power == 2 && block_state.r#east == East::Up && block_state.r#west == West::None { return 4118; }
        if block_state.r#west == West::Side && block_state.r#power == 2 && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#north == North::Up { return 4696; }
        if block_state.r#power == 3 && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::None { return 4847; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 13 && block_state.r#south == South::Up { return 4935; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 9 && block_state.r#north == North::None && block_state.r#east == East::Side { return 4612; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#power == 14 && block_state.r#west == West::Side { return 5095; }
        if block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#power == 3 && block_state.r#west == West::Side && block_state.r#south == South::None { return 4708; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 10 { return 4047; }
        if block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#west == West::None { return 4208; }
        if block_state.r#power == 5 && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Side { return 4871; }
        if block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 7 { return 5032; }
        if block_state.r#power == 1 && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#south == South::Up { return 4827; }
        if block_state.r#south == South::Up && block_state.r#power == 4 && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#east == East::Up { return 3848; }
        if block_state.r#power == 9 && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::Up { return 4182; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#power == 12 && block_state.r#east == East::Up && block_state.r#north == North::None { return 4214; }
        if block_state.r#north == North::Up && block_state.r#power == 6 && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::None { return 4736; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 0 && block_state.r#south == South::None { return 4826; }
        if block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#power == 2 && block_state.r#east == East::Up && block_state.r#south == South::None { return 3979; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 6 && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4443; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#power == 5 && block_state.r#north == North::None && block_state.r#west == West::None { return 5009; }
        if block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 4 { return 4136; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 3 && block_state.r#east == East::Side && block_state.r#north == North::Up { return 4270; }
        if block_state.r#power == 13 && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#north == North::Side { return 4937; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#power == 0 && block_state.r#west == West::None { return 4538; }
        if block_state.r#west == West::Side && block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#power == 0 && block_state.r#north == North::Side { return 3958; }
        if block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#power == 6 && block_state.r#west == West::None { return 3869; }
        if block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 12 { return 4206; }
        if block_state.r#power == 8 && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::Up { return 4753; }
        if block_state.r#power == 2 && block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#north == North::Side { return 4409; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 3 && block_state.r#west == West::None && block_state.r#south == South::Up { return 4127; }
        if block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#power == 12 && block_state.r#east == East::Up && block_state.r#south == South::None { return 4068; }
        if block_state.r#east == East::None && block_state.r#power == 0 && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::None { return 4965; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#power == 15 && block_state.r#east == East::Side { return 4673; }
        if block_state.r#power == 3 && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#north == North::Up { return 3841; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#power == 14 && block_state.r#north == North::Up { return 4374; }
        if block_state.r#north == North::Up && block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::None { return 4784; }
        if block_state.r#north == North::Up && block_state.r#power == 14 && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::None { return 4804; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#power == 8 && block_state.r#north == North::None && block_state.r#west == West::None { return 5036; }
        if block_state.r#power == 7 && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#south == South::None { return 4313; }
        if block_state.r#power == 9 && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#north == North::Side { return 4043; }
        if block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#west == West::None { return 3920; }
        if block_state.r#east == East::Up && block_state.r#power == 12 && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::None { return 4211; }
        if block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#power == 6 && block_state.r#west == West::None && block_state.r#east == East::Up { return 4010; }
        if block_state.r#north == North::None && block_state.r#power == 14 && block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#south == South::Up { return 4226; }
        if block_state.r#east == East::Side && block_state.r#power == 14 && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#south == South::None { return 4663; }
        if block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#power == 6 { return 4584; }
        if block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#power == 14 { return 4656; }
        if block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#power == 13 && block_state.r#east == East::None && block_state.r#north == North::Up { return 4796; }
        if block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 14 { return 4808; }
        if block_state.r#east == East::None && block_state.r#power == 2 && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#south == South::None { return 4843; }
        if block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#power == 9 && block_state.r#north == North::None { return 5044; }
        if block_state.r#power == 4 && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#east == East::None { return 4999; }
        if block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 4 { return 4711; }
        if block_state.r#north == North::None && block_state.r#power == 0 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::Side { return 4966; }
        if block_state.r#east == East::None && block_state.r#power == 5 && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::Up { return 4725; }
        if block_state.r#power == 7 && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#south == South::Up { return 3873; }
        if block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#power == 11 && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4491; }
        if block_state.r#power == 3 && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#west == West::None { return 4277; }
        if block_state.r#east == East::None && block_state.r#power == 1 && block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#west == West::Side { return 4831; }
        if block_state.r#power == 2 && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#north == North::Side { return 4410; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#power == 9 { return 4473; }
        if block_state.r#west == West::None && block_state.r#power == 5 && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#north == North::Side { return 4433; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#power == 11 { return 4487; }
        if block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#east == East::Side && block_state.r#west == West::None { return 4541; }
        if block_state.r#power == 13 && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#west == West::Up { return 4647; }
        if block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 2 { return 4841; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 12 && block_state.r#south == South::Up && block_state.r#west == West::None { return 4064; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 6 { return 4591; }
        if block_state.r#north == North::Side && block_state.r#power == 14 && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#east == East::None { return 4944; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#power == 5 { return 3861; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#power == 8 && block_state.r#west == West::Side { return 4897; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#power == 9 { return 4184; }
        if block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#power == 14 && block_state.r#east == East::Up { return 4229; }
        if block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#power == 9 && block_state.r#north == North::Up && block_state.r#west == West::Up { return 4326; }
        if block_state.r#power == 2 && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#north == North::None { return 4119; }
        if block_state.r#south == South::None && block_state.r#power == 3 && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#west == West::Up { return 4275; }
        if block_state.r#east == East::Up && block_state.r#power == 7 && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Up { return 4163; }
        if block_state.r#power == 6 && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#west == West::None { return 4592; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#power == 14 && block_state.r#east == East::Side { return 4659; }
        if block_state.r#south == South::Side && block_state.r#power == 8 && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#west == West::Up { return 4461; }
        if block_state.r#power == 2 && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Side { return 4699; }
        if block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#power == 11 && block_state.r#east == East::Up { return 4054; }
        if block_state.r#power == 13 && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#north == North::Up { return 3934; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#power == 10 { return 4339; }
        if block_state.r#east == East::None && block_state.r#power == 12 && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::Up { return 4932; }
        if block_state.r#power == 5 && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#south == South::Side { return 4724; }
        if block_state.r#power == 13 && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#west == West::Up { return 3933; }
        if block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#power == 8 { return 4173; }
        if block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 8 && block_state.r#south == South::Side { return 4894; }
        if block_state.r#power == 11 && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#north == North::Side { return 4488; }
        if block_state.r#east == East::Side && block_state.r#power == 14 && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::None { return 4373; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 0 && block_state.r#east == East::Side && block_state.r#north == North::None { return 4531; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 13 && block_state.r#south == South::Side && block_state.r#west == West::None { return 4364; }
        if block_state.r#south == South::Side && block_state.r#power == 15 && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#east == East::Up { return 4092; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#power == 2 && block_state.r#east == East::Up { return 4121; }
        if block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 1 && block_state.r#east == East::Side && block_state.r#north == North::Side { return 4395; }
        if block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#power == 5 && block_state.r#east == East::Side && block_state.r#north == North::Up { return 4294; }
        if block_state.r#power == 6 && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Side { return 4874; }
        if block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#power == 4 { return 4566; }
        if block_state.r#power == 12 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::None { return 4070; }
        if block_state.r#west == West::Up && block_state.r#power == 1 && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::None { return 4113; }
        if block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#power == 2 { return 4837; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#power == 5 && block_state.r#north == North::Side && block_state.r#south == South::None { return 4438; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#power == 0 && block_state.r#north == North::None && block_state.r#south == South::Side { return 4534; }
        if block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#south == South::Side && block_state.r#power == 4 { return 4426; }
        if block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#power == 14 { return 3937; }
        if block_state.r#west == West::Up && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 2 { return 4551; }
        if block_state.r#power == 14 && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::None { return 4952; }
        if block_state.r#west == West::Side && block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 7 { return 4021; }
        if block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 12 && block_state.r#east == East::None && block_state.r#north == North::Up { return 4789; }
        if block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Up { return 4973; }
        if block_state.r#power == 10 && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#south == South::Side { return 3903; }
        if block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 12 { return 4063; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 4 { return 3853; }
        if block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#power == 13 && block_state.r#north == North::Side { return 4508; }
        if block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#power == 15 && block_state.r#east == East::None && block_state.r#north == North::Up { return 4814; }
        if block_state.r#west == West::None && block_state.r#power == 5 && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#north == North::None { return 5012; }
        if block_state.r#power == 9 && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#north == North::None { return 5050; }
        if block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 0 && block_state.r#west == West::Side { return 3955; }
        if block_state.r#north == North::None && block_state.r#power == 14 && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4662; }
        if block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#power == 7 { return 4454; }
        if block_state.r#power == 15 && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Up { return 3948; }
        if block_state.r#power == 12 && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#west == West::None { return 4787; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#power == 5 { return 5015; }
        if block_state.r#power == 10 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4044; }
        if block_state.r#power == 11 && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#east == East::Up { return 3914; }
        if block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#power == 12 && block_state.r#west == West::Up { return 4065; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#south == South::Side && block_state.r#power == 11 { return 3912; }
        if block_state.r#east == East::Side && block_state.r#power == 10 && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::None { return 4628; }
        if block_state.r#power == 15 && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#east == East::Up { return 4236; }
        if block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 9 && block_state.r#south == South::Side { return 4038; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 1 { return 4835; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 2 && block_state.r#west == West::None { return 4988; }
        if block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#power == 2 && block_state.r#south == South::Side { return 3832; }
        if block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#power == 11 && block_state.r#east == East::None { return 4918; }
        if block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#power == 6 { return 4153; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#power == 10 { return 5058; }
        if block_state.r#power == 11 && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::Side { return 4056; }
        if block_state.r#west == West::Side && block_state.r#power == 2 && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#south == South::None { return 4123; }
        if block_state.r#power == 11 && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#east == East::Side { return 4341; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#power == 1 { return 4396; }
        if block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#power == 8 { return 4603; }
        if block_state.r#power == 2 && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#south == South::None { return 4698; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#power == 7 { return 4887; }
        if block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 15 && block_state.r#west == West::Side && block_state.r#south == South::Side { return 4957; }
        if block_state.r#north == North::Up && block_state.r#power == 7 && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#east == East::Side { return 4307; }
        if block_state.r#east == East::Up && block_state.r#power == 10 && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Up { return 3908; }
        if block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 4 && block_state.r#north == North::None && block_state.r#south == South::None { return 4140; }
        if block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 13 { return 4222; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#power == 5 && block_state.r#east == East::Side { return 4581; }
        if block_state.r#east == East::Up && block_state.r#power == 0 && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#north == North::None { return 4099; }
        if block_state.r#power == 13 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::Up { return 4794; }
        if block_state.r#east == East::Up && block_state.r#power == 5 && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#north == North::Side { return 4001; }
        if block_state.r#east == East::Up && block_state.r#power == 8 && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::None { return 4031; }
        if block_state.r#power == 0 && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::Side { return 4246; }
        if block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#power == 15 { return 5097; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#power == 10 && block_state.r#west == West::Side { return 3901; }
        if block_state.r#power == 15 && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#south == South::Up { return 4665; }
        if block_state.r#north == North::Up && block_state.r#power == 2 && block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#south == South::Up { return 4262; }
        if block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#power == 8 && block_state.r#north == North::None && block_state.r#west == West::None { return 4607; }
        if block_state.r#west == West::Up && block_state.r#power == 13 && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#south == South::Up { return 4071; }
        if block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#power == 11 { return 4490; }
        if block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#power == 9 { return 4035; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#power == 0 && block_state.r#west == West::None && block_state.r#north == North::Up { return 4682; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 4 { return 5002; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#power == 9 { return 4325; }
        if block_state.r#power == 1 && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::None { return 3965; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#power == 12 && block_state.r#south == South::None && block_state.r#west == West::None { return 3926; }
        if block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 10 && block_state.r#south == South::None && block_state.r#west == West::Side { return 4627; }
        if block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 9 { return 4616; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#power == 13 && block_state.r#east == East::Up && block_state.r#west == West::None { return 4223; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 7 && block_state.r#south == South::Up && block_state.r#west == West::None { return 4883; }
        if block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#power == 9 && block_state.r#east == East::Up { return 4185; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#power == 7 { return 4596; }
        if block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#south == South::Side && block_state.r#power == 14 && block_state.r#north == North::None { return 4227; }
        if block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 7 && block_state.r#south == South::None && block_state.r#west == West::Up { return 4599; }
        if block_state.r#power == 5 && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4146; }
        if block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#power == 15 && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4381; }
        if block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 10 { return 4911; }
        if block_state.r#power == 13 && block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#south == South::Side { return 4506; }
        if block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#power == 8 { return 4315; }
        if block_state.r#south == South::None && block_state.r#power == 11 && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4348; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#power == 4 { return 4422; }
        if block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#power == 14 && block_state.r#east == East::None { return 5092; }
        if block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#power == 6 { return 4445; }
        if block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#power == 11 { return 4344; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 2 && block_state.r#south == South::None && block_state.r#west == West::Up { return 3978; }
        if block_state.r#north == North::None && block_state.r#power == 5 && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4582; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#power == 7 { return 4595; }
        if block_state.r#power == 14 && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#west == West::Up { return 4080; }
        if block_state.r#power == 8 && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::Side { return 4751; }
        if block_state.r#power == 11 && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#west == West::Side { return 4636; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#power == 1 && block_state.r#west == West::Up { return 4833; }
        if block_state.r#power == 13 && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#north == North::Up { return 4367; }
        if block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#power == 6 && block_state.r#north == North::Up && block_state.r#west == West::Side { return 4300; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 0 && block_state.r#north == North::Side && block_state.r#west == West::Side { return 4825; }
        if block_state.r#power == 3 && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#north == North::None { return 4992; }
        if block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#power == 15 && block_state.r#south == South::Up && block_state.r#east == East::Up { return 4234; }
        if block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#power == 3 && block_state.r#east == East::Up && block_state.r#west == West::Side { return 3982; }
        if block_state.r#power == 15 && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#east == East::Side { return 4666; }
        if block_state.r#power == 12 && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Side { return 4646; }
        if block_state.r#power == 15 && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Side { return 3952; }
        if block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#power == 9 { return 4324; }
        if block_state.r#north == North::Up && block_state.r#power == 2 && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#south == South::Side { return 3831; }
        if block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#power == 14 && block_state.r#north == North::Side && block_state.r#south == South::Side { return 4948; }
        if block_state.r#south == South::Up && block_state.r#power == 13 && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#east == East::None { return 5080; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 3 { return 3837; }
        if block_state.r#power == 4 && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::None { return 4568; }
        if block_state.r#east == East::None && block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::None { return 4979; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#power == 13 { return 3929; }
        if block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#power == 0 && block_state.r#north == North::Up && block_state.r#south == South::None { return 4250; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 8 && block_state.r#south == South::None { return 4609; }
        if block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#power == 1 && block_state.r#north == North::None { return 4109; }
        if block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#power == 1 { return 4254; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 11 && block_state.r#north == North::None { return 5067; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 5 && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4576; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#power == 4 && block_state.r#south == South::Up { return 5000; }
        if block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#power == 7 { return 4017; }
        if block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 12 { return 4929; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#power == 4 && block_state.r#south == South::None && block_state.r#east == East::None { return 5006; }
        if block_state.r#power == 9 && block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#south == South::None { return 4475; }
        if block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 10 && block_state.r#south == South::Up && block_state.r#east == East::Side { return 4476; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#power == 10 { return 4484; }
        if block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 12 && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4066; }
        if block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#power == 11 && block_state.r#north == North::None && block_state.r#west == West::None { return 4637; }
        if block_state.r#west == West::Side && block_state.r#power == 0 && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Up { return 4681; }
        if block_state.r#power == 11 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::Side { return 5065; }
        if block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 10 && block_state.r#west == West::Side { return 4483; }
        if block_state.r#west == West::None && block_state.r#power == 2 && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#south == South::None { return 4268; }
        if block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 0 { return 4387; }
        if block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 0 { return 4102; }
        if block_state.r#power == 14 && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4228; }
        if block_state.r#north == North::Up && block_state.r#power == 15 && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None { return 4817; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#power == 11 { return 4058; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#power == 8 { return 3890; }
        if block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#power == 12 && block_state.r#north == North::Up && block_state.r#south == South::Up { return 4351; }
        if block_state.r#power == 4 && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#west == West::Up { return 3846; }
        if block_state.r#power == 9 && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#south == South::Up { return 3893; }
        if block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#west == West::Side { return 4114; }
        if block_state.r#power == 5 && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#south == South::Up { return 3857; }
        if block_state.r#east == East::Side && block_state.r#power == 0 && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::Up { return 4247; }
        if block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#power == 4 && block_state.r#east == East::None && block_state.r#north == North::Side { return 4855; }
        if block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#power == 1 { return 4685; }
        if block_state.r#power == 4 && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4710; }
        if block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#power == 5 && block_state.r#south == South::None && block_state.r#west == West::None { return 4727; }
        if block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#power == 0 && block_state.r#north == North::Up && block_state.r#west == West::Side { return 4678; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 12 && block_state.r#north == North::None { return 4212; }
        if block_state.r#power == 1 && block_state.r#west == West::Up && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#north == North::None { return 4542; }
        if block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#power == 9 { return 4756; }
        if block_state.r#power == 7 && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#north == North::None { return 4162; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 8 && block_state.r#east == East::Side { return 4462; }
        if block_state.r#power == 15 && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#west == West::Up { return 4521; }
        if block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#power == 10 && block_state.r#north == North::Side && block_state.r#south == South::None { return 4914; }
        if block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#power == 3 && block_state.r#south == South::None && block_state.r#north == North::None { return 4996; }
        if block_state.r#north == North::Up && block_state.r#power == 11 && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#west == West::None { return 4349; }
        if block_state.r#power == 8 && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::Side { return 4033; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 8 && block_state.r#south == South::Side && block_state.r#west == West::None { return 4895; }
        if block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 2 { return 4548; }
        if block_state.r#power == 3 && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#north == North::Side { return 4419; }
        if block_state.r#east == East::Side && block_state.r#power == 3 && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4560; }
        if block_state.r#power == 12 && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#south == South::Side { return 4785; }
        if block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 9 { return 3891; }
        if block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#south == South::Up { return 4971; }
        if block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#power == 10 { return 3906; }
        if block_state.r#east == East::Side && block_state.r#power == 6 && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#south == South::Side { return 4444; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#power == 10 && block_state.r#east == East::Up && block_state.r#west == West::Up { return 4194; }
        if block_state.r#power == 10 && block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#south == South::Side { return 4479; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#power == 15 && block_state.r#west == West::Side { return 4528; }
        if block_state.r#south == South::None && block_state.r#power == 13 && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#east == East::None { return 4943; }
        if block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#south == South::None { return 4115; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#power == 7 && block_state.r#west == West::Up && block_state.r#east == East::Up { return 4020; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 13 && block_state.r#east == East::Side && block_state.r#north == North::Up { return 4365; }
        if block_state.r#power == 6 && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Side { return 4733; }
        if block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#power == 6 && block_state.r#north == North::Up && block_state.r#south == South::Side { return 4299; }
        if block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#west == West::None && block_state.r#power == 5 { return 4145; }
        if block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#power == 2 && block_state.r#south == South::None { return 4842; }
        if block_state.r#power == 7 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#west == West::Up { return 4884; }
        if block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 15 && block_state.r#south == South::Up { return 4809; }
        if block_state.r#power == 12 && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::Up { return 4352; }
        if block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#west == West::Side && block_state.r#power == 9 { return 4042; }
        if block_state.r#power == 13 && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#south == South::Up { return 4505; }
        if block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#power == 11 && block_state.r#south == South::Side && block_state.r#north == North::None { return 4202; }
        if block_state.r#power == 0 && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#east == East::Side { return 4248; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#power == 8 && block_state.r#east == East::Up { return 4177; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 3 && block_state.r#west == West::None && block_state.r#south == South::Up { return 3983; }
        if block_state.r#power == 8 && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::Up { return 4171; }
        if block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 9 { return 4903; }
        if block_state.r#north == North::None && block_state.r#power == 1 && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#south == South::None { return 4977; }
        if block_state.r#power == 9 && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#east == East::Up { return 3896; }
        if block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#power == 11 { return 3909; }
        if block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 0 && block_state.r#south == South::Up { return 3956; }
        if block_state.r#west == West::Up && block_state.r#south == South::Side && block_state.r#power == 2 && block_state.r#east == East::Up && block_state.r#north == North::Side { return 3975; }
        if block_state.r#south == South::None && block_state.r#power == 11 && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::Side { return 4060; }
        if block_state.r#east == East::Side && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 0 && block_state.r#south == South::Side { return 4245; }
        if block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#power == 8 && block_state.r#north == North::Up && block_state.r#south == South::Side { return 4317; }
        if block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#power == 1 { return 4688; }
        if block_state.r#power == 6 && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#north == North::Side { return 4878; }
        if block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#power == 6 && block_state.r#south == South::None && block_state.r#north == North::None { return 5022; }
        if block_state.r#power == 13 && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#west == West::None { return 5084; }
        if block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#power == 0 && block_state.r#south == South::Up { return 3954; }
        if block_state.r#south == South::None && block_state.r#power == 8 && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#north == North::Up { return 4320; }
        if block_state.r#power == 9 && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#west == West::Up { return 4755; }
        if block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 15 { return 4955; }
        if block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#power == 14 && block_state.r#east == East::None && block_state.r#north == North::Side { return 4951; }
        if block_state.r#power == 9 && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#west == West::Up { return 4467; }
        if block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#power == 14 { return 4517; }
        if block_state.r#power == 2 && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::Side { return 4552; }
        if block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 14 { return 4803; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 1 { return 4545; }
        if block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#power == 1 { return 3823; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#power == 7 && block_state.r#south == South::Up && block_state.r#west == West::None { return 3875; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 2 && block_state.r#south == South::Up && block_state.r#west == West::None { return 4550; }
        if block_state.r#power == 5 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::Side { return 4866; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 10 && block_state.r#north == North::None { return 5060; }
        if block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#power == 7 && block_state.r#east == East::Side { return 4309; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::Side && block_state.r#power == 3 { return 4414; }
        if block_state.r#west == West::Up && block_state.r#power == 6 && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::None { return 4302; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#power == 12 { return 5078; }
        if block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#power == 12 && block_state.r#east == East::Up && block_state.r#north == North::None { return 4209; }
        if block_state.r#power == 6 && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#west == West::Up { return 4587; }
        if block_state.r#east == East::Up && block_state.r#power == 0 && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#north == North::Up { return 3816; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#power == 7 { return 5033; }
        if block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 14 && block_state.r#south == South::None && block_state.r#east == East::None { return 4950; }
        if block_state.r#west == West::Side && block_state.r#power == 1 && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#east == East::None { return 4975; }
        if block_state.r#power == 12 && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#north == North::None { return 5075; }
        if block_state.r#power == 7 && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::Up { return 4737; }
        if block_state.r#power == 5 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4000; }
        if block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#power == 14 && block_state.r#south == South::None && block_state.r#west == West::Up { return 4806; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#power == 14 { return 4949; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 11 && block_state.r#south == South::Side { return 5066; }
        if block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#power == 4 && block_state.r#east == East::None && block_state.r#west == West::Up { return 5001; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#power == 7 { return 4164; }
        if block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 13 { return 4073; }
        if block_state.r#power == 10 && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side { return 5056; }
        if block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 8 { return 4890; }
        if block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#power == 5 && block_state.r#south == South::Side { return 5010; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#power == 5 && block_state.r#south == South::None { return 3863; }
        if block_state.r#power == 6 && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#west == West::Up { return 4590; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 11 && block_state.r#east == East::Up && block_state.r#north == North::Up { return 3915; }
        if block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#power == 13 && block_state.r#east == East::Up && block_state.r#north == North::Side { return 4074; }
        if block_state.r#power == 8 && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#west == West::None { return 4610; }
        if block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 1 && block_state.r#east == East::None && block_state.r#north == North::Up { return 4683; }
        if block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 7 && block_state.r#west == West::Up && block_state.r#north == North::Up { return 4740; }
        if block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#west == West::Up && block_state.r#power == 10 { return 5052; }
        if block_state.r#north == North::Side && block_state.r#power == 6 && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#east == East::Up { return 4011; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#power == 8 && block_state.r#west == West::Up { return 4314; }
        if block_state.r#power == 11 && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#south == South::None { return 4492; }
        if block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#power == 3 && block_state.r#north == North::None { return 4557; }
        if block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#power == 8 { return 3886; }
        if block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#power == 3 && block_state.r#west == West::None && block_state.r#south == South::None { return 4421; }
        if block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 1 && block_state.r#south == South::Up && block_state.r#north == North::Up { return 3819; }
        if block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 10 { return 4046; }
        if block_state.r#north == North::Side && block_state.r#power == 9 && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#south == South::Side { return 4470; }
        if block_state.r#power == 2 && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#west == West::Up { return 4122; }
        if block_state.r#south == South::Side && block_state.r#power == 10 && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::None { return 3905; }
        if block_state.r#west == West::Up && block_state.r#power == 0 && block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#east == East::None { return 4674; }
        if block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#west == West::Side { return 4690; }
        if block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 15 && block_state.r#south == South::Side { return 4093; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#power == 2 { return 4700; }
        if block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#power == 9 { return 4906; }
        if block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#power == 3 { return 4990; }
        if block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#power == 14 && block_state.r#west == West::None && block_state.r#east == East::Up { return 3941; }
        if block_state.r#west == West::None && block_state.r#power == 13 && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#north == North::None { return 5081; }
        if block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#power == 11 && block_state.r#north == North::Up && block_state.r#west == West::None { return 4775; }
        if block_state.r#south == South::Up && block_state.r#power == 1 && block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#west == West::Side { return 3820; }
        if block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 1 && block_state.r#south == South::None { return 3825; }
        if block_state.r#north == North::Up && block_state.r#power == 11 && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4345; }
        if block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#power == 13 { return 4366; }
        if block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#power == 8 && block_state.r#east == East::Up && block_state.r#west == West::Up { return 3888; }
        if block_state.r#west == West::Side && block_state.r#power == 4 && block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#east == East::None { return 4714; }
        if block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#power == 10 && block_state.r#south == South::None { return 4771; }
        if block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#power == 2 { return 4982; }
        if block_state.r#power == 10 && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#west == West::Side { return 5053; }
        if block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 11 { return 4485; }
        if block_state.r#power == 4 && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#east == East::Up { return 4134; }
        if block_state.r#west == West::Side && block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#power == 10 && block_state.r#north == North::Side { return 4480; }
        if block_state.r#east == East::None && block_state.r#power == 14 && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#north == North::None { return 5088; }
        if block_state.r#north == North::Side && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#power == 14 && block_state.r#west == West::Up { return 4515; }
        if block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#power == 10 && block_state.r#east == East::None && block_state.r#north == North::None { return 5057; }
        if block_state.r#south == South::Up && block_state.r#power == 0 && block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#east == East::Side { return 4388; }
        if block_state.r#power == 7 && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#south == South::Up { return 4738; }
        if block_state.r#power == 2 && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#north == North::None { return 4549; }
        if block_state.r#west == West::Up && block_state.r#power == 10 && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::None { return 4620; }
        if block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#power == 8 && block_state.r#east == East::Up { return 4030; }
        if block_state.r#power == 0 && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::None { return 4106; }
        if block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#power == 2 { return 4116; }
        if block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 14 { return 4369; }
        if block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#power == 8 { return 3889; }
        if block_state.r#east == East::Up && block_state.r#power == 1 && block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#west == West::None { return 3971; }
        if block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#power == 0 { return 4389; }
        if block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 6 { return 4014; }
        if block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#power == 15 { return 4812; }
        if block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 6 && block_state.r#north == North::Up { return 3868; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 10 { return 4916; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 11 && block_state.r#south == South::None && block_state.r#west == West::Up { return 4203; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#power == 5 && block_state.r#east == East::Up && block_state.r#west == West::Side { return 4147; }
        if block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#power == 2 && block_state.r#south == South::Up && block_state.r#east == East::Up { return 3829; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#power == 11 && block_state.r#north == North::Side { return 4059; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#power == 12 && block_state.r#west == West::None { return 4067; }
        if block_state.r#west == West::None && block_state.r#power == 4 && block_state.r#north == North::Up && block_state.r#south == South::None && block_state.r#east == East::Side { return 4286; }
        if block_state.r#power == 4 && block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#west == West::Side { return 3994; }
        if block_state.r#east == East::Up && block_state.r#power == 10 && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#south == South::None { return 4195; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 4 { return 3996; }
        if block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#power == 1 { return 4684; }
        if block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 1 && block_state.r#north == North::Up && block_state.r#south == South::None { return 3826; }
        if block_state.r#power == 6 && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Up { return 4442; }
        if block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#power == 14 && block_state.r#east == East::Side && block_state.r#south == South::Side { return 4516; }
        if block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::None && block_state.r#power == 11 && block_state.r#east == East::Side { return 4632; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 8 && block_state.r#south == South::Up && block_state.r#west == West::Up { return 4602; }
        if block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 8 && block_state.r#north == North::Up && block_state.r#south == South::Up { return 3883; }
        if block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#power == 1 { return 3824; }
        if block_state.r#power == 6 && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::Up { return 4013; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 10 && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4191; }
        if block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#power == 15 && block_state.r#north == North::None && block_state.r#east == East::Up { return 4237; }
        if block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#power == 10 && block_state.r#west == West::Up && block_state.r#east == East::None { return 4767; }
        if block_state.r#north == North::Side && block_state.r#power == 1 && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#west == West::None { return 4832; }
        if block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#power == 3 && block_state.r#east == East::None { return 4850; }
        if block_state.r#east == East::Up && block_state.r#power == 3 && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None { return 4133; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#power == 12 && block_state.r#south == South::None && block_state.r#west == West::Up { return 5076; }
        if block_state.r#power == 4 && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4858; }
        if block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#power == 1 && block_state.r#north == North::Side { return 3963; }
        if block_state.r#east == East::Side && block_state.r#power == 7 && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#south == South::Side { return 4452; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#power == 5 && block_state.r#south == South::Side && block_state.r#east == East::Side { return 4580; }
        if block_state.r#power == 6 && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Up && block_state.r#north == North::Side { return 4016; }
        if block_state.r#east == East::None && block_state.r#power == 9 && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::Side { return 4904; }
        if block_state.r#south == South::Up && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#power == 11 { return 4342; }
        if block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#power == 12 && block_state.r#east == East::None && block_state.r#south == South::Up { return 4926; }
        if block_state.r#power == 3 && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#south == South::None { return 4132; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#power == 15 && block_state.r#north == North::Up { return 4378; }
        if block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 6 && block_state.r#west == West::Up && block_state.r#south == South::Up { return 4008; }
        if block_state.r#power == 0 && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#east == East::None { return 4677; }
        if block_state.r#power == 5 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::Side { return 4006; }
        if block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 7 { return 3877; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#power == 8 { return 4891; }
        if block_state.r#power == 8 && block_state.r#north == North::None && block_state.r#west == West::Up && block_state.r#south == South::Up && block_state.r#east == East::None { return 5034; }
        if block_state.r#east == East::Up && block_state.r#power == 10 && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::None { return 4049; }
        if block_state.r#east == East::Up && block_state.r#power == 7 && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4165; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 13 && block_state.r#south == South::None && block_state.r#west == West::Up { return 4221; }
        if block_state.r#east == East::Side && block_state.r#power == 5 && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::None { return 4577; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#power == 10 && block_state.r#north == North::Up { return 4772; }
        if block_state.r#east == East::Up && block_state.r#north == North::Up && block_state.r#power == 0 && block_state.r#south == South::Side && block_state.r#west == West::None { return 3815; }
        if block_state.r#north == North::None && block_state.r#power == 8 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::None { return 5039; }
        if block_state.r#south == South::Side && block_state.r#power == 13 && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#east == East::Up { return 4219; }
        if block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 11 && block_state.r#north == North::None { return 4197; }
        if block_state.r#power == 8 && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#north == North::Up && block_state.r#west == West::Side { return 4321; }
        if block_state.r#south == South::Up && block_state.r#power == 10 && block_state.r#west == West::Side && block_state.r#east == East::Side && block_state.r#north == North::Side { return 4477; }
        if block_state.r#north == North::Up && block_state.r#power == 8 && block_state.r#east == East::Up && block_state.r#south == South::Side && block_state.r#west == West::Up { return 3885; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#power == 5 && block_state.r#west == West::None && block_state.r#south == South::None { return 4151; }
        if block_state.r#north == North::Up && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#power == 0 && block_state.r#west == West::None { return 4244; }
        if block_state.r#power == 9 && block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#south == South::None { return 4329; }
        if block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#east == East::Up && block_state.r#power == 5 { return 4004; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#power == 12 { return 4639; }
        if block_state.r#power == 0 && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::None && block_state.r#north == North::Side { return 4822; }
        if block_state.r#power == 0 && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#east == East::Up { return 3961; }
        if block_state.r#west == West::None && block_state.r#north == North::Up && block_state.r#power == 7 && block_state.r#east == East::Up && block_state.r#south == South::Side { return 3878; }
        if block_state.r#east == East::Side && block_state.r#power == 0 && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#north == North::Side { return 4393; }
        if block_state.r#power == 11 && block_state.r#north == North::Up && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Up { return 3917; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#power == 7 { return 4451; }
        if block_state.r#west == West::Side && block_state.r#north == North::Up && block_state.r#power == 6 && block_state.r#south == South::None && block_state.r#east == East::None { return 4735; }
        if block_state.r#west == West::None && block_state.r#power == 8 && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#south == South::None { return 4754; }
        if block_state.r#east == East::Up && block_state.r#power == 7 && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::None { return 4169; }
        if block_state.r#west == West::Up && block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#power == 2 { return 4839; }
        if block_state.r#power == 2 && block_state.r#south == South::None && block_state.r#east == East::Up && block_state.r#west == West::Up && block_state.r#north == North::Up { return 3834; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#power == 6 && block_state.r#west == West::Side { return 4876; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#power == 6 { return 5024; }
        if block_state.r#power == 9 && block_state.r#south == South::Side && block_state.r#west == West::Up && block_state.r#east == East::None && block_state.r#north == North::None { return 5046; }
        if block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#power == 11 && block_state.r#west == West::Up && block_state.r#south == South::Up { return 4629; }
        if block_state.r#power == 14 && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#west == West::Side { return 4513; }
        if block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#power == 7 && block_state.r#west == West::Side && block_state.r#north == North::Side { return 4885; }
        if block_state.r#north == North::Up && block_state.r#power == 9 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::Side { return 4759; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#power == 11 && block_state.r#north == North::None && block_state.r#east == East::Side { return 4635; }
        if block_state.r#north == North::Up && block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#south == South::Up && block_state.r#power == 2 { return 3828; }
        if block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 11 && block_state.r#west == West::None && block_state.r#south == South::None { return 4205; }
        if block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 3 && block_state.r#east == East::Side { return 4276; }
        if block_state.r#east == East::Side && block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#power == 11 { return 4346; }
        if block_state.r#north == North::Side && block_state.r#power == 14 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::Up { return 4947; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#power == 4 && block_state.r#north == North::Side { return 4430; }
        if block_state.r#north == North::Side && block_state.r#east == East::Up && block_state.r#power == 4 && block_state.r#west == West::None && block_state.r#south == South::None { return 3998; }
        if block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#power == 3 && block_state.r#north == North::Side && block_state.r#west == West::Side { return 3988; }
        if block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 12 && block_state.r#east == East::Side && block_state.r#south == South::Up { return 4350; }
        if block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 6 && block_state.r#south == South::Up && block_state.r#west == West::None { return 4586; }
        if block_state.r#east == East::Side && block_state.r#power == 12 && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#south == South::Side { return 4353; }
        if block_state.r#east == East::Side && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#power == 7 { return 4456; }
        if block_state.r#south == South::Up && block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 5 && block_state.r#west == West::Up { return 4431; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#power == 11 { return 4486; }
        if block_state.r#north == North::Up && block_state.r#power == 5 && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#east == East::None { return 4720; }
        if block_state.r#south == South::Side && block_state.r#north == North::Side && block_state.r#west == West::Up && block_state.r#power == 3 && block_state.r#east == East::None { return 4848; }
        if block_state.r#power == 2 && block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#west == West::Side { return 4120; }
        if block_state.r#south == South::Side && block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#power == 4 && block_state.r#east == East::Side { return 4282; }
        if block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#power == 7 && block_state.r#west == West::None && block_state.r#north == North::Side { return 4457; }
        if block_state.r#north == North::Up && block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#power == 0 { return 3814; }
        if block_state.r#power == 7 && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#west == West::Up { return 4167; }
        if block_state.r#east == East::Up && block_state.r#power == 11 && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::None { return 4055; }
        if block_state.r#power == 4 && block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#east == East::Side { return 4429; }
        if block_state.r#west == West::None && block_state.r#power == 0 && block_state.r#north == North::Up && block_state.r#east == East::None && block_state.r#south == South::Side { return 4679; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#power == 1 && block_state.r#west == West::None && block_state.r#south == South::Up { return 4397; }
        if block_state.r#power == 15 && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#east == East::Side { return 4526; }
        if block_state.r#north == North::Up && block_state.r#power == 8 && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#west == West::Side { return 4747; }
        if block_state.r#north == North::Side && block_state.r#power == 15 && block_state.r#west == West::Up && block_state.r#east == East::Side && block_state.r#south == South::None { return 4527; }
        if block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 13 && block_state.r#south == South::Up && block_state.r#east == East::None { return 4791; }
        if block_state.r#power == 2 && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#east == East::None { return 4844; }
        if block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#power == 6 { return 3871; }
        if block_state.r#south == South::Side && block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 3 && block_state.r#west == West::Up { return 4272; }
        if block_state.r#east == East::None && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#north == North::Side && block_state.r#power == 13 { return 4940; }
        if block_state.r#east == East::Side && block_state.r#power == 3 && block_state.r#west == West::None && block_state.r#south == South::Up && block_state.r#north == North::Up { return 4271; }
        if block_state.r#west == West::Side && block_state.r#power == 10 && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::Up { return 4045; }
        if block_state.r#power == 13 && block_state.r#south == South::None && block_state.r#east == East::Side && block_state.r#north == North::None && block_state.r#west == West::None { return 4655; }
        if block_state.r#north == North::None && block_state.r#west == West::Side && block_state.r#south == South::None && block_state.r#power == 8 && block_state.r#east == East::None { return 5041; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#power == 10 && block_state.r#west == West::Side { return 4915; }
        if block_state.r#west == West::None && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#east == East::Up && block_state.r#power == 1 { return 4112; }
        if block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#north == North::Up && block_state.r#power == 6 && block_state.r#east == East::Up { return 3870; }
        if block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#power == 15 && block_state.r#east == East::None { return 4960; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#power == 11 && block_state.r#west == West::Up { return 5064; }
        if block_state.r#east == East::Up && block_state.r#north == North::None && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 6 { return 4152; }
        if block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 6 && block_state.r#north == North::Up && block_state.r#east == East::Side { return 4297; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#power == 4 { return 4427; }
        if block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#power == 0 && block_state.r#east == East::Up { return 3962; }
        if block_state.r#north == North::None && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::Side && block_state.r#power == 5 { return 5011; }
        if block_state.r#north == North::Up && block_state.r#west == West::Side && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#power == 13 { return 4792; }
        if block_state.r#power == 7 && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Up { return 5027; }
        if block_state.r#east == East::Side && block_state.r#north == North::Up && block_state.r#power == 2 && block_state.r#south == South::Side && block_state.r#west == West::Up { return 4263; }
        if block_state.r#power == 1 && block_state.r#south == South::Up && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#west == West::Side { return 3964; }
        if block_state.r#east == East::Side && block_state.r#north == North::Side && block_state.r#west == West::Side && block_state.r#power == 6 && block_state.r#south == South::None { return 4447; }
        if block_state.r#power == 15 && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#west == West::Up && block_state.r#north == North::None { return 5103; }
        if block_state.r#south == South::Side && block_state.r#power == 5 && block_state.r#north == North::Up && block_state.r#east == East::Up && block_state.r#west == West::Up { return 3858; }
        if block_state.r#east == East::Up && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#power == 12 && block_state.r#west == West::Side { return 4213; }
        if block_state.r#east == East::Side && block_state.r#power == 4 && block_state.r#south == South::None && block_state.r#north == North::Side && block_state.r#west == West::Up { return 4428; }
        if block_state.r#west == West::Up && block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#power == 10 && block_state.r#south == South::None { return 4050; }
        if block_state.r#north == North::Side && block_state.r#east == East::None && block_state.r#power == 4 && block_state.r#south == South::None && block_state.r#west == West::Side { return 4861; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#power == 8 && block_state.r#west == West::Up && block_state.r#north == North::Side { return 4896; }
        if block_state.r#north == North::Side && block_state.r#power == 7 && block_state.r#east == East::Side && block_state.r#south == South::None && block_state.r#west == West::Up { return 4455; }
        if block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#south == South::Side && block_state.r#power == 12 && block_state.r#west == West::Side { return 4642; }
        if block_state.r#east == East::None && block_state.r#power == 0 && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::Side { return 4819; }
        if block_state.r#north == North::Side && block_state.r#south == South::Side && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#power == 0 { return 4823; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#power == 14 && block_state.r#west == West::None && block_state.r#north == North::Side { return 4514; }
        if block_state.r#east == East::Up && block_state.r#north == North::Side && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#power == 3 { return 3989; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Up && block_state.r#power == 8 && block_state.r#south == South::Up { return 4748; }
        if block_state.r#east == East::None && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#west == West::Side && block_state.r#power == 15 { return 4954; }
        if block_state.r#west == West::None && block_state.r#east == East::Side && block_state.r#power == 12 && block_state.r#south == South::Side && block_state.r#north == North::Up { return 4355; }
        if block_state.r#east == East::Side && block_state.r#south == South::Up && block_state.r#west == West::Up && block_state.r#power == 9 && block_state.r#north == North::Up { return 4323; }
        if block_state.r#west == West::Side && block_state.r#power == 14 && block_state.r#north == North::Up && block_state.r#east == East::Side && block_state.r#south == South::Side { return 4372; }
        if block_state.r#south == South::Up && block_state.r#west == West::None && block_state.r#power == 14 && block_state.r#east == East::Side && block_state.r#north == North::Up { return 4370; }
        if block_state.r#north == North::Side && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Up && block_state.r#power == 0 { return 4820; }
        if block_state.r#east == East::None && block_state.r#power == 9 && block_state.r#west == West::Up && block_state.r#south == South::None && block_state.r#north == North::None { return 5049; }
        if block_state.r#north == North::Up && block_state.r#power == 11 && block_state.r#south == South::Side && block_state.r#east == East::None && block_state.r#west == West::None { return 4778; }
        if block_state.r#east == East::Up && block_state.r#power == 3 && block_state.r#south == South::Side && block_state.r#north == North::None && block_state.r#west == West::Side { return 4129; }
        if block_state.r#south == South::None && block_state.r#west == West::Side && block_state.r#north == North::Side && block_state.r#power == 8 && block_state.r#east == East::Side { return 4465; }
        if block_state.r#power == 0 && block_state.r#north == North::Side && block_state.r#south == South::Up && block_state.r#east == East::None && block_state.r#west == West::Up { return 4818; }
        if block_state.r#south == South::Side && block_state.r#west == West::Side && block_state.r#north == North::None && block_state.r#east == East::Side && block_state.r#power == 4 { return 4570; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 4333 {
            return Some(RedstoneWire {
                r#power: 10,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4198 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::Side,
                r#power: 11,
            });
        }
        if state_id == 4338 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::None,
                r#power: 10,
            });
        }
        if state_id == 3966 {
            return Some(RedstoneWire {
                r#power: 1,
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4898 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#south: South::None,
                r#power: 8,
                r#east: East::None,
            });
        }
        if state_id == 3866 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#south: South::Up,
                r#east: East::Up,
                r#power: 6,
            });
        }
        if state_id == 4356 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#power: 12,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4762 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4434 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#power: 5,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4131 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::None,
                r#power: 3,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 5042 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 8,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4667 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Side,
                r#power: 15,
                r#south: South::Up,
            });
        }
        if state_id == 3892 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#west: West::Side,
                r#power: 9,
                r#south: South::Up,
            });
        }
        if state_id == 3931 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#power: 13,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4037 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 9,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4908 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::None,
                r#west: West::Up,
                r#power: 10,
                r#north: North::Side,
            });
        }
        if state_id == 4024 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 7,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4084 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 14,
                r#east: East::Up,
            });
        }
        if state_id == 3856 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 5,
                r#south: South::Up,
            });
        }
        if state_id == 4005 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 5,
                r#west: West::Up,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4159 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#south: South::None,
                r#east: East::Up,
                r#power: 6,
            });
        }
        if state_id == 4555 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 2,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4371 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 14,
            });
        }
        if state_id == 4384 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4124 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 2,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 3904 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 10,
                r#north: North::Up,
            });
        }
        if state_id == 4598 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#power: 7,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4564 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#north: North::None,
                r#power: 3,
                r#east: East::Side,
            });
        }
        if state_id == 4242 {
            return Some(RedstoneWire {
                r#power: 0,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4408 {
            return Some(RedstoneWire {
                r#power: 2,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 3916 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
                r#power: 11,
            });
        }
        if state_id == 4987 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#power: 2,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 3879 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 7,
            });
        }
        if state_id == 4207 {
            return Some(RedstoneWire {
                r#power: 12,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4801 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#power: 14,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4899 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#power: 9,
            });
        }
        if state_id == 3986 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Up,
                r#power: 3,
                r#south: South::Side,
            });
        }
        if state_id == 4830 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Side,
                r#power: 1,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4200 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 11,
                r#east: East::Up,
            });
        }
        if state_id == 4304 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Up,
                r#power: 6,
            });
        }
        if state_id == 4264 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 2,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4305 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 7,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4651 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::None,
                r#power: 13,
                r#south: South::Side,
            });
        }
        if state_id == 4091 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 15,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4723 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::None,
                r#power: 5,
                r#north: North::Up,
            });
        }
        if state_id == 4098 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::None,
                r#power: 0,
            });
        }
        if state_id == 4963 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::None,
                r#power: 0,
            });
        }
        if state_id == 5094 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Up,
                r#power: 14,
            });
        }
        if state_id == 4406 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::None,
                r#power: 2,
            });
        }
        if state_id == 3849 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::Up,
                r#power: 4,
            });
        }
        if state_id == 5061 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 5098 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 15,
                r#east: East::None,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4512 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 14,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
            });
        }
        if state_id == 5074 {
            return Some(RedstoneWire {
                r#power: 12,
                r#south: South::Side,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 4873 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 6,
                r#north: North::Side,
            });
        }
        if state_id == 5077 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 12,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4845 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::None,
                r#north: North::Side,
                r#power: 3,
            });
        }
        if state_id == 4231 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Side,
                r#power: 14,
                r#south: South::None,
            });
        }
        if state_id == 4285 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#power: 4,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 3833 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Side,
                r#power: 2,
            });
        }
        if state_id == 4111 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 1,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4439 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::None,
                r#power: 5,
            });
        }
        if state_id == 4154 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 6,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 3936 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 14,
            });
        }
        if state_id == 3911 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#power: 11,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4734 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::None,
                r#east: East::None,
                r#power: 6,
            });
        }
        if state_id == 5062 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 11,
                r#east: East::None,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4256 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#power: 1,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4235 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#south: South::Up,
                r#power: 15,
                r#west: West::None,
            });
        }
        if state_id == 4879 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::None,
                r#power: 6,
            });
        }
        if state_id == 4382 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 15,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4779 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::None,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4537 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 0,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4624 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 10,
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4981 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 2,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4025 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#power: 7,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4238 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#west: West::None,
                r#power: 15,
                r#east: East::Up,
            });
        }
        if state_id == 5059 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#power: 10,
                r#west: West::Side,
            });
        }
        if state_id == 3927 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Up,
                r#power: 13,
                r#west: West::Up,
            });
        }
        if state_id == 4075 {
            return Some(RedstoneWire {
                r#power: 13,
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4083 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#power: 14,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4411 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 2,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 4974 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#power: 1,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4504 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 13,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4614 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 9,
                r#south: South::Side,
                r#north: North::None,
                r#east: East::Side,
            });
        }
        if state_id == 4022 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Up,
                r#power: 7,
                r#south: South::Side,
            });
        }
        if state_id == 4178 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Up,
                r#power: 8,
            });
        }
        if state_id == 4623 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 10,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4220 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::Up,
                r#west: West::None,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4702 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#power: 3,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4233 {
            return Some(RedstoneWire {
                r#power: 15,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4862 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#power: 4,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4864 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#power: 5,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4547 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 1,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 3973 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 2,
                r#north: North::Side,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4731 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::None,
                r#power: 6,
            });
        }
        if state_id == 3855 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 5,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4774 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 4007 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 5,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4880 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#power: 6,
                r#north: North::Side,
            });
        }
        if state_id == 4330 {
            return Some(RedstoneWire {
                r#power: 9,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4530 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::Up,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::None,
            });
        }
        if state_id == 4676 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::None,
                r#north: North::Up,
                r#power: 0,
                r#west: West::None,
            });
        }
        if state_id == 4907 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 9,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4905 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::None,
                r#power: 9,
            });
        }
        if state_id == 4994 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#power: 3,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4362 {
            return Some(RedstoneWire {
                r#power: 13,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4571 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 5073 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#power: 12,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 3880 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Up,
                r#power: 7,
            });
        }
        if state_id == 4260 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::Side,
                r#power: 2,
                r#west: West::Up,
            });
        }
        if state_id == 4273 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::Side,
                r#power: 3,
            });
        }
        if state_id == 4958 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#power: 15,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4675 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#power: 0,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 3867 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#power: 6,
                r#north: North::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4749 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4854 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::None,
                r#power: 4,
                r#west: West::Up,
            });
        }
        if state_id == 5071 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#power: 12,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4150 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4062 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 12,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4930 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::Side,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 5008 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::Side,
                r#power: 5,
            });
        }
        if state_id == 4553 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#power: 2,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 4687 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Side,
                r#power: 1,
                r#north: North::Up,
                r#east: East::None,
            });
        }
        if state_id == 4368 {
            return Some(RedstoneWire {
                r#power: 14,
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4730 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 6,
                r#east: East::None,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4945 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Up,
                r#power: 14,
            });
        }
        if state_id == 4713 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 4,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 3922 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 12,
                r#west: West::Side,
            });
        }
        if state_id == 3938 {
            return Some(RedstoneWire {
                r#power: 14,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 5003 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 4,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4594 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#south: South::Up,
                r#power: 7,
                r#east: East::Side,
            });
        }
        if state_id == 4865 {
            return Some(RedstoneWire {
                r#power: 5,
                r#west: West::None,
                r#south: South::Up,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4267 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 2,
            });
        }
        if state_id == 5085 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 13,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 4316 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::None,
                r#power: 8,
                r#south: South::Up,
            });
        }
        if state_id == 4196 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Up,
                r#power: 10,
            });
        }
        if state_id == 4379 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#power: 15,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 4166 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::None,
                r#power: 7,
                r#south: South::Side,
            });
        }
        if state_id == 4380 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4496 {
            return Some(RedstoneWire {
                r#power: 12,
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4585 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::None,
                r#power: 6,
            });
        }
        if state_id == 4868 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::None,
                r#north: North::Side,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4390 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::Side,
            });
        }
        if state_id == 3935 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 13,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 3984 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 3,
                r#west: West::Up,
            });
        }
        if state_id == 4575 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 5,
                r#west: West::Up,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4432 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 5,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4742 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#west: West::None,
                r#power: 7,
                r#south: South::Side,
            });
        }
        if state_id == 4686 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#south: South::Side,
                r#power: 1,
                r#north: North::Up,
            });
        }
        if state_id == 4631 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Side,
                r#power: 11,
            });
        }
        if state_id == 4743 {
            return Some(RedstoneWire {
                r#power: 7,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4110 {
            return Some(RedstoneWire {
                r#power: 1,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
            });
        }
        if state_id == 3921 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 12,
                r#east: East::Up,
            });
        }
        if state_id == 4361 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#power: 13,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4913 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Side,
                r#power: 10,
                r#west: West::None,
            });
        }
        if state_id == 5068 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::None,
                r#power: 11,
            });
        }
        if state_id == 3894 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#power: 9,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3881 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#power: 7,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4053 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3876 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#power: 7,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4027 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::Up,
                r#power: 8,
                r#south: South::Up,
            });
        }
        if state_id == 4100 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#north: North::None,
                r#power: 0,
                r#south: South::Up,
            });
        }
        if state_id == 4135 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 4,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4215 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::None,
                r#power: 13,
                r#west: West::Up,
            });
        }
        if state_id == 4225 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4343 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#power: 11,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4257 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 1,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4255 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 1,
            });
        }
        if state_id == 5013 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 5043 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 9,
            });
        }
        if state_id == 3842 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Up,
                r#east: East::Up,
                r#power: 3,
            });
        }
        if state_id == 4266 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#power: 2,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4920 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Side,
                r#power: 11,
            });
        }
        if state_id == 4795 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 13,
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4781 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#power: 11,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4076 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#south: South::Side,
                r#power: 13,
                r#east: East::Up,
            });
        }
        if state_id == 4377 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::Side,
                r#west: West::Up,
                r#power: 15,
            });
        }
        if state_id == 4158 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 6,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4597 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#power: 7,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 5093 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#west: West::None,
                r#power: 14,
                r#north: North::None,
            });
        }
        if state_id == 4565 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#power: 3,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 4995 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#power: 3,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 3950 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 15,
            });
        }
        if state_id == 4556 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::None,
                r#power: 2,
            });
        }
        if state_id == 4799 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 13,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 5089 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::None,
                r#power: 14,
            });
        }
        if state_id == 4709 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#south: South::None,
                r#power: 3,
                r#west: West::None,
            });
        }
        if state_id == 4567 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 4,
            });
        }
        if state_id == 4729 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::Up,
                r#power: 6,
            });
        }
        if state_id == 4529 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 15,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4463 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#power: 8,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4524 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 15,
                r#west: West::Up,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4559 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 3,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4998 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::None,
                r#power: 4,
            });
        }
        if state_id == 4840 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#power: 2,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4959 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#power: 15,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4386 {
            return Some(RedstoneWire {
                r#power: 0,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4170 {
            return Some(RedstoneWire {
                r#power: 8,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4658 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Up,
                r#east: East::Side,
                r#power: 14,
                r#north: North::None,
            });
        }
        if state_id == 4669 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 15,
                r#north: North::None,
            });
        }
        if state_id == 4836 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::None,
                r#power: 2,
                r#north: North::Side,
            });
        }
        if state_id == 4664 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 14,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 4912 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 10,
                r#north: North::Side,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4201 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#power: 11,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4383 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 15,
                r#east: East::Side,
                r#west: West::Up,
                r#south: South::None,
            });
        }
        if state_id == 4540 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#east: East::Side,
                r#power: 1,
                r#west: West::Side,
            });
        }
        if state_id == 3918 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Up,
                r#power: 12,
                r#west: West::Up,
            });
        }
        if state_id == 4563 {
            return Some(RedstoneWire {
                r#power: 3,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 3972 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#power: 2,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4259 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 1,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 4520 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#power: 14,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 5099 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#power: 15,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4391 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 0,
                r#west: West::None,
            });
        }
        if state_id == 4413 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 3,
                r#south: South::Up,
            });
        }
        if state_id == 4363 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#power: 13,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 3852 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 4,
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::None,
            });
        }
        if state_id == 4061 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#west: West::None,
                r#power: 11,
                r#north: North::Side,
            });
        }
        if state_id == 3812 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 0,
                r#north: North::Up,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4834 {
            return Some(RedstoneWire {
                r#power: 1,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4130 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 3,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4187 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 9,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 3835 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::None,
                r#power: 2,
                r#west: West::Side,
            });
        }
        if state_id == 4160 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 6,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4902 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 9,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4672 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::None,
                r#power: 15,
                r#north: North::None,
            });
        }
        if state_id == 3924 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Up,
                r#power: 12,
                r#north: North::Up,
            });
        }
        if state_id == 4956 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#power: 15,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4976 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Side,
                r#power: 1,
                r#north: North::None,
            });
        }
        if state_id == 4989 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 3,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 4295 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Up,
                r#west: West::None,
                r#power: 5,
            });
        }
        if state_id == 4869 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::None,
                r#power: 5,
            });
        }
        if state_id == 4739 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Up,
                r#power: 7,
            });
        }
        if state_id == 4095 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Up,
                r#power: 15,
            });
        }
        if state_id == 4983 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 2,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4783 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::None,
                r#power: 12,
                r#north: North::Up,
            });
        }
        if state_id == 3843 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 3,
            });
        }
        if state_id == 4769 {
            return Some(RedstoneWire {
                r#power: 10,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4280 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#south: South::Up,
                r#power: 4,
                r#north: North::Up,
            });
        }
        if state_id == 4252 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 1,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4640 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 12,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4279 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::Up,
                r#power: 4,
            });
        }
        if state_id == 3862 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Up,
                r#power: 5,
            });
        }
        if state_id == 5028 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Up,
                r#east: East::None,
                r#power: 7,
            });
        }
        if state_id == 3822 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#power: 1,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 3845 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::None,
                r#power: 3,
            });
        }
        if state_id == 5091 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4782 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Up,
                r#power: 12,
                r#south: South::Up,
            });
        }
        if state_id == 4420 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 3,
                r#south: South::None,
            });
        }
        if state_id == 4474 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 9,
                r#west: West::Side,
            });
        }
        if state_id == 4870 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::None,
                r#power: 5,
                r#south: South::None,
            });
        }
        if state_id == 4722 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Up,
                r#power: 5,
                r#south: South::Side,
            });
        }
        if state_id == 3987 {
            return Some(RedstoneWire {
                r#power: 3,
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 4308 {
            return Some(RedstoneWire {
                r#power: 7,
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4604 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::None,
                r#west: West::None,
                r#power: 8,
            });
        }
        if state_id == 3930 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 13,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4633 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 11,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4701 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 4856 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
                r#power: 4,
                r#east: East::None,
            });
        }
        if state_id == 4019 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::None,
                r#power: 7,
                r#south: South::Up,
            });
        }
        if state_id == 4204 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4269 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 3,
                r#north: North::Up,
            });
        }
        if state_id == 4928 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#east: East::None,
                r#power: 12,
                r#north: North::Side,
            });
        }
        if state_id == 4174 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 8,
            });
        }
        if state_id == 4292 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4757 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::None,
                r#power: 9,
            });
        }
        if state_id == 4082 {
            return Some(RedstoneWire {
                r#power: 14,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4258 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 1,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4697 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 2,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4143 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 5,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4283 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 4,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4510 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#power: 13,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4657 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 14,
            });
        }
        if state_id == 5082 {
            return Some(RedstoneWire {
                r#power: 13,
                r#north: North::None,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4621 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#power: 10,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4481 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 10,
            });
        }
        if state_id == 4400 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 1,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4501 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 12,
                r#west: West::Side,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 4511 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4239 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::None,
            });
        }
        if state_id == 4694 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#power: 2,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4893 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 8,
                r#south: South::Side,
            });
        }
        if state_id == 4849 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 3928 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#power: 13,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4040 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 9,
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4961 {
            return Some(RedstoneWire {
                r#power: 15,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4780 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 11,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 3821 {
            return Some(RedstoneWire {
                r#power: 1,
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4094 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 15,
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4984 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Side,
                r#power: 2,
            });
        }
        if state_id == 4466 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 8,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4458 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 8,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4003 {
            return Some(RedstoneWire {
                r#power: 5,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 3993 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 4,
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4704 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 3,
            });
        }
        if state_id == 4993 {
            return Some(RedstoneWire {
                r#power: 3,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 5004 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#power: 4,
            });
        }
        if state_id == 4186 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 9,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4079 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#power: 13,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4139 {
            return Some(RedstoneWire {
                r#power: 4,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4536 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::None,
                r#power: 0,
            });
        }
        if state_id == 4810 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 15,
                r#east: East::None,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 3995 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4216 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#south: South::Up,
                r#power: 13,
                r#east: East::Up,
            });
        }
        if state_id == 3949 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 15,
            });
        }
        if state_id == 4243 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#power: 0,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4319 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::None,
                r#power: 8,
                r#north: North::Up,
            });
        }
        if state_id == 5086 {
            return Some(RedstoneWire {
                r#power: 13,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 5101 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 15,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 3844 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#power: 3,
                r#north: North::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3854 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::None,
                r#power: 4,
            });
        }
        if state_id == 4101 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 0,
                r#north: North::None,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4148 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#power: 5,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4394 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 0,
            });
        }
        if state_id == 4766 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::None,
                r#power: 10,
            });
        }
        if state_id == 4752 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::Up,
                r#power: 8,
                r#south: South::None,
            });
        }
        if state_id == 4417 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 3,
            });
        }
        if state_id == 4310 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Side,
                r#power: 7,
            });
        }
        if state_id == 4768 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::None,
                r#power: 10,
            });
        }
        if state_id == 4815 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#power: 15,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 4846 {
            return Some(RedstoneWire {
                r#power: 3,
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4392 {
            return Some(RedstoneWire {
                r#power: 0,
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4224 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 14,
                r#west: West::Up,
                r#north: North::None,
                r#south: South::Up,
            });
        }
        if state_id == 4863 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 5,
            });
        }
        if state_id == 4901 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 9,
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::None,
            });
        }
        if state_id == 4938 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 13,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4108 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 1,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4156 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Side,
                r#power: 6,
                r#south: South::Side,
            });
        }
        if state_id == 4608 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Up,
                r#power: 8,
                r#north: North::None,
            });
        }
        if state_id == 4931 {
            return Some(RedstoneWire {
                r#power: 12,
                r#south: South::Side,
                r#east: East::None,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4588 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 6,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 4589 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Side,
                r#power: 6,
            });
        }
        if state_id == 4788 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#power: 12,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 4927 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 12,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4653 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 13,
            });
        }
        if state_id == 4573 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4706 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 3,
                r#north: North::Up,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4900 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4969 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#power: 0,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4712 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#east: East::None,
                r#power: 4,
                r#south: South::Up,
            });
        }
        if state_id == 4500 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 12,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4660 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::None,
                r#power: 14,
                r#south: South::Side,
            });
        }
        if state_id == 4291 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 5,
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 5019 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#power: 6,
                r#east: East::None,
                r#west: West::Up,
            });
        }
        if state_id == 3940 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 14,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4435 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 5,
            });
        }
        if state_id == 4412 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#power: 2,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4210 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 12,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4425 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 4,
                r#east: East::Side,
            });
        }
        if state_id == 4453 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 7,
            });
        }
        if state_id == 4650 {
            return Some(RedstoneWire {
                r#power: 13,
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 5021 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#power: 6,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 5031 {
            return Some(RedstoneWire {
                r#power: 7,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Up,
                r#south: South::None,
            });
        }
        if state_id == 3810 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#power: 0,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 3990 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 4,
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4359 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#power: 13,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4619 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#north: North::None,
                r#power: 9,
                r#west: West::None,
            });
        }
        if state_id == 4322 {
            return Some(RedstoneWire {
                r#power: 8,
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 5017 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::None,
                r#power: 6,
            });
        }
        if state_id == 4265 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
                r#power: 2,
            });
        }
        if state_id == 4617 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::None,
            });
        }
        if state_id == 4574 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#power: 4,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4652 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#power: 13,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4821 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
                r#power: 0,
                r#south: South::Side,
            });
        }
        if state_id == 4471 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 9,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4991 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4041 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Up,
                r#power: 9,
            });
        }
        if state_id == 4489 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 11,
            });
        }
        if state_id == 4358 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::None,
                r#power: 12,
            });
        }
        if state_id == 4018 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::Up,
                r#power: 7,
            });
        }
        if state_id == 4287 {
            return Some(RedstoneWire {
                r#power: 5,
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3902 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Up,
                r#power: 10,
            });
        }
        if state_id == 4036 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 9,
                r#east: East::Up,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4332 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 10,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4693 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Up,
                r#power: 2,
            });
        }
        if state_id == 4695 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 2,
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4816 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::None,
                r#west: West::Side,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 4828 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 1,
                r#south: South::Up,
            });
        }
        if state_id == 4415 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
                r#power: 3,
            });
        }
        if state_id == 4877 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::None,
                r#power: 6,
                r#west: West::None,
            });
        }
        if state_id == 4924 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::None,
                r#power: 11,
            });
        }
        if state_id == 4641 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 12,
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4881 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 7,
                r#east: East::None,
            });
        }
        if state_id == 4985 {
            return Some(RedstoneWire {
                r#power: 2,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4997 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 3,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 5035 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::None,
                r#power: 8,
            });
        }
        if state_id == 4072 {
            return Some(RedstoneWire {
                r#power: 13,
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4519 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 14,
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4405 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 2,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 5083 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 13,
                r#east: East::None,
            });
        }
        if state_id == 4230 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 14,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 5063 {
            return Some(RedstoneWire {
                r#power: 11,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 4732 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 6,
                r#east: East::None,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4857 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#power: 4,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4745 {
            return Some(RedstoneWire {
                r#power: 7,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4805 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#power: 14,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4942 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 13,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4334 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 10,
                r#west: West::None,
            });
        }
        if state_id == 3900 {
            return Some(RedstoneWire {
                r#power: 10,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3977 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::Side,
                r#power: 2,
            });
        }
        if state_id == 4892 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#power: 8,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4015 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 6,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4464 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 8,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4798 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Side,
                r#power: 13,
            });
        }
        if state_id == 4986 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::None,
                r#power: 2,
            });
        }
        if state_id == 4765 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 10,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 5005 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#south: South::None,
                r#power: 4,
                r#north: North::None,
            });
        }
        if state_id == 4105 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
                r#power: 0,
            });
        }
        if state_id == 4946 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#power: 14,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4312 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 7,
            });
        }
        if state_id == 4468 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 9,
            });
        }
        if state_id == 4057 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Side,
                r#power: 11,
                r#west: West::Side,
            });
        }
        if state_id == 4418 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 3,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4750 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::Side,
                r#power: 8,
                r#east: East::None,
            });
        }
        if state_id == 4953 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#power: 15,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4692 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#east: East::None,
                r#power: 2,
                r#west: West::Up,
            });
        }
        if state_id == 3874 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#power: 7,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4579 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#power: 5,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4535 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::None,
                r#power: 0,
            });
        }
        if state_id == 4793 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#power: 13,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 5072 {
            return Some(RedstoneWire {
                r#power: 12,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 5100 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Up,
                r#power: 15,
                r#south: South::Side,
            });
        }
        if state_id == 5045 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#east: East::None,
                r#power: 9,
                r#west: West::None,
            });
        }
        if state_id == 4933 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#south: South::None,
                r#power: 12,
                r#west: West::Side,
            });
        }
        if state_id == 5029 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#east: East::None,
                r#power: 7,
                r#south: South::Side,
            });
        }
        if state_id == 4661 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 14,
            });
        }
        if state_id == 5016 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Up,
                r#power: 6,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 5007 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::None,
                r#north: North::None,
                r#power: 5,
            });
        }
        if state_id == 4561 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 3,
            });
        }
        if state_id == 4718 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#power: 4,
            });
        }
        if state_id == 4853 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#power: 3,
                r#north: North::Side,
            });
        }
        if state_id == 3850 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 4,
            });
        }
        if state_id == 4034 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#power: 8,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 3887 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4968 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 0,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4241 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::None,
                r#power: 15,
                r#south: South::None,
            });
        }
        if state_id == 3899 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Up,
                r#west: West::None,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4357 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 12,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4398 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 1,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4764 {
            return Some(RedstoneWire {
                r#power: 10,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 3907 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Up,
                r#power: 10,
                r#west: West::Side,
            });
        }
        if state_id == 4404 {
            return Some(RedstoneWire {
                r#power: 2,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4872 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 6,
                r#south: South::Up,
            });
        }
        if state_id == 5026 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Side,
                r#power: 7,
                r#south: South::Up,
            });
        }
        if state_id == 5104 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#north: North::None,
                r#power: 15,
                r#east: East::None,
            });
        }
        if state_id == 4532 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 0,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4605 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::None,
                r#power: 8,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4142 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#west: West::None,
                r#power: 4,
                r#north: North::None,
            });
        }
        if state_id == 3969 {
            return Some(RedstoneWire {
                r#power: 1,
                r#south: South::None,
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4416 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4626 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#power: 10,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4002 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#power: 5,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4103 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 0,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4199 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Up,
                r#power: 11,
                r#south: South::Up,
            });
        }
        if state_id == 4253 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#south: South::Up,
                r#power: 1,
                r#east: East::Side,
            });
        }
        if state_id == 3839 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#south: South::Up,
                r#power: 3,
                r#north: North::Up,
            });
        }
        if state_id == 3932 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#power: 13,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4193 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 10,
                r#south: South::Side,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 4744 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 7,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4155 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::Side,
                r#north: North::None,
                r#power: 6,
            });
        }
        if state_id == 5020 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#east: East::None,
                r#power: 6,
                r#west: West::Side,
            });
        }
        if state_id == 4217 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#power: 13,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4569 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 4,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4758 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 9,
                r#east: East::None,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 3830 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::None,
                r#power: 2,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3951 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 15,
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4761 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 9,
                r#south: South::None,
            });
        }
        if state_id == 4327 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 9,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::Side,
            });
        }
        if state_id == 3968 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#power: 1,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4278 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Up,
                r#power: 4,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4298 {
            return Some(RedstoneWire {
                r#power: 6,
                r#west: West::None,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4402 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#power: 1,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 4583 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4923 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#power: 11,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 3957 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::Side,
                r#power: 0,
            });
        }
        if state_id == 4721 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Up,
                r#north: North::Up,
                r#power: 5,
            });
        }
        if state_id == 4728 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#power: 6,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 4032 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 8,
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4029 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 8,
            });
        }
        if state_id == 4423 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4867 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::None,
                r#power: 5,
            });
        }
        if state_id == 3859 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 5,
                r#south: South::Side,
            });
        }
        if state_id == 4790 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#east: East::None,
                r#power: 12,
                r#south: South::None,
            });
        }
        if state_id == 4089 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#power: 15,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 3974 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Up,
                r#power: 2,
                r#south: South::Up,
            });
        }
        if state_id == 4296 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 6,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4424 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#power: 4,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4777 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::None,
            });
        }
        if state_id == 3840 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Up,
                r#power: 3,
            });
        }
        if state_id == 4126 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 3,
                r#north: North::None,
            });
        }
        if state_id == 4502 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Side,
                r#power: 12,
                r#north: North::Side,
            });
        }
        if state_id == 4909 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#power: 10,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4716 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 4,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4980 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
                r#power: 2,
            });
        }
        if state_id == 3910 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 11,
                r#east: East::Up,
            });
        }
        if state_id == 3980 {
            return Some(RedstoneWire {
                r#power: 2,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4261 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::Side,
                r#power: 2,
                r#west: West::Side,
            });
        }
        if state_id == 4407 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#power: 2,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4539 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
                r#power: 1,
                r#west: West::Up,
            });
        }
        if state_id == 4917 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#west: West::Up,
                r#power: 11,
                r#south: South::Up,
            });
        }
        if state_id == 4934 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4088 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 14,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4922 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::None,
                r#power: 11,
            });
        }
        if state_id == 4335 {
            return Some(RedstoneWire {
                r#power: 10,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 3827 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 1,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4436 {
            return Some(RedstoneWire {
                r#power: 5,
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4340 {
            return Some(RedstoneWire {
                r#power: 10,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4522 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Up,
                r#power: 15,
                r#west: West::Side,
            });
        }
        if state_id == 5018 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4525 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::Side,
                r#power: 15,
                r#north: North::Side,
            });
        }
        if state_id == 4190 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#west: West::None,
                r#power: 10,
                r#east: East::Up,
            });
        }
        if state_id == 3838 {
            return Some(RedstoneWire {
                r#power: 3,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4293 {
            return Some(RedstoneWire {
                r#power: 5,
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4572 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 4,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 3913 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::Up,
                r#east: East::Up,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 3847 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 4,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4399 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 1,
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4851 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Side,
                r#south: South::None,
                r#power: 3,
            });
        }
        if state_id == 4507 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#power: 13,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 3919 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 12,
                r#south: South::Up,
            });
        }
        if state_id == 4625 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 10,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4741 {
            return Some(RedstoneWire {
                r#power: 7,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4086 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 14,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4188 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 10,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4249 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 0,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Side,
            });
        }
        if state_id == 4472 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
                r#power: 9,
            });
        }
        if state_id == 4680 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 0,
            });
        }
        if state_id == 4168 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#power: 7,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 3897 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 9,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 3923 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 12,
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4318 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#power: 8,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4087 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#power: 14,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4385 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Up,
                r#power: 15,
                r#east: East::Side,
            });
        }
        if state_id == 4460 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 8,
                r#north: North::Side,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4562 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#power: 3,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4622 {
            return Some(RedstoneWire {
                r#power: 10,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4978 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Side,
                r#power: 1,
            });
        }
        if state_id == 4028 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Side,
                r#power: 8,
                r#west: West::None,
            });
        }
        if state_id == 3967 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 1,
            });
        }
        if state_id == 4648 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::None,
                r#power: 13,
            });
        }
        if state_id == 4288 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 5,
            });
        }
        if state_id == 3970 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#power: 1,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3939 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 14,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4176 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::None,
                r#east: East::Up,
                r#power: 8,
                r#south: South::None,
            });
        }
        if state_id == 4437 {
            return Some(RedstoneWire {
                r#power: 5,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4972 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Side,
                r#south: South::Up,
                r#power: 1,
            });
        }
        if state_id == 4090 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Side,
                r#power: 15,
                r#south: South::Up,
            });
        }
        if state_id == 4838 {
            return Some(RedstoneWire {
                r#power: 2,
                r#west: West::None,
                r#south: South::Up,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4691 {
            return Some(RedstoneWire {
                r#power: 1,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 5014 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#power: 5,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 5030 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#west: West::None,
                r#power: 7,
                r#north: North::None,
            });
        }
        if state_id == 4141 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::None,
                r#power: 4,
            });
        }
        if state_id == 3882 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Up,
                r#power: 8,
                r#south: South::Up,
            });
        }
        if state_id == 5051 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 9,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4189 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::None,
                r#power: 10,
                r#west: West::Side,
            });
        }
        if state_id == 4726 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 5,
                r#south: South::None,
            });
        }
        if state_id == 4232 {
            return Some(RedstoneWire {
                r#power: 14,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4882 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 7,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4337 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 10,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4498 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#power: 12,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4962 {
            return Some(RedstoneWire {
                r#power: 0,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::None,
            });
        }
        if state_id == 4613 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 9,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
            });
        }
        if state_id == 3953 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#power: 15,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4284 {
            return Some(RedstoneWire {
                r#power: 4,
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4786 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#power: 12,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 3865 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 6,
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4403 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 1,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 3946 {
            return Some(RedstoneWire {
                r#power: 15,
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4509 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 13,
                r#south: South::None,
            });
        }
        if state_id == 4096 {
            return Some(RedstoneWire {
                r#power: 15,
                r#south: South::None,
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4125 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Up,
                r#power: 3,
                r#east: East::Up,
            });
        }
        if state_id == 4274 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 3,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4441 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Side,
                r#power: 6,
            });
        }
        if state_id == 4533 {
            return Some(RedstoneWire {
                r#power: 0,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
            });
        }
        if state_id == 4558 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::None,
                r#power: 3,
                r#south: South::Up,
            });
        }
        if state_id == 4630 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 11,
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4746 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#power: 8,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4670 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#north: North::None,
                r#power: 15,
                r#south: South::Side,
            });
        }
        if state_id == 4886 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 7,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 5096 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#power: 14,
                r#east: East::None,
            });
        }
        if state_id == 3999 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 5,
                r#south: South::Up,
            });
        }
        if state_id == 4615 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 9,
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 5038 {
            return Some(RedstoneWire {
                r#power: 8,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4654 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::None,
                r#power: 13,
            });
        }
        if state_id == 5087 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#power: 13,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4281 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 4,
            });
        }
        if state_id == 4703 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 3,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4157 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 6,
                r#south: South::Side,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4023 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#power: 7,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4180 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::None,
                r#power: 9,
                r#south: South::Up,
            });
        }
        if state_id == 4290 {
            return Some(RedstoneWire {
                r#power: 5,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4301 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#power: 6,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4776 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 11,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4967 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 0,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 3895 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 9,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4192 {
            return Some(RedstoneWire {
                r#power: 10,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4543 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 1,
            });
        }
        if state_id == 3811 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 0,
                r#east: East::Up,
            });
        }
        if state_id == 4593 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#power: 7,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4578 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 5,
                r#west: West::Up,
            });
        }
        if state_id == 4601 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
                r#power: 7,
                r#east: East::Side,
            });
        }
        if state_id == 4440 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::Side,
                r#power: 6,
                r#west: West::Up,
            });
        }
        if state_id == 4544 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::Side,
                r#power: 1,
            });
        }
        if state_id == 3992 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::None,
                r#power: 4,
            });
        }
        if state_id == 4859 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Side,
                r#power: 4,
            });
        }
        if state_id == 4773 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::Up,
            });
        }
        if state_id == 3947 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#power: 15,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4446 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::Side,
                r#power: 6,
                r#west: West::Up,
            });
        }
        if state_id == 4077 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 13,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4078 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 4925 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Side,
                r#power: 11,
            });
        }
        if state_id == 4347 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 11,
            });
        }
        if state_id == 4459 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 8,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4770 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::None,
                r#power: 10,
                r#south: South::None,
            });
        }
        if state_id == 4715 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Side,
                r#north: North::Up,
                r#power: 4,
            });
        }
        if state_id == 3959 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 0,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 5040 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#east: East::None,
                r#power: 8,
                r#north: North::None,
            });
        }
        if state_id == 5054 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::None,
                r#power: 10,
            });
        }
        if state_id == 4939 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::None,
                r#power: 13,
                r#south: South::Side,
            });
        }
        if state_id == 4618 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 9,
                r#west: West::Side,
                r#south: South::None,
                r#east: East::Side,
            });
        }
        if state_id == 4331 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#north: North::Up,
                r#power: 9,
                r#south: South::None,
            });
        }
        if state_id == 4811 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 15,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 3851 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 4,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4645 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#power: 12,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4448 {
            return Some(RedstoneWire {
                r#power: 6,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 3944 {
            return Some(RedstoneWire {
                r#power: 14,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Up,
            });
        }
        if state_id == 3976 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 2,
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4829 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 1,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 3945 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Up,
                r#power: 15,
            });
        }
        if state_id == 4478 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#south: South::Up,
                r#east: East::Side,
                r#power: 10,
            });
        }
        if state_id == 4888 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 7,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4289 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 5,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4634 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Side,
                r#power: 11,
            });
        }
        if state_id == 4941 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::None,
                r#power: 13,
            });
        }
        if state_id == 4128 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::Up,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4482 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Up,
                r#power: 10,
                r#north: North::Side,
            });
        }
        if state_id == 5023 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#power: 6,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 3813 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 0,
            });
        }
        if state_id == 3884 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::None,
                r#north: North::Up,
                r#power: 8,
            });
        }
        if state_id == 4117 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 2,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 3898 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 9,
            });
        }
        if state_id == 4144 {
            return Some(RedstoneWire {
                r#power: 5,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3860 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 5,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4401 {
            return Some(RedstoneWire {
                r#power: 1,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4668 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#east: East::Side,
                r#power: 15,
                r#west: West::Up,
            });
        }
        if state_id == 4813 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4689 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 1,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 4518 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::None,
                r#power: 14,
            });
        }
        if state_id == 4069 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 12,
            });
        }
        if state_id == 4494 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 12,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4643 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 12,
                r#north: North::None,
            });
        }
        if state_id == 4719 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 5,
            });
        }
        if state_id == 4493 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::Side,
                r#power: 11,
                r#west: West::None,
            });
        }
        if state_id == 4919 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 11,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4039 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Side,
                r#power: 9,
                r#west: West::Side,
            });
        }
        if state_id == 4546 {
            return Some(RedstoneWire {
                r#power: 1,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4097 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 15,
                r#south: South::None,
            });
        }
        if state_id == 4085 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 14,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4104 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 0,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4138 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4469 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#power: 9,
                r#north: North::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4802 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#power: 14,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4705 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 3,
                r#east: East::None,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4009 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Side,
                r#power: 6,
            });
        }
        if state_id == 4026 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#power: 8,
            });
        }
        if state_id == 4824 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Side,
                r#power: 0,
                r#west: West::Up,
            });
        }
        if state_id == 4218 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 5055 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
                r#power: 10,
                r#south: South::Side,
            });
        }
        if state_id == 4354 {
            return Some(RedstoneWire {
                r#power: 12,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4449 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 7,
                r#east: East::Side,
            });
        }
        if state_id == 4311 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 7,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 5048 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#power: 9,
            });
        }
        if state_id == 4638 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 12,
                r#east: East::Side,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4910 {
            return Some(RedstoneWire {
                r#power: 10,
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4137 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::Up,
                r#power: 4,
            });
        }
        if state_id == 4671 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 15,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 3818 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Up,
                r#power: 0,
            });
        }
        if state_id == 5037 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#power: 8,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4179 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#power: 9,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4760 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 9,
                r#west: West::None,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4107 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 1,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4495 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 12,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4807 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 14,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 4306 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#power: 7,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4707 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::None,
                r#north: North::Up,
                r#power: 3,
                r#east: East::None,
            });
        }
        if state_id == 4611 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 9,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4964 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#power: 0,
                r#south: South::Up,
            });
        }
        if state_id == 3991 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
                r#power: 4,
                r#west: West::Side,
            });
        }
        if state_id == 4081 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 14,
                r#east: East::Up,
                r#west: West::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4149 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#south: South::None,
                r#power: 5,
                r#east: East::Up,
            });
        }
        if state_id == 4181 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 9,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Up,
            });
        }
        if state_id == 4450 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Side,
                r#power: 7,
            });
        }
        if state_id == 5047 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Side,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 4012 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#power: 6,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4375 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Side,
                r#power: 14,
            });
        }
        if state_id == 5079 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4860 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::None,
                r#power: 4,
            });
        }
        if state_id == 4303 {
            return Some(RedstoneWire {
                r#power: 6,
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4172 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 4497 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Side,
                r#power: 12,
                r#west: West::Up,
            });
        }
        if state_id == 5025 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::None,
                r#power: 7,
            });
        }
        if state_id == 4051 {
            return Some(RedstoneWire {
                r#power: 10,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 3872 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#south: South::None,
                r#north: North::Up,
                r#power: 6,
            });
        }
        if state_id == 3960 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::Up,
                r#power: 0,
            });
        }
        if state_id == 4600 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::None,
                r#power: 7,
            });
        }
        if state_id == 3985 {
            return Some(RedstoneWire {
                r#power: 3,
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4763 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#power: 9,
                r#north: North::Up,
            });
        }
        if state_id == 4052 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Side,
                r#power: 10,
            });
        }
        if state_id == 4644 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Up,
                r#power: 12,
                r#north: North::None,
            });
        }
        if state_id == 4048 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 10,
                r#south: South::Side,
            });
        }
        if state_id == 4499 {
            return Some(RedstoneWire {
                r#power: 12,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4649 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Side,
                r#power: 13,
                r#south: South::Up,
            });
        }
        if state_id == 3997 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::None,
                r#power: 4,
            });
        }
        if state_id == 4175 {
            return Some(RedstoneWire {
                r#power: 8,
                r#west: West::None,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4921 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#power: 11,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4970 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#power: 0,
                r#south: South::None,
            });
        }
        if state_id == 5102 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#power: 15,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4875 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 6,
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4240 {
            return Some(RedstoneWire {
                r#power: 15,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 3817 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::None,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4797 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 13,
            });
        }
        if state_id == 3864 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Up,
                r#power: 6,
            });
        }
        if state_id == 4889 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#west: West::None,
                r#power: 7,
                r#east: East::None,
            });
        }
        if state_id == 4328 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 9,
                r#west: West::None,
            });
        }
        if state_id == 4717 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#power: 4,
                r#north: North::Up,
                r#east: East::None,
            });
        }
        if state_id == 4800 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 14,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 5070 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 12,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 3943 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 14,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4523 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 15,
                r#south: South::Up,
            });
        }
        if state_id == 4161 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Up,
                r#power: 7,
                r#east: East::Up,
            });
        }
        if state_id == 3836 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::None,
                r#power: 2,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 3925 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 12,
                r#east: East::Up,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 3942 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 14,
                r#south: South::None,
            });
        }
        if state_id == 4251 {
            return Some(RedstoneWire {
                r#power: 1,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4852 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#east: East::None,
                r#power: 3,
                r#north: North::Side,
            });
        }
        if state_id == 5069 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#power: 11,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4554 {
            return Some(RedstoneWire {
                r#power: 2,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4376 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 14,
            });
        }
        if state_id == 4503 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#power: 13,
                r#south: South::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4336 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 10,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4936 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#north: North::Side,
                r#power: 13,
                r#west: West::Side,
            });
        }
        if state_id == 5090 {
            return Some(RedstoneWire {
                r#power: 14,
                r#south: South::Up,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4183 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 9,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 5105 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 15,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 3981 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 3,
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4360 {
            return Some(RedstoneWire {
                r#power: 13,
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4606 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4118 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#power: 2,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4696 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 2,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4847 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 4935 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 13,
                r#south: South::Up,
            });
        }
        if state_id == 4612 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#power: 9,
                r#north: North::None,
                r#east: East::Side,
            });
        }
        if state_id == 5095 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#power: 14,
                r#west: West::Side,
            });
        }
        if state_id == 4708 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::None,
                r#power: 3,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 4047 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Up,
                r#power: 10,
            });
        }
        if state_id == 4208 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 12,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4871 {
            return Some(RedstoneWire {
                r#power: 5,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 5032 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::None,
                r#power: 7,
            });
        }
        if state_id == 4827 {
            return Some(RedstoneWire {
                r#power: 1,
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 3848 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 4,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4182 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4214 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::None,
                r#power: 12,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4736 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 6,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 4826 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Side,
                r#power: 0,
                r#south: South::None,
            });
        }
        if state_id == 3979 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Side,
                r#power: 2,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4443 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 6,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 5009 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#power: 5,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4136 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::None,
                r#power: 4,
            });
        }
        if state_id == 4270 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#power: 3,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4937 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4538 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#south: South::None,
                r#power: 0,
                r#west: West::None,
            });
        }
        if state_id == 3958 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Side,
                r#east: East::Up,
                r#power: 0,
                r#north: North::Side,
            });
        }
        if state_id == 3869 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Up,
                r#power: 6,
                r#west: West::None,
            });
        }
        if state_id == 4206 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::Up,
                r#power: 12,
            });
        }
        if state_id == 4753 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4409 {
            return Some(RedstoneWire {
                r#power: 2,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4127 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 3,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4068 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#power: 12,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4965 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 0,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
            });
        }
        if state_id == 4673 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
                r#power: 15,
                r#east: East::Side,
            });
        }
        if state_id == 3841 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4374 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Side,
                r#power: 14,
                r#north: North::Up,
            });
        }
        if state_id == 4784 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 12,
                r#south: South::Up,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 4804 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 14,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 5036 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#power: 8,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4313 {
            return Some(RedstoneWire {
                r#power: 7,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4043 {
            return Some(RedstoneWire {
                r#power: 9,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 3920 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Up,
                r#power: 12,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4211 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 12,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4010 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#power: 6,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4226 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 14,
                r#east: East::Up,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4663 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 14,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4584 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::Side,
                r#power: 6,
            });
        }
        if state_id == 4656 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::None,
                r#power: 14,
            });
        }
        if state_id == 4796 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#power: 13,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 4808 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#east: East::None,
                r#west: West::None,
                r#power: 14,
            });
        }
        if state_id == 4843 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 2,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 5044 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Up,
                r#east: East::None,
                r#power: 9,
                r#north: North::None,
            });
        }
        if state_id == 4999 {
            return Some(RedstoneWire {
                r#power: 4,
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 4711 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::None,
                r#power: 4,
            });
        }
        if state_id == 4966 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 0,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4725 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 5,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 3873 {
            return Some(RedstoneWire {
                r#power: 7,
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4491 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#power: 11,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4277 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4831 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 1,
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4410 {
            return Some(RedstoneWire {
                r#power: 2,
                r#west: West::Up,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4473 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::Up,
                r#power: 9,
            });
        }
        if state_id == 4433 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 5,
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4487 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
                r#power: 11,
            });
        }
        if state_id == 4541 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#power: 1,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 4647 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 4841 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#east: East::None,
                r#west: West::None,
                r#power: 2,
            });
        }
        if state_id == 4064 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#power: 12,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4591 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#west: West::Side,
                r#south: South::None,
                r#power: 6,
            });
        }
        if state_id == 4944 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 14,
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 3861 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::None,
                r#power: 5,
            });
        }
        if state_id == 4897 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::None,
                r#power: 8,
                r#west: West::Side,
            });
        }
        if state_id == 4184 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::None,
                r#power: 9,
            });
        }
        if state_id == 4229 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#north: North::None,
                r#power: 14,
                r#east: East::Up,
            });
        }
        if state_id == 4326 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#power: 9,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4119 {
            return Some(RedstoneWire {
                r#power: 2,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4275 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 3,
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4163 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 7,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Up,
            });
        }
        if state_id == 4592 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Side,
                r#west: West::None,
            });
        }
        if state_id == 4659 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 14,
                r#east: East::Side,
            });
        }
        if state_id == 4461 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 8,
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4699 {
            return Some(RedstoneWire {
                r#power: 2,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4054 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Side,
                r#power: 11,
                r#east: East::Up,
            });
        }
        if state_id == 3934 {
            return Some(RedstoneWire {
                r#power: 13,
                r#south: South::None,
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4339 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::None,
                r#west: West::Side,
                r#power: 10,
            });
        }
        if state_id == 4932 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 12,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4724 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 3933 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::Up,
                r#south: South::None,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4173 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::Up,
                r#power: 8,
            });
        }
        if state_id == 4894 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Side,
                r#power: 8,
                r#south: South::Side,
            });
        }
        if state_id == 4488 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4373 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 14,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4531 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#power: 0,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4364 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 13,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4092 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 15,
                r#north: North::Side,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4121 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#west: West::None,
                r#power: 2,
                r#east: East::Up,
            });
        }
        if state_id == 4395 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#power: 1,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4294 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#power: 5,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4874 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4566 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Up,
                r#power: 4,
            });
        }
        if state_id == 4070 {
            return Some(RedstoneWire {
                r#power: 12,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4113 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 1,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4837 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Up,
                r#north: North::Side,
                r#power: 2,
            });
        }
        if state_id == 4438 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#power: 5,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4534 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#power: 0,
                r#north: North::None,
                r#south: South::Side,
            });
        }
        if state_id == 4426 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::Side,
                r#power: 4,
            });
        }
        if state_id == 3937 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Side,
                r#power: 14,
            });
        }
        if state_id == 4551 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Side,
                r#north: North::None,
                r#east: East::Side,
                r#power: 2,
            });
        }
        if state_id == 4952 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::None,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4021 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Side,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 7,
            });
        }
        if state_id == 4789 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#power: 12,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 4973 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 1,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 3903 {
            return Some(RedstoneWire {
                r#power: 10,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4063 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 12,
            });
        }
        if state_id == 3853 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::None,
                r#power: 4,
            });
        }
        if state_id == 4508 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::None,
                r#power: 13,
                r#north: North::Side,
            });
        }
        if state_id == 4814 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#power: 15,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 5012 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 5,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 5050 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 3955 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 0,
                r#west: West::Side,
            });
        }
        if state_id == 4662 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 14,
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4454 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#south: South::Side,
                r#east: East::Side,
                r#power: 7,
            });
        }
        if state_id == 3948 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4787 {
            return Some(RedstoneWire {
                r#power: 12,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 5015 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#power: 5,
            });
        }
        if state_id == 4044 {
            return Some(RedstoneWire {
                r#power: 10,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3914 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 4065 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::Side,
                r#power: 12,
                r#west: West::Up,
            });
        }
        if state_id == 3912 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::Side,
                r#power: 11,
            });
        }
        if state_id == 4628 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 10,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4236 {
            return Some(RedstoneWire {
                r#power: 15,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4038 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 9,
                r#south: South::Side,
            });
        }
        if state_id == 4835 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Side,
                r#power: 1,
            });
        }
        if state_id == 4988 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#power: 2,
                r#west: West::None,
            });
        }
        if state_id == 3832 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 2,
                r#south: South::Side,
            });
        }
        if state_id == 4918 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Side,
                r#power: 11,
                r#east: East::None,
            });
        }
        if state_id == 4153 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Up,
                r#power: 6,
            });
        }
        if state_id == 5058 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Up,
                r#power: 10,
            });
        }
        if state_id == 4056 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4123 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 2,
                r#north: North::None,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4341 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4396 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
                r#power: 1,
            });
        }
        if state_id == 4603 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::None,
                r#power: 8,
            });
        }
        if state_id == 4698 {
            return Some(RedstoneWire {
                r#power: 2,
                r#north: North::Up,
                r#east: East::None,
                r#west: West::Up,
                r#south: South::None,
            });
        }
        if state_id == 4887 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Up,
                r#south: South::None,
                r#power: 7,
            });
        }
        if state_id == 4957 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#power: 15,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4307 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 7,
                r#west: West::None,
                r#south: South::Up,
                r#east: East::Side,
            });
        }
        if state_id == 3908 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 10,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Up,
            });
        }
        if state_id == 4140 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#power: 4,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 4222 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::None,
                r#east: East::Up,
                r#north: North::None,
                r#power: 13,
            });
        }
        if state_id == 4581 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#north: North::None,
                r#power: 5,
                r#east: East::Side,
            });
        }
        if state_id == 4099 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 0,
                r#west: West::Side,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4794 {
            return Some(RedstoneWire {
                r#power: 13,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4001 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 5,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4031 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 8,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4246 {
            return Some(RedstoneWire {
                r#power: 0,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 5097 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::None,
                r#east: East::None,
                r#power: 15,
            });
        }
        if state_id == 3901 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Up,
                r#power: 10,
                r#west: West::Side,
            });
        }
        if state_id == 4665 {
            return Some(RedstoneWire {
                r#power: 15,
                r#north: North::None,
                r#east: East::Side,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4262 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 2,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4607 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Side,
                r#power: 8,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4071 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 13,
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4490 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::None,
                r#north: North::Side,
                r#power: 11,
            });
        }
        if state_id == 4035 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#power: 9,
            });
        }
        if state_id == 4682 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#power: 0,
                r#west: West::None,
                r#north: North::Up,
            });
        }
        if state_id == 5002 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 4,
            });
        }
        if state_id == 4325 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Up,
                r#power: 9,
            });
        }
        if state_id == 3965 {
            return Some(RedstoneWire {
                r#power: 1,
                r#east: East::Up,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 3926 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 12,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4627 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#power: 10,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4616 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::None,
                r#power: 9,
            });
        }
        if state_id == 4223 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#power: 13,
                r#east: East::Up,
                r#west: West::None,
            });
        }
        if state_id == 4883 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#power: 7,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4185 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Up,
                r#south: South::None,
                r#power: 9,
                r#east: East::Up,
            });
        }
        if state_id == 4596 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
                r#power: 7,
            });
        }
        if state_id == 4227 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::Side,
                r#power: 14,
                r#north: North::None,
            });
        }
        if state_id == 4599 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#power: 7,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4146 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4381 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#power: 15,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4911 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
                r#power: 10,
            });
        }
        if state_id == 4506 {
            return Some(RedstoneWire {
                r#power: 13,
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4315 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#south: South::Up,
                r#power: 8,
            });
        }
        if state_id == 4348 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 11,
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4422 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Up,
                r#south: South::Up,
                r#power: 4,
            });
        }
        if state_id == 5092 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Side,
                r#power: 14,
                r#east: East::None,
            });
        }
        if state_id == 4445 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Side,
                r#power: 6,
            });
        }
        if state_id == 4344 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Side,
                r#power: 11,
            });
        }
        if state_id == 3978 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#power: 2,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4582 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 5,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4595 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Up,
                r#power: 7,
            });
        }
        if state_id == 4080 {
            return Some(RedstoneWire {
                r#power: 14,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4751 {
            return Some(RedstoneWire {
                r#power: 8,
                r#north: North::Up,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Side,
            });
        }
        if state_id == 4636 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4833 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::None,
                r#power: 1,
                r#west: West::Up,
            });
        }
        if state_id == 4367 {
            return Some(RedstoneWire {
                r#power: 13,
                r#west: West::None,
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 4300 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#power: 6,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4825 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#power: 0,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4992 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4234 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::None,
                r#power: 15,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 3982 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#power: 3,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4666 {
            return Some(RedstoneWire {
                r#power: 15,
                r#west: West::Side,
                r#north: North::None,
                r#south: South::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4646 {
            return Some(RedstoneWire {
                r#power: 12,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Side,
            });
        }
        if state_id == 3952 {
            return Some(RedstoneWire {
                r#power: 15,
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4324 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Up,
                r#west: West::Side,
                r#power: 9,
            });
        }
        if state_id == 3831 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 2,
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4948 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#power: 14,
                r#north: North::Side,
                r#south: South::Side,
            });
        }
        if state_id == 5080 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 13,
                r#north: North::None,
                r#west: West::Side,
                r#east: East::None,
            });
        }
        if state_id == 3837 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Up,
                r#power: 3,
            });
        }
        if state_id == 4568 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::Side,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4979 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 1,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 3929 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::None,
                r#south: South::Up,
                r#power: 13,
            });
        }
        if state_id == 4250 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#power: 0,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4609 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::None,
                r#power: 8,
                r#south: South::None,
            });
        }
        if state_id == 4109 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Up,
                r#power: 1,
                r#north: North::None,
            });
        }
        if state_id == 4254 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 1,
            });
        }
        if state_id == 5067 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#west: West::Up,
                r#power: 11,
                r#north: North::None,
            });
        }
        if state_id == 4576 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#power: 5,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 5000 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#power: 4,
                r#south: South::Up,
            });
        }
        if state_id == 4017 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Up,
                r#west: West::Up,
                r#power: 7,
            });
        }
        if state_id == 4929 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 12,
            });
        }
        if state_id == 5006 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::None,
                r#power: 4,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 4475 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Side,
                r#west: West::None,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4476 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#power: 10,
                r#south: South::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4484 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::None,
                r#south: South::None,
                r#power: 10,
            });
        }
        if state_id == 4066 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#power: 12,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4637 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#power: 11,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4681 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 0,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Up,
            });
        }
        if state_id == 5065 {
            return Some(RedstoneWire {
                r#power: 11,
                r#south: South::Side,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 4483 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 10,
                r#west: West::Side,
            });
        }
        if state_id == 4268 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 2,
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4387 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 0,
            });
        }
        if state_id == 4102 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::None,
                r#power: 0,
            });
        }
        if state_id == 4228 {
            return Some(RedstoneWire {
                r#power: 14,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4817 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 15,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 4058 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::None,
                r#power: 11,
            });
        }
        if state_id == 3890 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Up,
                r#power: 8,
            });
        }
        if state_id == 4351 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#power: 12,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 3846 {
            return Some(RedstoneWire {
                r#power: 4,
                r#north: North::Up,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3893 {
            return Some(RedstoneWire {
                r#power: 9,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4114 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 1,
                r#south: South::None,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3857 {
            return Some(RedstoneWire {
                r#power: 5,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4247 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 0,
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Up,
            });
        }
        if state_id == 4855 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Up,
                r#power: 4,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4685 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::None,
                r#power: 1,
            });
        }
        if state_id == 4710 {
            return Some(RedstoneWire {
                r#power: 4,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4727 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 5,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4678 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#power: 0,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4212 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Up,
                r#power: 12,
                r#north: North::None,
            });
        }
        if state_id == 4542 {
            return Some(RedstoneWire {
                r#power: 1,
                r#west: West::Up,
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::None,
            });
        }
        if state_id == 4756 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Up,
                r#north: North::Up,
                r#power: 9,
            });
        }
        if state_id == 4162 {
            return Some(RedstoneWire {
                r#power: 7,
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4462 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 8,
                r#east: East::Side,
            });
        }
        if state_id == 4521 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4914 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#power: 10,
                r#north: North::Side,
                r#south: South::None,
            });
        }
        if state_id == 4996 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#power: 3,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 4349 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 11,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 4033 {
            return Some(RedstoneWire {
                r#power: 8,
                r#south: South::None,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4895 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#power: 8,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4548 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::None,
                r#power: 2,
            });
        }
        if state_id == 4419 {
            return Some(RedstoneWire {
                r#power: 3,
                r#west: West::Up,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4560 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 3,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4785 {
            return Some(RedstoneWire {
                r#power: 12,
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 3891 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 9,
            });
        }
        if state_id == 4971 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
                r#power: 1,
                r#south: South::Up,
            });
        }
        if state_id == 3906 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Up,
                r#power: 10,
            });
        }
        if state_id == 4444 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 6,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4194 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::None,
                r#power: 10,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4479 {
            return Some(RedstoneWire {
                r#power: 10,
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4528 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#east: East::Side,
                r#power: 15,
                r#west: West::Side,
            });
        }
        if state_id == 4943 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 13,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 4115 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::None,
                r#power: 1,
                r#south: South::None,
            });
        }
        if state_id == 4020 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#power: 7,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4365 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#power: 13,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4733 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::Up,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4299 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#power: 6,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4145 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::None,
                r#east: East::Up,
                r#west: West::None,
                r#power: 5,
            });
        }
        if state_id == 4842 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#west: West::Up,
                r#power: 2,
                r#south: South::None,
            });
        }
        if state_id == 4884 {
            return Some(RedstoneWire {
                r#power: 7,
                r#south: South::Side,
                r#east: East::None,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4809 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 15,
                r#south: South::Up,
            });
        }
        if state_id == 4352 {
            return Some(RedstoneWire {
                r#power: 12,
                r#west: West::None,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4042 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::Side,
                r#power: 9,
            });
        }
        if state_id == 4505 {
            return Some(RedstoneWire {
                r#power: 13,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4202 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#power: 11,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4248 {
            return Some(RedstoneWire {
                r#power: 0,
                r#west: West::Up,
                r#south: South::None,
                r#north: North::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4177 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#west: West::Side,
                r#power: 8,
                r#east: East::Up,
            });
        }
        if state_id == 3983 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#power: 3,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4171 {
            return Some(RedstoneWire {
                r#power: 8,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4903 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Side,
                r#power: 9,
            });
        }
        if state_id == 4977 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 1,
                r#west: West::Up,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 3896 {
            return Some(RedstoneWire {
                r#power: 9,
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 3909 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::Up,
                r#power: 11,
            });
        }
        if state_id == 3956 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 0,
                r#south: South::Up,
            });
        }
        if state_id == 3975 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Side,
                r#power: 2,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4060 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 11,
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4245 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 0,
                r#south: South::Side,
            });
        }
        if state_id == 4317 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#power: 8,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4688 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::None,
                r#power: 1,
            });
        }
        if state_id == 4878 {
            return Some(RedstoneWire {
                r#power: 6,
                r#east: East::None,
                r#west: West::Up,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 5022 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#power: 6,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 5084 {
            return Some(RedstoneWire {
                r#power: 13,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 3954 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#west: West::Up,
                r#power: 0,
                r#south: South::Up,
            });
        }
        if state_id == 4320 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#power: 8,
                r#west: West::Up,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4755 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Up,
                r#east: East::None,
                r#north: North::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4955 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::None,
                r#power: 15,
            });
        }
        if state_id == 4951 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#power: 14,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 4467 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Up,
                r#north: North::Side,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4517 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Side,
                r#north: North::Side,
                r#power: 14,
            });
        }
        if state_id == 4552 {
            return Some(RedstoneWire {
                r#power: 2,
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 4803 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 14,
            });
        }
        if state_id == 4545 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Up,
                r#power: 1,
            });
        }
        if state_id == 3823 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::Side,
                r#power: 1,
            });
        }
        if state_id == 3875 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 7,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4550 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#power: 2,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4866 {
            return Some(RedstoneWire {
                r#power: 5,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 5060 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#power: 10,
                r#north: North::None,
            });
        }
        if state_id == 4309 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 7,
                r#east: East::Side,
            });
        }
        if state_id == 4414 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::Side,
                r#power: 3,
            });
        }
        if state_id == 4302 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 6,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 5078 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#power: 12,
            });
        }
        if state_id == 4209 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#power: 12,
                r#east: East::Up,
                r#north: North::None,
            });
        }
        if state_id == 4587 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::Side,
                r#west: West::Up,
            });
        }
        if state_id == 3816 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 0,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 5033 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#power: 7,
            });
        }
        if state_id == 4950 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Up,
                r#power: 14,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 4975 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 1,
                r#north: North::None,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 5075 {
            return Some(RedstoneWire {
                r#power: 12,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Side,
                r#north: North::None,
            });
        }
        if state_id == 4737 {
            return Some(RedstoneWire {
                r#power: 7,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4000 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4806 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Up,
                r#power: 14,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4949 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
                r#power: 14,
            });
        }
        if state_id == 5066 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#power: 11,
                r#south: South::Side,
            });
        }
        if state_id == 5001 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::None,
                r#power: 4,
                r#east: East::None,
                r#west: West::Up,
            });
        }
        if state_id == 4164 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#east: East::Up,
                r#west: West::Up,
                r#power: 7,
            });
        }
        if state_id == 4073 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 13,
            });
        }
        if state_id == 5056 {
            return Some(RedstoneWire {
                r#power: 10,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4890 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
                r#power: 8,
            });
        }
        if state_id == 5010 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Up,
                r#north: North::None,
                r#power: 5,
                r#south: South::Side,
            });
        }
        if state_id == 3863 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::None,
                r#power: 5,
                r#south: South::None,
            });
        }
        if state_id == 4590 {
            return Some(RedstoneWire {
                r#power: 6,
                r#north: North::None,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 3915 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#power: 11,
                r#east: East::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4074 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#power: 13,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4610 {
            return Some(RedstoneWire {
                r#power: 8,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4683 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Up,
                r#power: 1,
                r#east: East::None,
                r#north: North::Up,
            });
        }
        if state_id == 4740 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#power: 7,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 5052 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::Up,
                r#power: 10,
            });
        }
        if state_id == 4011 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 6,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4314 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::Up,
                r#power: 8,
                r#west: West::Up,
            });
        }
        if state_id == 4492 {
            return Some(RedstoneWire {
                r#power: 11,
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 4557 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::Side,
                r#power: 3,
                r#north: North::None,
            });
        }
        if state_id == 3886 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Up,
                r#south: South::Side,
                r#power: 8,
            });
        }
        if state_id == 4421 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#power: 3,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 3819 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#power: 1,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4046 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 10,
            });
        }
        if state_id == 4470 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 9,
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4122 {
            return Some(RedstoneWire {
                r#power: 2,
                r#south: South::None,
                r#east: East::Up,
                r#north: North::None,
                r#west: West::Up,
            });
        }
        if state_id == 3905 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 10,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 4674 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 0,
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4690 {
            return Some(RedstoneWire {
                r#power: 1,
                r#south: South::None,
                r#north: North::Up,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4093 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 15,
                r#south: South::Side,
            });
        }
        if state_id == 4700 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Up,
                r#east: East::None,
                r#power: 2,
            });
        }
        if state_id == 4906 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Side,
                r#south: South::None,
                r#east: East::None,
                r#power: 9,
            });
        }
        if state_id == 4990 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Up,
                r#power: 3,
            });
        }
        if state_id == 3941 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Up,
                r#power: 14,
                r#west: West::None,
                r#east: East::Up,
            });
        }
        if state_id == 5081 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 13,
                r#east: East::None,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4775 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Up,
                r#power: 11,
                r#north: North::Up,
                r#west: West::None,
            });
        }
        if state_id == 3820 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 1,
                r#east: East::Up,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3825 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::Up,
                r#power: 1,
                r#south: South::None,
            });
        }
        if state_id == 4345 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 11,
                r#east: East::Side,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4366 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Up,
                r#south: South::None,
                r#power: 13,
            });
        }
        if state_id == 3888 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#north: North::Up,
                r#power: 8,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4714 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 4,
                r#north: North::Up,
                r#south: South::Side,
                r#east: East::None,
            });
        }
        if state_id == 4771 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 10,
                r#south: South::None,
            });
        }
        if state_id == 4982 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#power: 2,
            });
        }
        if state_id == 5053 {
            return Some(RedstoneWire {
                r#power: 10,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4485 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 11,
            });
        }
        if state_id == 4134 {
            return Some(RedstoneWire {
                r#power: 4,
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4480 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#south: South::Side,
                r#east: East::Side,
                r#power: 10,
                r#north: North::Side,
            });
        }
        if state_id == 5088 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 14,
                r#south: South::Up,
                r#west: West::Up,
                r#north: North::None,
            });
        }
        if state_id == 4515 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 14,
                r#west: West::Up,
            });
        }
        if state_id == 5057 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#power: 10,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 4388 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 0,
                r#north: North::Side,
                r#west: West::None,
                r#east: East::Side,
            });
        }
        if state_id == 4738 {
            return Some(RedstoneWire {
                r#power: 7,
                r#west: West::Side,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 4549 {
            return Some(RedstoneWire {
                r#power: 2,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::None,
            });
        }
        if state_id == 4620 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#power: 10,
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::None,
            });
        }
        if state_id == 4030 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::Side,
                r#power: 8,
                r#east: East::Up,
            });
        }
        if state_id == 4106 {
            return Some(RedstoneWire {
                r#power: 0,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 4116 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#west: West::Up,
                r#south: South::Up,
                r#power: 2,
            });
        }
        if state_id == 4369 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 14,
            });
        }
        if state_id == 3889 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Up,
                r#power: 8,
            });
        }
        if state_id == 3971 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 1,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::None,
            });
        }
        if state_id == 4389 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::Side,
                r#north: North::Side,
                r#power: 0,
            });
        }
        if state_id == 4014 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::None,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 6,
            });
        }
        if state_id == 4812 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::None,
                r#power: 15,
            });
        }
        if state_id == 3868 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#power: 6,
                r#north: North::Up,
            });
        }
        if state_id == 4916 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::None,
                r#power: 10,
            });
        }
        if state_id == 4203 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 11,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4147 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#power: 5,
                r#east: East::Up,
                r#west: West::Side,
            });
        }
        if state_id == 3829 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#power: 2,
                r#south: South::Up,
                r#east: East::Up,
            });
        }
        if state_id == 4059 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Up,
                r#power: 11,
                r#north: North::Side,
            });
        }
        if state_id == 4067 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Side,
                r#power: 12,
                r#west: West::None,
            });
        }
        if state_id == 4286 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 4,
                r#north: North::Up,
                r#south: South::None,
                r#east: East::Side,
            });
        }
        if state_id == 3994 {
            return Some(RedstoneWire {
                r#power: 4,
                r#north: North::Side,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4195 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 10,
                r#north: North::None,
                r#west: West::Side,
                r#south: South::None,
            });
        }
        if state_id == 3996 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 4,
            });
        }
        if state_id == 4684 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::None,
                r#south: South::Up,
                r#power: 1,
            });
        }
        if state_id == 3826 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 1,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4442 {
            return Some(RedstoneWire {
                r#power: 6,
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4516 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::Side,
                r#power: 14,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4632 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::None,
                r#power: 11,
                r#east: East::Side,
            });
        }
        if state_id == 4602 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#power: 8,
                r#south: South::Up,
                r#west: West::Up,
            });
        }
        if state_id == 3883 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#east: East::Up,
                r#power: 8,
                r#north: North::Up,
                r#south: South::Up,
            });
        }
        if state_id == 3824 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::Up,
                r#power: 1,
            });
        }
        if state_id == 4013 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Up,
            });
        }
        if state_id == 4191 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 10,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4237 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#power: 15,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4767 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#power: 10,
                r#west: West::Up,
                r#east: East::None,
            });
        }
        if state_id == 4832 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 1,
                r#east: East::None,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4850 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Side,
                r#power: 3,
                r#east: East::None,
            });
        }
        if state_id == 4133 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 3,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 5076 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#power: 12,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4858 {
            return Some(RedstoneWire {
                r#power: 4,
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 3963 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::Up,
                r#power: 1,
                r#north: North::Side,
            });
        }
        if state_id == 4452 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 7,
                r#north: North::Side,
                r#west: West::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4580 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::None,
                r#power: 5,
                r#south: South::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4016 {
            return Some(RedstoneWire {
                r#power: 6,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4904 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 9,
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 4342 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#north: North::Up,
                r#east: East::Side,
                r#west: West::Side,
                r#power: 11,
            });
        }
        if state_id == 4926 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#power: 12,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 4132 {
            return Some(RedstoneWire {
                r#power: 3,
                r#north: North::None,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::None,
            });
        }
        if state_id == 4378 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::Side,
                r#power: 15,
                r#north: North::Up,
            });
        }
        if state_id == 4008 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#power: 6,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4677 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::Side,
                r#west: West::Up,
                r#north: North::Up,
                r#east: East::None,
            });
        }
        if state_id == 4006 {
            return Some(RedstoneWire {
                r#power: 5,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 3877 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 7,
            });
        }
        if state_id == 4891 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Side,
                r#power: 8,
            });
        }
        if state_id == 5034 {
            return Some(RedstoneWire {
                r#power: 8,
                r#north: North::None,
                r#west: West::Up,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4049 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 10,
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 4165 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 7,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4221 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 13,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4577 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 5,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4772 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
                r#power: 10,
                r#north: North::Up,
            });
        }
        if state_id == 3815 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Up,
                r#power: 0,
                r#south: South::Side,
                r#west: West::None,
            });
        }
        if state_id == 5039 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#power: 8,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4219 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 13,
                r#west: West::Side,
                r#north: North::None,
                r#east: East::Up,
            });
        }
        if state_id == 4197 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 11,
                r#north: North::None,
            });
        }
        if state_id == 4321 {
            return Some(RedstoneWire {
                r#power: 8,
                r#east: East::Side,
                r#south: South::None,
                r#north: North::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4477 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#power: 10,
                r#west: West::Side,
                r#east: East::Side,
                r#north: North::Side,
            });
        }
        if state_id == 3885 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 8,
                r#east: East::Up,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4151 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#power: 5,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4244 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Up,
                r#east: East::Side,
                r#power: 0,
                r#west: West::None,
            });
        }
        if state_id == 4329 {
            return Some(RedstoneWire {
                r#power: 9,
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4004 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#south: South::Side,
                r#east: East::Up,
                r#power: 5,
            });
        }
        if state_id == 4639 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#north: North::None,
                r#west: West::Side,
                r#power: 12,
            });
        }
        if state_id == 4822 {
            return Some(RedstoneWire {
                r#power: 0,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::None,
                r#north: North::Side,
            });
        }
        if state_id == 3961 {
            return Some(RedstoneWire {
                r#power: 0,
                r#west: West::Side,
                r#north: North::Side,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 3878 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#north: North::Up,
                r#power: 7,
                r#east: East::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4393 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 0,
                r#west: West::Side,
                r#south: South::None,
                r#north: North::Side,
            });
        }
        if state_id == 3917 {
            return Some(RedstoneWire {
                r#power: 11,
                r#north: North::Up,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Up,
            });
        }
        if state_id == 4451 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#north: North::Side,
                r#south: South::Up,
                r#power: 7,
            });
        }
        if state_id == 4735 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#north: North::Up,
                r#power: 6,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 4754 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 8,
                r#east: East::None,
                r#north: North::Up,
                r#south: South::None,
            });
        }
        if state_id == 4169 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 7,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 4839 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Side,
                r#east: East::None,
                r#south: South::Side,
                r#power: 2,
            });
        }
        if state_id == 3834 {
            return Some(RedstoneWire {
                r#power: 2,
                r#south: South::None,
                r#east: East::Up,
                r#west: West::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4876 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Side,
                r#power: 6,
                r#west: West::Side,
            });
        }
        if state_id == 5024 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#power: 6,
            });
        }
        if state_id == 5046 {
            return Some(RedstoneWire {
                r#power: 9,
                r#south: South::Side,
                r#west: West::Up,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 4629 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::None,
                r#power: 11,
                r#west: West::Up,
                r#south: South::Up,
            });
        }
        if state_id == 4513 {
            return Some(RedstoneWire {
                r#power: 14,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4885 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::None,
                r#power: 7,
                r#west: West::Side,
                r#north: North::Side,
            });
        }
        if state_id == 4759 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 9,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4635 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#power: 11,
                r#north: North::None,
                r#east: East::Side,
            });
        }
        if state_id == 3828 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Up,
                r#east: East::Up,
                r#south: South::Up,
                r#power: 2,
            });
        }
        if state_id == 4205 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Up,
                r#power: 11,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 4276 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::None,
                r#power: 3,
                r#east: East::Side,
            });
        }
        if state_id == 4346 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::None,
                r#south: South::Side,
                r#north: North::Up,
                r#power: 11,
            });
        }
        if state_id == 4947 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 14,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Up,
            });
        }
        if state_id == 4430 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#south: South::None,
                r#power: 4,
                r#north: North::Side,
            });
        }
        if state_id == 3998 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::Up,
                r#power: 4,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 3988 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#power: 3,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4350 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#power: 12,
                r#east: East::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4586 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#power: 6,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4353 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 12,
                r#west: West::Up,
                r#north: North::Up,
                r#south: South::Side,
            });
        }
        if state_id == 4456 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#west: West::Side,
                r#north: North::Side,
                r#south: South::None,
                r#power: 7,
            });
        }
        if state_id == 4431 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#east: East::Side,
                r#north: North::Side,
                r#power: 5,
                r#west: West::Up,
            });
        }
        if state_id == 4486 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 11,
            });
        }
        if state_id == 4720 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 5,
                r#west: West::Side,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4848 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Side,
                r#west: West::Up,
                r#power: 3,
                r#east: East::None,
            });
        }
        if state_id == 4120 {
            return Some(RedstoneWire {
                r#power: 2,
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4282 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#north: North::Up,
                r#west: West::Side,
                r#power: 4,
                r#east: East::Side,
            });
        }
        if state_id == 4457 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::None,
                r#power: 7,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 3814 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#south: South::Side,
                r#west: West::Side,
                r#east: East::Up,
                r#power: 0,
            });
        }
        if state_id == 4167 {
            return Some(RedstoneWire {
                r#power: 7,
                r#north: North::None,
                r#east: East::Up,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4055 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 11,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::None,
            });
        }
        if state_id == 4429 {
            return Some(RedstoneWire {
                r#power: 4,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4679 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#power: 0,
                r#north: North::Up,
                r#east: East::None,
                r#south: South::Side,
            });
        }
        if state_id == 4397 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#power: 1,
                r#west: West::None,
                r#south: South::Up,
            });
        }
        if state_id == 4526 {
            return Some(RedstoneWire {
                r#power: 15,
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Side,
                r#east: East::Side,
            });
        }
        if state_id == 4747 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 8,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::Side,
            });
        }
        if state_id == 4527 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 15,
                r#west: West::Up,
                r#east: East::Side,
                r#south: South::None,
            });
        }
        if state_id == 4791 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#north: North::Up,
                r#power: 13,
                r#south: South::Up,
                r#east: East::None,
            });
        }
        if state_id == 4844 {
            return Some(RedstoneWire {
                r#power: 2,
                r#west: West::None,
                r#north: North::Side,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 3871 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#east: East::Up,
                r#south: South::None,
                r#power: 6,
            });
        }
        if state_id == 4272 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#east: East::Side,
                r#north: North::Up,
                r#power: 3,
                r#west: West::Up,
            });
        }
        if state_id == 4940 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::Side,
                r#west: West::None,
                r#north: North::Side,
                r#power: 13,
            });
        }
        if state_id == 4271 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 3,
                r#west: West::None,
                r#south: South::Up,
                r#north: North::Up,
            });
        }
        if state_id == 4045 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 10,
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::Up,
            });
        }
        if state_id == 4655 {
            return Some(RedstoneWire {
                r#power: 13,
                r#south: South::None,
                r#east: East::Side,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 5041 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#west: West::Side,
                r#south: South::None,
                r#power: 8,
                r#east: East::None,
            });
        }
        if state_id == 4915 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Side,
                r#power: 10,
                r#west: West::Side,
            });
        }
        if state_id == 4112 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#south: South::Side,
                r#north: North::None,
                r#east: East::Up,
                r#power: 1,
            });
        }
        if state_id == 3870 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Up,
                r#north: North::Up,
                r#power: 6,
                r#east: East::Up,
            });
        }
        if state_id == 4960 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::None,
                r#west: West::Side,
                r#power: 15,
                r#east: East::None,
            });
        }
        if state_id == 5064 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Side,
                r#power: 11,
                r#west: West::Up,
            });
        }
        if state_id == 4152 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::None,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 6,
            });
        }
        if state_id == 4297 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::Side,
                r#power: 6,
                r#north: North::Up,
                r#east: East::Side,
            });
        }
        if state_id == 4427 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::Side,
                r#power: 4,
            });
        }
        if state_id == 3962 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#south: South::None,
                r#power: 0,
                r#east: East::Up,
            });
        }
        if state_id == 5011 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::Side,
                r#power: 5,
            });
        }
        if state_id == 4792 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#west: West::Side,
                r#south: South::Up,
                r#east: East::None,
                r#power: 13,
            });
        }
        if state_id == 5027 {
            return Some(RedstoneWire {
                r#power: 7,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Up,
            });
        }
        if state_id == 4263 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Up,
                r#power: 2,
                r#south: South::Side,
                r#west: West::Up,
            });
        }
        if state_id == 3964 {
            return Some(RedstoneWire {
                r#power: 1,
                r#south: South::Up,
                r#east: East::Up,
                r#north: North::Side,
                r#west: West::Side,
            });
        }
        if state_id == 4447 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#north: North::Side,
                r#west: West::Side,
                r#power: 6,
                r#south: South::None,
            });
        }
        if state_id == 5103 {
            return Some(RedstoneWire {
                r#power: 15,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Up,
                r#north: North::None,
            });
        }
        if state_id == 3858 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#power: 5,
                r#north: North::Up,
                r#east: East::Up,
                r#west: West::Up,
            });
        }
        if state_id == 4213 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#south: South::None,
                r#north: North::None,
                r#power: 12,
                r#west: West::Side,
            });
        }
        if state_id == 4428 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#power: 4,
                r#south: South::None,
                r#north: North::Side,
                r#west: West::Up,
            });
        }
        if state_id == 4050 {
            return Some(RedstoneWire {
                r#west: West::Up,
                r#east: East::Up,
                r#north: North::Side,
                r#power: 10,
                r#south: South::None,
            });
        }
        if state_id == 4861 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#east: East::None,
                r#power: 4,
                r#south: South::None,
                r#west: West::Side,
            });
        }
        if state_id == 4896 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#south: South::None,
                r#power: 8,
                r#west: West::Up,
                r#north: North::Side,
            });
        }
        if state_id == 4455 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#power: 7,
                r#east: East::Side,
                r#south: South::None,
                r#west: West::Up,
            });
        }
        if state_id == 4642 {
            return Some(RedstoneWire {
                r#north: North::None,
                r#east: East::Side,
                r#south: South::Side,
                r#power: 12,
                r#west: West::Side,
            });
        }
        if state_id == 4819 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 0,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
            });
        }
        if state_id == 4823 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#south: South::Side,
                r#west: West::None,
                r#east: East::None,
                r#power: 0,
            });
        }
        if state_id == 4514 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#power: 14,
                r#west: West::None,
                r#north: North::Side,
            });
        }
        if state_id == 3989 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#north: North::Side,
                r#south: South::None,
                r#west: West::None,
                r#power: 3,
            });
        }
        if state_id == 4748 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Up,
                r#power: 8,
                r#south: South::Up,
            });
        }
        if state_id == 4954 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#north: North::Side,
                r#south: South::Up,
                r#west: West::Side,
                r#power: 15,
            });
        }
        if state_id == 4355 {
            return Some(RedstoneWire {
                r#west: West::None,
                r#east: East::Side,
                r#power: 12,
                r#south: South::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4323 {
            return Some(RedstoneWire {
                r#east: East::Side,
                r#south: South::Up,
                r#west: West::Up,
                r#power: 9,
                r#north: North::Up,
            });
        }
        if state_id == 4372 {
            return Some(RedstoneWire {
                r#west: West::Side,
                r#power: 14,
                r#north: North::Up,
                r#east: East::Side,
                r#south: South::Side,
            });
        }
        if state_id == 4370 {
            return Some(RedstoneWire {
                r#south: South::Up,
                r#west: West::None,
                r#power: 14,
                r#east: East::Side,
                r#north: North::Up,
            });
        }
        if state_id == 4820 {
            return Some(RedstoneWire {
                r#north: North::Side,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Up,
                r#power: 0,
            });
        }
        if state_id == 5049 {
            return Some(RedstoneWire {
                r#east: East::None,
                r#power: 9,
                r#west: West::Up,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 4778 {
            return Some(RedstoneWire {
                r#north: North::Up,
                r#power: 11,
                r#south: South::Side,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 4129 {
            return Some(RedstoneWire {
                r#east: East::Up,
                r#power: 3,
                r#south: South::Side,
                r#north: North::None,
                r#west: West::Side,
            });
        }
        if state_id == 4465 {
            return Some(RedstoneWire {
                r#south: South::None,
                r#west: West::Side,
                r#north: North::Side,
                r#power: 8,
                r#east: East::Side,
            });
        }
        if state_id == 4818 {
            return Some(RedstoneWire {
                r#power: 0,
                r#north: North::Side,
                r#south: South::Up,
                r#east: East::None,
                r#west: West::Up,
            });
        }
        if state_id == 4570 {
            return Some(RedstoneWire {
                r#south: South::Side,
                r#west: West::Side,
                r#north: North::None,
                r#east: East::Side,
                r#power: 4,
            });
        }
        return None;
    }
}

