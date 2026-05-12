use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tripwire {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub powered: bool,
    pub west: bool,
    pub attached: bool,
    pub disarmed: bool,
}


impl BlockState for Tripwire {
    fn to_id(self) -> i32 {
        if block_state.r#disarmed == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true && block_state.r#attached == true && block_state.r#powered == false && block_state.r#east == true { return 9404; }
        if block_state.r#disarmed == true && block_state.r#west == false && block_state.r#south == false && block_state.r#powered == true && block_state.r#attached == false && block_state.r#east == true && block_state.r#north == true { return 9465; }
        if block_state.r#west == true && block_state.r#attached == false && block_state.r#powered == false && block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#disarmed == false { return 9498; }
        if block_state.r#disarmed == true && block_state.r#attached == false && block_state.r#powered == false && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#north == false { return 9493; }
        if block_state.r#east == true && block_state.r#attached == true && block_state.r#north == false && block_state.r#disarmed == true && block_state.r#powered == true && block_state.r#south == false && block_state.r#west == false { return 9409; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#attached == false && block_state.r#south == true && block_state.r#powered == true && block_state.r#disarmed == false && block_state.r#east == true { return 9495; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#south == false && block_state.r#attached == false && block_state.r#disarmed == false && block_state.r#north == true && block_state.r#powered == true { return 9496; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#attached == false && block_state.r#powered == true && block_state.r#east == false && block_state.r#south == true && block_state.r#disarmed == false { return 9518; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#powered == false && block_state.r#west == false && block_state.r#disarmed == false && block_state.r#south == true && block_state.r#attached == false { return 9515; }
        if block_state.r#disarmed == true && block_state.r#powered == false && block_state.r#north == false && block_state.r#east == true && block_state.r#south == true && block_state.r#west == false && block_state.r#attached == true { return 9411; }
        if block_state.r#powered == true && block_state.r#disarmed == false && block_state.r#north == true && block_state.r#south == true && block_state.r#east == true && block_state.r#attached == true && block_state.r#west == false { return 9431; }
        if block_state.r#west == true && block_state.r#attached == false && block_state.r#disarmed == true && block_state.r#north == true && block_state.r#powered == true && block_state.r#east == true && block_state.r#south == false { return 9464; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#attached == false && block_state.r#east == true && block_state.r#disarmed == false && block_state.r#north == false && block_state.r#powered == false { return 9508; }
        if block_state.r#powered == true && block_state.r#attached == true && block_state.r#disarmed == true && block_state.r#east == false && block_state.r#west == false && block_state.r#north == true && block_state.r#south == false { return 9417; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#disarmed == false && block_state.r#attached == true && block_state.r#powered == false && block_state.r#east == false && block_state.r#south == true { return 9451; }
        if block_state.r#powered == true && block_state.r#attached == true && block_state.r#east == false && block_state.r#north == false && block_state.r#west == true && block_state.r#south == true && block_state.r#disarmed == false { return 9454; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#attached == true && block_state.r#disarmed == false && block_state.r#east == true && block_state.r#north == true && block_state.r#powered == false { return 9436; }
        if block_state.r#attached == true && block_state.r#south == false && block_state.r#disarmed == true && block_state.r#north == false && block_state.r#west == true && block_state.r#powered == true && block_state.r#east == true { return 9408; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#west == true && block_state.r#east == false && block_state.r#powered == false && block_state.r#disarmed == false && block_state.r#attached == true { return 9458; }
        if block_state.r#powered == false && block_state.r#attached == true && block_state.r#disarmed == true && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true { return 9420; }
        if block_state.r#south == true && block_state.r#disarmed == false && block_state.r#east == false && block_state.r#north == true && block_state.r#attached == false && block_state.r#west == false && block_state.r#powered == true { return 9511; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#powered == false && block_state.r#south == false && block_state.r#attached == false && block_state.r#disarmed == false && block_state.r#north == true { return 9516; }
        if block_state.r#attached == false && block_state.r#north == false && block_state.r#powered == true && block_state.r#disarmed == false && block_state.r#west == false && block_state.r#south == true && block_state.r#east == true { return 9503; }
        if block_state.r#attached == false && block_state.r#disarmed == true && block_state.r#south == true && block_state.r#powered == false && block_state.r#north == false && block_state.r#east == false && block_state.r#west == false { return 9491; }
        if block_state.r#west == false && block_state.r#disarmed == true && block_state.r#east == false && block_state.r#north == true && block_state.r#powered == false && block_state.r#attached == true && block_state.r#south == true { return 9419; }
        if block_state.r#east == false && block_state.r#powered == false && block_state.r#attached == true && block_state.r#disarmed == true && block_state.r#south == true && block_state.r#west == true && block_state.r#north == false { return 9426; }
        if block_state.r#west == true && block_state.r#attached == true && block_state.r#powered == false && block_state.r#south == false && block_state.r#disarmed == true && block_state.r#east == false && block_state.r#north == false { return 9428; }
        if block_state.r#south == true && block_state.r#disarmed == true && block_state.r#east == true && block_state.r#north == true && block_state.r#powered == false && block_state.r#attached == true && block_state.r#west == true { return 9402; }
        if block_state.r#attached == false && block_state.r#disarmed == true && block_state.r#powered == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == false { return 9475; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#disarmed == true && block_state.r#east == false && block_state.r#powered == true && block_state.r#attached == false && block_state.r#north == false { return 9488; }
        if block_state.r#disarmed == true && block_state.r#attached == false && block_state.r#north == false && block_state.r#east == true && block_state.r#west == true && block_state.r#powered == false && block_state.r#south == true { return 9474; }
        if block_state.r#west == true && block_state.r#disarmed == false && block_state.r#attached == false && block_state.r#east == true && block_state.r#north == false && block_state.r#powered == true && block_state.r#south == true { return 9502; }
        if block_state.r#disarmed == true && block_state.r#north == true && block_state.r#powered == true && block_state.r#south == true && block_state.r#west == true && block_state.r#attached == false && block_state.r#east == false { return 9478; }
        if block_state.r#attached == true && block_state.r#north == false && block_state.r#south == true && block_state.r#powered == true && block_state.r#west == false && block_state.r#disarmed == false && block_state.r#east == true { return 9439; }
        if block_state.r#disarmed == true && block_state.r#west == false && block_state.r#north == true && block_state.r#attached == true && block_state.r#south == true && block_state.r#powered == false && block_state.r#east == true { return 9403; }
        if block_state.r#attached == false && block_state.r#south == false && block_state.r#powered == false && block_state.r#east == true && block_state.r#west == true && block_state.r#north == true && block_state.r#disarmed == true { return 9468; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#west == false && block_state.r#attached == true && block_state.r#east == false && block_state.r#powered == true && block_state.r#disarmed == true { return 9415; }
        if block_state.r#powered == false && block_state.r#east == false && block_state.r#north == true && block_state.r#disarmed == true && block_state.r#south == true && block_state.r#attached == true && block_state.r#west == true { return 9418; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#powered == true && block_state.r#disarmed == true && block_state.r#south == false && block_state.r#east == false && block_state.r#attached == true { return 9424; }
        if block_state.r#west == true && block_state.r#disarmed == false && block_state.r#north == false && block_state.r#east == true && block_state.r#attached == false && block_state.r#south == true && block_state.r#powered == false { return 9506; }
        if block_state.r#attached == false && block_state.r#disarmed == true && block_state.r#north == true && block_state.r#east == false && block_state.r#powered == false && block_state.r#south == true && block_state.r#west == true { return 9482; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#powered == false && block_state.r#disarmed == false && block_state.r#attached == false && block_state.r#east == true && block_state.r#north == true { return 9501; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#north == false && block_state.r#disarmed == true && block_state.r#attached == false && block_state.r#east == false && block_state.r#powered == false { return 9490; }
        if block_state.r#east == false && block_state.r#powered == true && block_state.r#attached == true && block_state.r#north == false && block_state.r#south == true && block_state.r#disarmed == true && block_state.r#west == false { return 9423; }
        if block_state.r#powered == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#attached == true && block_state.r#disarmed == true { return 9413; }
        if block_state.r#attached == true && block_state.r#west == false && block_state.r#north == false && block_state.r#disarmed == true && block_state.r#south == false && block_state.r#powered == true && block_state.r#east == false { return 9425; }
        if block_state.r#north == true && block_state.r#disarmed == false && block_state.r#attached == true && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true && block_state.r#powered == true { return 9430; }
        if block_state.r#east == true && block_state.r#attached == true && block_state.r#north == false && block_state.r#powered == true && block_state.r#disarmed == false && block_state.r#south == true && block_state.r#west == true { return 9438; }
        if block_state.r#disarmed == false && block_state.r#powered == false && block_state.r#north == false && block_state.r#attached == true && block_state.r#west == false && block_state.r#south == true && block_state.r#east == true { return 9443; }
        if block_state.r#east == true && block_state.r#powered == false && block_state.r#disarmed == true && block_state.r#south == true && block_state.r#west == true && block_state.r#attached == false && block_state.r#north == true { return 9466; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#west == false && block_state.r#attached == false && block_state.r#east == true && block_state.r#powered == false && block_state.r#disarmed == false { return 9507; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#powered == false && block_state.r#attached == false && block_state.r#east == true && block_state.r#south == true && block_state.r#disarmed == true { return 9467; }
        if block_state.r#attached == true && block_state.r#east == true && block_state.r#powered == true && block_state.r#north == true && block_state.r#disarmed == true && block_state.r#south == true && block_state.r#west == false { return 9399; }
        if block_state.r#west == false && block_state.r#disarmed == false && block_state.r#powered == false && block_state.r#north == false && block_state.r#attached == true && block_state.r#east == true && block_state.r#south == false { return 9445; }
        if block_state.r#north == true && block_state.r#disarmed == false && block_state.r#powered == true && block_state.r#south == false && block_state.r#west == true && block_state.r#attached == true && block_state.r#east == true { return 9432; }
        if block_state.r#west == false && block_state.r#attached == true && block_state.r#disarmed == false && block_state.r#north == true && block_state.r#powered == true && block_state.r#east == true && block_state.r#south == false { return 9433; }
        if block_state.r#disarmed == false && block_state.r#west == true && block_state.r#powered == false && block_state.r#east == false && block_state.r#north == false && block_state.r#attached == false && block_state.r#south == true { return 9522; }
        if block_state.r#disarmed == false && block_state.r#north == true && block_state.r#powered == true && block_state.r#west == true && block_state.r#south == true && block_state.r#attached == true && block_state.r#east == false { return 9446; }
        if block_state.r#powered == true && block_state.r#west == false && block_state.r#south == false && block_state.r#north == false && block_state.r#disarmed == false && block_state.r#east == false && block_state.r#attached == false { return 9521; }
        if block_state.r#attached == true && block_state.r#west == true && block_state.r#north == true && block_state.r#disarmed == false && block_state.r#east == true && block_state.r#powered == false && block_state.r#south == true { return 9434; }
        if block_state.r#attached == true && block_state.r#west == true && block_state.r#north == false && block_state.r#disarmed == false && block_state.r#east == true && block_state.r#powered == false && block_state.r#south == false { return 9444; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#disarmed == true && block_state.r#powered == true && block_state.r#attached == false && block_state.r#north == false && block_state.r#west == true { return 9470; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#disarmed == true && block_state.r#powered == true && block_state.r#east == false && block_state.r#attached == true && block_state.r#south == true { return 9414; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#disarmed == true && block_state.r#attached == false && block_state.r#powered == false && block_state.r#west == true && block_state.r#north == false { return 9492; }
        if block_state.r#attached == true && block_state.r#disarmed == true && block_state.r#east == true && block_state.r#north == true && block_state.r#powered == false && block_state.r#south == false && block_state.r#west == false { return 9405; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#west == true && block_state.r#disarmed == false && block_state.r#powered == false && block_state.r#attached == false && block_state.r#north == true { return 9514; }
        if block_state.r#east == true && block_state.r#powered == true && block_state.r#west == true && block_state.r#disarmed == true && block_state.r#attached == false && block_state.r#north == false && block_state.r#south == false { return 9472; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#attached == true && block_state.r#disarmed == true && block_state.r#powered == false && block_state.r#east == false { return 9429; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#powered == true && block_state.r#south == true && block_state.r#west == true && block_state.r#disarmed == false && block_state.r#attached == false { return 9510; }
        if block_state.r#powered == false && block_state.r#attached == false && block_state.r#east == true && block_state.r#disarmed == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false { return 9469; }
        if block_state.r#attached == false && block_state.r#powered == true && block_state.r#west == false && block_state.r#east == true && block_state.r#south == true && block_state.r#north == true && block_state.r#disarmed == true { return 9463; }
        if block_state.r#attached == false && block_state.r#north == false && block_state.r#powered == true && block_state.r#south == false && block_state.r#west == false && block_state.r#east == false && block_state.r#disarmed == true { return 9489; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#powered == false && block_state.r#south == false && block_state.r#east == true && block_state.r#attached == false && block_state.r#disarmed == true { return 9477; }
        if block_state.r#south == true && block_state.r#attached == false && block_state.r#disarmed == false && block_state.r#west == false && block_state.r#east == false && block_state.r#north == false && block_state.r#powered == true { return 9519; }
        if block_state.r#powered == true && block_state.r#disarmed == false && block_state.r#south == false && block_state.r#west == false && block_state.r#attached == false && block_state.r#east == false && block_state.r#north == true { return 9513; }
        if block_state.r#east == false && block_state.r#disarmed == false && block_state.r#west == false && block_state.r#north == true && block_state.r#south == false && block_state.r#powered == true && block_state.r#attached == true { return 9449; }
        if block_state.r#west == false && block_state.r#disarmed == false && block_state.r#north == false && block_state.r#east == false && block_state.r#powered == true && block_state.r#south == false && block_state.r#attached == true { return 9457; }
        if block_state.r#attached == true && block_state.r#west == false && block_state.r#disarmed == false && block_state.r#east == false && block_state.r#north == false && block_state.r#powered == false && block_state.r#south == false { return 9461; }
        if block_state.r#north == false && block_state.r#powered == false && block_state.r#east == true && block_state.r#attached == false && block_state.r#south == false && block_state.r#west == true && block_state.r#disarmed == true { return 9476; }
        if block_state.r#west == false && block_state.r#attached == false && block_state.r#powered == false && block_state.r#east == false && block_state.r#disarmed == true && block_state.r#north == true && block_state.r#south == true { return 9483; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#powered == true && block_state.r#disarmed == true && block_state.r#attached == true && block_state.r#south == true && block_state.r#west == true { return 9422; }
        if block_state.r#powered == true && block_state.r#west == true && block_state.r#north == false && block_state.r#south == true && block_state.r#attached == false && block_state.r#east == false && block_state.r#disarmed == true { return 9486; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#powered == false && block_state.r#south == false && block_state.r#attached == false && block_state.r#disarmed == false && block_state.r#west == false { return 9525; }
        if block_state.r#east == true && block_state.r#powered == true && block_state.r#attached == true && block_state.r#north == true && block_state.r#disarmed == true && block_state.r#south == false && block_state.r#west == true { return 9400; }
        if block_state.r#east == false && block_state.r#disarmed == false && block_state.r#north == false && block_state.r#attached == true && block_state.r#powered == false && block_state.r#west == false && block_state.r#south == true { return 9459; }
        if block_state.r#north == true && block_state.r#powered == false && block_state.r#attached == true && block_state.r#disarmed == false && block_state.r#east == false && block_state.r#west == true && block_state.r#south == true { return 9450; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#east == true && block_state.r#disarmed == false && block_state.r#powered == true && block_state.r#north == false && block_state.r#attached == false { return 9505; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#attached == true && block_state.r#disarmed == false && block_state.r#east == true && block_state.r#north == false && block_state.r#powered == false { return 9442; }
        if block_state.r#powered == true && block_state.r#west == true && block_state.r#north == false && block_state.r#disarmed == false && block_state.r#east == true && block_state.r#attached == true && block_state.r#south == false { return 9440; }
        if block_state.r#attached == false && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true && block_state.r#east == false && block_state.r#powered == true && block_state.r#disarmed == false { return 9512; }
        if block_state.r#powered == true && block_state.r#attached == true && block_state.r#south == true && block_state.r#disarmed == false && block_state.r#east == false && block_state.r#north == false && block_state.r#west == false { return 9455; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#north == true && block_state.r#east == true && block_state.r#powered == false && block_state.r#attached == false && block_state.r#disarmed == false { return 9499; }
        if block_state.r#east == true && block_state.r#attached == true && block_state.r#disarmed == false && block_state.r#west == false && block_state.r#north == true && block_state.r#powered == false && block_state.r#south == false { return 9437; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#powered == true && block_state.r#north == false && block_state.r#disarmed == true && block_state.r#south == true && block_state.r#attached == true { return 9407; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#attached == true && block_state.r#disarmed == false && block_state.r#east == true && block_state.r#north == false && block_state.r#powered == true { return 9441; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#disarmed == true && block_state.r#attached == false && block_state.r#north == true && block_state.r#east == false && block_state.r#powered == true { return 9479; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#disarmed == false && block_state.r#east == true && block_state.r#powered == false && block_state.r#attached == false && block_state.r#west == true { return 9500; }
        if block_state.r#east == false && block_state.r#attached == true && block_state.r#north == true && block_state.r#west == false && block_state.r#disarmed == false && block_state.r#powered == false && block_state.r#south == false { return 9453; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == false && block_state.r#powered == false && block_state.r#attached == false && block_state.r#disarmed == false && block_state.r#south == true { return 9523; }
        if block_state.r#disarmed == true && block_state.r#west == false && block_state.r#powered == true && block_state.r#attached == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false { return 9473; }
        if block_state.r#powered == true && block_state.r#west == true && block_state.r#attached == true && block_state.r#disarmed == false && block_state.r#south == false && block_state.r#north == false && block_state.r#east == false { return 9456; }
        if block_state.r#attached == false && block_state.r#north == true && block_state.r#powered == true && block_state.r#disarmed == false && block_state.r#south == true && block_state.r#east == true && block_state.r#west == true { return 9494; }
        if block_state.r#east == true && block_state.r#powered == false && block_state.r#south == true && block_state.r#west == false && block_state.r#attached == true && block_state.r#north == true && block_state.r#disarmed == false { return 9435; }
        if block_state.r#east == true && block_state.r#powered == true && block_state.r#west == true && block_state.r#south == true && block_state.r#attached == false && block_state.r#disarmed == true && block_state.r#north == true { return 9462; }
        if block_state.r#disarmed == true && block_state.r#attached == false && block_state.r#north == true && block_state.r#powered == false && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false { return 9485; }
        if block_state.r#north == false && block_state.r#disarmed == false && block_state.r#attached == false && block_state.r#south == false && block_state.r#east == true && block_state.r#west == true && block_state.r#powered == true { return 9504; }
        if block_state.r#disarmed == false && block_state.r#north == true && block_state.r#powered == true && block_state.r#west == false && block_state.r#south == true && block_state.r#attached == true && block_state.r#east == false { return 9447; }
        if block_state.r#attached == true && block_state.r#east == false && block_state.r#powered == false && block_state.r#south == false && block_state.r#disarmed == false && block_state.r#north == true && block_state.r#west == true { return 9452; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#disarmed == true && block_state.r#east == true && block_state.r#attached == false && block_state.r#powered == true { return 9471; }
        if block_state.r#north == false && block_state.r#disarmed == false && block_state.r#powered == true && block_state.r#south == false && block_state.r#east == false && block_state.r#west == true && block_state.r#attached == false { return 9520; }
        if block_state.r#south == false && block_state.r#powered == false && block_state.r#east == true && block_state.r#attached == true && block_state.r#north == false && block_state.r#west == true && block_state.r#disarmed == true { return 9412; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#disarmed == true && block_state.r#west == false && block_state.r#attached == true && block_state.r#north == true && block_state.r#powered == false { return 9421; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#powered == true && block_state.r#attached == false && block_state.r#north == true && block_state.r#disarmed == false && block_state.r#east == true { return 9497; }
        if block_state.r#attached == false && block_state.r#north == true && block_state.r#east == false && block_state.r#powered == false && block_state.r#disarmed == true && block_state.r#south == false && block_state.r#west == true { return 9484; }
        if block_state.r#west == true && block_state.r#disarmed == false && block_state.r#north == true && block_state.r#attached == true && block_state.r#powered == true && block_state.r#south == false && block_state.r#east == false { return 9448; }
        if block_state.r#attached == false && block_state.r#disarmed == false && block_state.r#south == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#powered == false { return 9509; }
        if block_state.r#east == false && block_state.r#powered == false && block_state.r#south == false && block_state.r#west == true && block_state.r#north == false && block_state.r#attached == true && block_state.r#disarmed == false { return 9460; }
        if block_state.r#west == true && block_state.r#attached == true && block_state.r#east == false && block_state.r#north == true && block_state.r#disarmed == true && block_state.r#powered == true && block_state.r#south == false { return 9416; }
        if block_state.r#attached == false && block_state.r#disarmed == false && block_state.r#east == false && block_state.r#north == false && block_state.r#powered == false && block_state.r#south == false && block_state.r#west == true { return 9524; }
        if block_state.r#attached == false && block_state.r#north == true && block_state.r#powered == true && block_state.r#south == false && block_state.r#west == false && block_state.r#disarmed == true && block_state.r#east == false { return 9481; }
        if block_state.r#east == false && block_state.r#attached == false && block_state.r#west == false && block_state.r#powered == true && block_state.r#disarmed == true && block_state.r#north == false && block_state.r#south == true { return 9487; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#attached == true && block_state.r#north == true && block_state.r#west == true && block_state.r#powered == true && block_state.r#disarmed == true { return 9398; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#attached == false && block_state.r#west == false && block_state.r#powered == false && block_state.r#east == false && block_state.r#disarmed == false { return 9517; }
        if block_state.r#west == true && block_state.r#attached == false && block_state.r#disarmed == true && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false && block_state.r#powered == true { return 9480; }
        if block_state.r#attached == true && block_state.r#south == true && block_state.r#disarmed == true && block_state.r#north == false && block_state.r#west == true && block_state.r#powered == true && block_state.r#east == true { return 9406; }
        if block_state.r#disarmed == true && block_state.r#powered == false && block_state.r#east == false && block_state.r#north == false && block_state.r#attached == true && block_state.r#west == false && block_state.r#south == true { return 9427; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#disarmed == true && block_state.r#attached == true && block_state.r#powered == false { return 9410; }
        if block_state.r#attached == true && block_state.r#disarmed == true && block_state.r#south == false && block_state.r#north == true && block_state.r#east == true && block_state.r#powered == true && block_state.r#west == false { return 9401; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9404 {
            return Some(Tripwire {
                r#disarmed: true,
                r#north: true,
                r#south: false,
                r#west: true,
                r#attached: true,
                r#powered: false,
                r#east: true,
            });
        }
        if state_id == 9465 {
            return Some(Tripwire {
                r#disarmed: true,
                r#west: false,
                r#south: false,
                r#powered: true,
                r#attached: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 9498 {
            return Some(Tripwire {
                r#west: true,
                r#attached: false,
                r#powered: false,
                r#north: true,
                r#east: true,
                r#south: true,
                r#disarmed: false,
            });
        }
        if state_id == 9493 {
            return Some(Tripwire {
                r#disarmed: true,
                r#attached: false,
                r#powered: false,
                r#east: false,
                r#south: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 9409 {
            return Some(Tripwire {
                r#east: true,
                r#attached: true,
                r#north: false,
                r#disarmed: true,
                r#powered: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 9495 {
            return Some(Tripwire {
                r#west: false,
                r#north: true,
                r#attached: false,
                r#south: true,
                r#powered: true,
                r#disarmed: false,
                r#east: true,
            });
        }
        if state_id == 9496 {
            return Some(Tripwire {
                r#east: true,
                r#west: true,
                r#south: false,
                r#attached: false,
                r#disarmed: false,
                r#north: true,
                r#powered: true,
            });
        }
        if state_id == 9518 {
            return Some(Tripwire {
                r#north: false,
                r#west: true,
                r#attached: false,
                r#powered: true,
                r#east: false,
                r#south: true,
                r#disarmed: false,
            });
        }
        if state_id == 9515 {
            return Some(Tripwire {
                r#north: true,
                r#east: false,
                r#powered: false,
                r#west: false,
                r#disarmed: false,
                r#south: true,
                r#attached: false,
            });
        }
        if state_id == 9411 {
            return Some(Tripwire {
                r#disarmed: true,
                r#powered: false,
                r#north: false,
                r#east: true,
                r#south: true,
                r#west: false,
                r#attached: true,
            });
        }
        if state_id == 9431 {
            return Some(Tripwire {
                r#powered: true,
                r#disarmed: false,
                r#north: true,
                r#south: true,
                r#east: true,
                r#attached: true,
                r#west: false,
            });
        }
        if state_id == 9464 {
            return Some(Tripwire {
                r#west: true,
                r#attached: false,
                r#disarmed: true,
                r#north: true,
                r#powered: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 9508 {
            return Some(Tripwire {
                r#west: true,
                r#south: false,
                r#attached: false,
                r#east: true,
                r#disarmed: false,
                r#north: false,
                r#powered: false,
            });
        }
        if state_id == 9417 {
            return Some(Tripwire {
                r#powered: true,
                r#attached: true,
                r#disarmed: true,
                r#east: false,
                r#west: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 9451 {
            return Some(Tripwire {
                r#west: false,
                r#north: true,
                r#disarmed: false,
                r#attached: true,
                r#powered: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 9454 {
            return Some(Tripwire {
                r#powered: true,
                r#attached: true,
                r#east: false,
                r#north: false,
                r#west: true,
                r#south: true,
                r#disarmed: false,
            });
        }
        if state_id == 9436 {
            return Some(Tripwire {
                r#west: true,
                r#south: false,
                r#attached: true,
                r#disarmed: false,
                r#east: true,
                r#north: true,
                r#powered: false,
            });
        }
        if state_id == 9408 {
            return Some(Tripwire {
                r#attached: true,
                r#south: false,
                r#disarmed: true,
                r#north: false,
                r#west: true,
                r#powered: true,
                r#east: true,
            });
        }
        if state_id == 9458 {
            return Some(Tripwire {
                r#south: true,
                r#north: false,
                r#west: true,
                r#east: false,
                r#powered: false,
                r#disarmed: false,
                r#attached: true,
            });
        }
        if state_id == 9420 {
            return Some(Tripwire {
                r#powered: false,
                r#attached: true,
                r#disarmed: true,
                r#east: false,
                r#north: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 9511 {
            return Some(Tripwire {
                r#south: true,
                r#disarmed: false,
                r#east: false,
                r#north: true,
                r#attached: false,
                r#west: false,
                r#powered: true,
            });
        }
        if state_id == 9516 {
            return Some(Tripwire {
                r#east: false,
                r#west: true,
                r#powered: false,
                r#south: false,
                r#attached: false,
                r#disarmed: false,
                r#north: true,
            });
        }
        if state_id == 9503 {
            return Some(Tripwire {
                r#attached: false,
                r#north: false,
                r#powered: true,
                r#disarmed: false,
                r#west: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 9491 {
            return Some(Tripwire {
                r#attached: false,
                r#disarmed: true,
                r#south: true,
                r#powered: false,
                r#north: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 9419 {
            return Some(Tripwire {
                r#west: false,
                r#disarmed: true,
                r#east: false,
                r#north: true,
                r#powered: false,
                r#attached: true,
                r#south: true,
            });
        }
        if state_id == 9426 {
            return Some(Tripwire {
                r#east: false,
                r#powered: false,
                r#attached: true,
                r#disarmed: true,
                r#south: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 9428 {
            return Some(Tripwire {
                r#west: true,
                r#attached: true,
                r#powered: false,
                r#south: false,
                r#disarmed: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 9402 {
            return Some(Tripwire {
                r#south: true,
                r#disarmed: true,
                r#east: true,
                r#north: true,
                r#powered: false,
                r#attached: true,
                r#west: true,
            });
        }
        if state_id == 9475 {
            return Some(Tripwire {
                r#attached: false,
                r#disarmed: true,
                r#powered: false,
                r#south: true,
                r#east: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 9488 {
            return Some(Tripwire {
                r#south: false,
                r#west: true,
                r#disarmed: true,
                r#east: false,
                r#powered: true,
                r#attached: false,
                r#north: false,
            });
        }
        if state_id == 9474 {
            return Some(Tripwire {
                r#disarmed: true,
                r#attached: false,
                r#north: false,
                r#east: true,
                r#west: true,
                r#powered: false,
                r#south: true,
            });
        }
        if state_id == 9502 {
            return Some(Tripwire {
                r#west: true,
                r#disarmed: false,
                r#attached: false,
                r#east: true,
                r#north: false,
                r#powered: true,
                r#south: true,
            });
        }
        if state_id == 9478 {
            return Some(Tripwire {
                r#disarmed: true,
                r#north: true,
                r#powered: true,
                r#south: true,
                r#west: true,
                r#attached: false,
                r#east: false,
            });
        }
        if state_id == 9439 {
            return Some(Tripwire {
                r#attached: true,
                r#north: false,
                r#south: true,
                r#powered: true,
                r#west: false,
                r#disarmed: false,
                r#east: true,
            });
        }
        if state_id == 9403 {
            return Some(Tripwire {
                r#disarmed: true,
                r#west: false,
                r#north: true,
                r#attached: true,
                r#south: true,
                r#powered: false,
                r#east: true,
            });
        }
        if state_id == 9468 {
            return Some(Tripwire {
                r#attached: false,
                r#south: false,
                r#powered: false,
                r#east: true,
                r#west: true,
                r#north: true,
                r#disarmed: true,
            });
        }
        if state_id == 9415 {
            return Some(Tripwire {
                r#north: true,
                r#south: true,
                r#west: false,
                r#attached: true,
                r#east: false,
                r#powered: true,
                r#disarmed: true,
            });
        }
        if state_id == 9418 {
            return Some(Tripwire {
                r#powered: false,
                r#east: false,
                r#north: true,
                r#disarmed: true,
                r#south: true,
                r#attached: true,
                r#west: true,
            });
        }
        if state_id == 9424 {
            return Some(Tripwire {
                r#north: false,
                r#west: true,
                r#powered: true,
                r#disarmed: true,
                r#south: false,
                r#east: false,
                r#attached: true,
            });
        }
        if state_id == 9506 {
            return Some(Tripwire {
                r#west: true,
                r#disarmed: false,
                r#north: false,
                r#east: true,
                r#attached: false,
                r#south: true,
                r#powered: false,
            });
        }
        if state_id == 9482 {
            return Some(Tripwire {
                r#attached: false,
                r#disarmed: true,
                r#north: true,
                r#east: false,
                r#powered: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 9501 {
            return Some(Tripwire {
                r#south: false,
                r#west: false,
                r#powered: false,
                r#disarmed: false,
                r#attached: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 9490 {
            return Some(Tripwire {
                r#south: true,
                r#west: true,
                r#north: false,
                r#disarmed: true,
                r#attached: false,
                r#east: false,
                r#powered: false,
            });
        }
        if state_id == 9423 {
            return Some(Tripwire {
                r#east: false,
                r#powered: true,
                r#attached: true,
                r#north: false,
                r#south: true,
                r#disarmed: true,
                r#west: false,
            });
        }
        if state_id == 9413 {
            return Some(Tripwire {
                r#powered: false,
                r#east: true,
                r#north: false,
                r#south: false,
                r#west: false,
                r#attached: true,
                r#disarmed: true,
            });
        }
        if state_id == 9425 {
            return Some(Tripwire {
                r#attached: true,
                r#west: false,
                r#north: false,
                r#disarmed: true,
                r#south: false,
                r#powered: true,
                r#east: false,
            });
        }
        if state_id == 9430 {
            return Some(Tripwire {
                r#north: true,
                r#disarmed: false,
                r#attached: true,
                r#west: true,
                r#east: true,
                r#south: true,
                r#powered: true,
            });
        }
        if state_id == 9438 {
            return Some(Tripwire {
                r#east: true,
                r#attached: true,
                r#north: false,
                r#powered: true,
                r#disarmed: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 9443 {
            return Some(Tripwire {
                r#disarmed: false,
                r#powered: false,
                r#north: false,
                r#attached: true,
                r#west: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 9466 {
            return Some(Tripwire {
                r#east: true,
                r#powered: false,
                r#disarmed: true,
                r#south: true,
                r#west: true,
                r#attached: false,
                r#north: true,
            });
        }
        if state_id == 9507 {
            return Some(Tripwire {
                r#north: false,
                r#south: true,
                r#west: false,
                r#attached: false,
                r#east: true,
                r#powered: false,
                r#disarmed: false,
            });
        }
        if state_id == 9467 {
            return Some(Tripwire {
                r#north: true,
                r#west: false,
                r#powered: false,
                r#attached: false,
                r#east: true,
                r#south: true,
                r#disarmed: true,
            });
        }
        if state_id == 9399 {
            return Some(Tripwire {
                r#attached: true,
                r#east: true,
                r#powered: true,
                r#north: true,
                r#disarmed: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 9445 {
            return Some(Tripwire {
                r#west: false,
                r#disarmed: false,
                r#powered: false,
                r#north: false,
                r#attached: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 9432 {
            return Some(Tripwire {
                r#north: true,
                r#disarmed: false,
                r#powered: true,
                r#south: false,
                r#west: true,
                r#attached: true,
                r#east: true,
            });
        }
        if state_id == 9433 {
            return Some(Tripwire {
                r#west: false,
                r#attached: true,
                r#disarmed: false,
                r#north: true,
                r#powered: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 9522 {
            return Some(Tripwire {
                r#disarmed: false,
                r#west: true,
                r#powered: false,
                r#east: false,
                r#north: false,
                r#attached: false,
                r#south: true,
            });
        }
        if state_id == 9446 {
            return Some(Tripwire {
                r#disarmed: false,
                r#north: true,
                r#powered: true,
                r#west: true,
                r#south: true,
                r#attached: true,
                r#east: false,
            });
        }
        if state_id == 9521 {
            return Some(Tripwire {
                r#powered: true,
                r#west: false,
                r#south: false,
                r#north: false,
                r#disarmed: false,
                r#east: false,
                r#attached: false,
            });
        }
        if state_id == 9434 {
            return Some(Tripwire {
                r#attached: true,
                r#west: true,
                r#north: true,
                r#disarmed: false,
                r#east: true,
                r#powered: false,
                r#south: true,
            });
        }
        if state_id == 9444 {
            return Some(Tripwire {
                r#attached: true,
                r#west: true,
                r#north: false,
                r#disarmed: false,
                r#east: true,
                r#powered: false,
                r#south: false,
            });
        }
        if state_id == 9470 {
            return Some(Tripwire {
                r#east: true,
                r#south: true,
                r#disarmed: true,
                r#powered: true,
                r#attached: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 9414 {
            return Some(Tripwire {
                r#west: true,
                r#north: true,
                r#disarmed: true,
                r#powered: true,
                r#east: false,
                r#attached: true,
                r#south: true,
            });
        }
        if state_id == 9492 {
            return Some(Tripwire {
                r#south: false,
                r#east: false,
                r#disarmed: true,
                r#attached: false,
                r#powered: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 9405 {
            return Some(Tripwire {
                r#attached: true,
                r#disarmed: true,
                r#east: true,
                r#north: true,
                r#powered: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 9514 {
            return Some(Tripwire {
                r#east: false,
                r#south: true,
                r#west: true,
                r#disarmed: false,
                r#powered: false,
                r#attached: false,
                r#north: true,
            });
        }
        if state_id == 9472 {
            return Some(Tripwire {
                r#east: true,
                r#powered: true,
                r#west: true,
                r#disarmed: true,
                r#attached: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 9429 {
            return Some(Tripwire {
                r#north: false,
                r#south: false,
                r#west: false,
                r#attached: true,
                r#disarmed: true,
                r#powered: false,
                r#east: false,
            });
        }
        if state_id == 9510 {
            return Some(Tripwire {
                r#east: false,
                r#north: true,
                r#powered: true,
                r#south: true,
                r#west: true,
                r#disarmed: false,
                r#attached: false,
            });
        }
        if state_id == 9469 {
            return Some(Tripwire {
                r#powered: false,
                r#attached: false,
                r#east: true,
                r#disarmed: true,
                r#north: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 9463 {
            return Some(Tripwire {
                r#attached: false,
                r#powered: true,
                r#west: false,
                r#east: true,
                r#south: true,
                r#north: true,
                r#disarmed: true,
            });
        }
        if state_id == 9489 {
            return Some(Tripwire {
                r#attached: false,
                r#north: false,
                r#powered: true,
                r#south: false,
                r#west: false,
                r#east: false,
                r#disarmed: true,
            });
        }
        if state_id == 9477 {
            return Some(Tripwire {
                r#west: false,
                r#north: false,
                r#powered: false,
                r#south: false,
                r#east: true,
                r#attached: false,
                r#disarmed: true,
            });
        }
        if state_id == 9519 {
            return Some(Tripwire {
                r#south: true,
                r#attached: false,
                r#disarmed: false,
                r#west: false,
                r#east: false,
                r#north: false,
                r#powered: true,
            });
        }
        if state_id == 9513 {
            return Some(Tripwire {
                r#powered: true,
                r#disarmed: false,
                r#south: false,
                r#west: false,
                r#attached: false,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 9449 {
            return Some(Tripwire {
                r#east: false,
                r#disarmed: false,
                r#west: false,
                r#north: true,
                r#south: false,
                r#powered: true,
                r#attached: true,
            });
        }
        if state_id == 9457 {
            return Some(Tripwire {
                r#west: false,
                r#disarmed: false,
                r#north: false,
                r#east: false,
                r#powered: true,
                r#south: false,
                r#attached: true,
            });
        }
        if state_id == 9461 {
            return Some(Tripwire {
                r#attached: true,
                r#west: false,
                r#disarmed: false,
                r#east: false,
                r#north: false,
                r#powered: false,
                r#south: false,
            });
        }
        if state_id == 9476 {
            return Some(Tripwire {
                r#north: false,
                r#powered: false,
                r#east: true,
                r#attached: false,
                r#south: false,
                r#west: true,
                r#disarmed: true,
            });
        }
        if state_id == 9483 {
            return Some(Tripwire {
                r#west: false,
                r#attached: false,
                r#powered: false,
                r#east: false,
                r#disarmed: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 9422 {
            return Some(Tripwire {
                r#north: false,
                r#east: false,
                r#powered: true,
                r#disarmed: true,
                r#attached: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 9486 {
            return Some(Tripwire {
                r#powered: true,
                r#west: true,
                r#north: false,
                r#south: true,
                r#attached: false,
                r#east: false,
                r#disarmed: true,
            });
        }
        if state_id == 9525 {
            return Some(Tripwire {
                r#east: false,
                r#north: false,
                r#powered: false,
                r#south: false,
                r#attached: false,
                r#disarmed: false,
                r#west: false,
            });
        }
        if state_id == 9400 {
            return Some(Tripwire {
                r#east: true,
                r#powered: true,
                r#attached: true,
                r#north: true,
                r#disarmed: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 9459 {
            return Some(Tripwire {
                r#east: false,
                r#disarmed: false,
                r#north: false,
                r#attached: true,
                r#powered: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 9450 {
            return Some(Tripwire {
                r#north: true,
                r#powered: false,
                r#attached: true,
                r#disarmed: false,
                r#east: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 9505 {
            return Some(Tripwire {
                r#south: false,
                r#west: false,
                r#east: true,
                r#disarmed: false,
                r#powered: true,
                r#north: false,
                r#attached: false,
            });
        }
        if state_id == 9442 {
            return Some(Tripwire {
                r#south: true,
                r#west: true,
                r#attached: true,
                r#disarmed: false,
                r#east: true,
                r#north: false,
                r#powered: false,
            });
        }
        if state_id == 9440 {
            return Some(Tripwire {
                r#powered: true,
                r#west: true,
                r#north: false,
                r#disarmed: false,
                r#east: true,
                r#attached: true,
                r#south: false,
            });
        }
        if state_id == 9512 {
            return Some(Tripwire {
                r#attached: false,
                r#north: true,
                r#south: false,
                r#west: true,
                r#east: false,
                r#powered: true,
                r#disarmed: false,
            });
        }
        if state_id == 9455 {
            return Some(Tripwire {
                r#powered: true,
                r#attached: true,
                r#south: true,
                r#disarmed: false,
                r#east: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 9499 {
            return Some(Tripwire {
                r#south: true,
                r#west: false,
                r#north: true,
                r#east: true,
                r#powered: false,
                r#attached: false,
                r#disarmed: false,
            });
        }
        if state_id == 9437 {
            return Some(Tripwire {
                r#east: true,
                r#attached: true,
                r#disarmed: false,
                r#west: false,
                r#north: true,
                r#powered: false,
                r#south: false,
            });
        }
        if state_id == 9407 {
            return Some(Tripwire {
                r#west: false,
                r#east: true,
                r#powered: true,
                r#north: false,
                r#disarmed: true,
                r#south: true,
                r#attached: true,
            });
        }
        if state_id == 9441 {
            return Some(Tripwire {
                r#south: false,
                r#west: false,
                r#attached: true,
                r#disarmed: false,
                r#east: true,
                r#north: false,
                r#powered: true,
            });
        }
        if state_id == 9479 {
            return Some(Tripwire {
                r#south: true,
                r#west: false,
                r#disarmed: true,
                r#attached: false,
                r#north: true,
                r#east: false,
                r#powered: true,
            });
        }
        if state_id == 9500 {
            return Some(Tripwire {
                r#south: false,
                r#north: true,
                r#disarmed: false,
                r#east: true,
                r#powered: false,
                r#attached: false,
                r#west: true,
            });
        }
        if state_id == 9453 {
            return Some(Tripwire {
                r#east: false,
                r#attached: true,
                r#north: true,
                r#west: false,
                r#disarmed: false,
                r#powered: false,
                r#south: false,
            });
        }
        if state_id == 9523 {
            return Some(Tripwire {
                r#west: false,
                r#east: false,
                r#north: false,
                r#powered: false,
                r#attached: false,
                r#disarmed: false,
                r#south: true,
            });
        }
        if state_id == 9473 {
            return Some(Tripwire {
                r#disarmed: true,
                r#west: false,
                r#powered: true,
                r#attached: false,
                r#east: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 9456 {
            return Some(Tripwire {
                r#powered: true,
                r#west: true,
                r#attached: true,
                r#disarmed: false,
                r#south: false,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 9494 {
            return Some(Tripwire {
                r#attached: false,
                r#north: true,
                r#powered: true,
                r#disarmed: false,
                r#south: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 9435 {
            return Some(Tripwire {
                r#east: true,
                r#powered: false,
                r#south: true,
                r#west: false,
                r#attached: true,
                r#north: true,
                r#disarmed: false,
            });
        }
        if state_id == 9462 {
            return Some(Tripwire {
                r#east: true,
                r#powered: true,
                r#west: true,
                r#south: true,
                r#attached: false,
                r#disarmed: true,
                r#north: true,
            });
        }
        if state_id == 9485 {
            return Some(Tripwire {
                r#disarmed: true,
                r#attached: false,
                r#north: true,
                r#powered: false,
                r#east: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 9504 {
            return Some(Tripwire {
                r#north: false,
                r#disarmed: false,
                r#attached: false,
                r#south: false,
                r#east: true,
                r#west: true,
                r#powered: true,
            });
        }
        if state_id == 9447 {
            return Some(Tripwire {
                r#disarmed: false,
                r#north: true,
                r#powered: true,
                r#west: false,
                r#south: true,
                r#attached: true,
                r#east: false,
            });
        }
        if state_id == 9452 {
            return Some(Tripwire {
                r#attached: true,
                r#east: false,
                r#powered: false,
                r#south: false,
                r#disarmed: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 9471 {
            return Some(Tripwire {
                r#west: false,
                r#north: false,
                r#south: true,
                r#disarmed: true,
                r#east: true,
                r#attached: false,
                r#powered: true,
            });
        }
        if state_id == 9520 {
            return Some(Tripwire {
                r#north: false,
                r#disarmed: false,
                r#powered: true,
                r#south: false,
                r#east: false,
                r#west: true,
                r#attached: false,
            });
        }
        if state_id == 9412 {
            return Some(Tripwire {
                r#south: false,
                r#powered: false,
                r#east: true,
                r#attached: true,
                r#north: false,
                r#west: true,
                r#disarmed: true,
            });
        }
        if state_id == 9421 {
            return Some(Tripwire {
                r#south: false,
                r#east: false,
                r#disarmed: true,
                r#west: false,
                r#attached: true,
                r#north: true,
                r#powered: false,
            });
        }
        if state_id == 9497 {
            return Some(Tripwire {
                r#south: false,
                r#west: false,
                r#powered: true,
                r#attached: false,
                r#north: true,
                r#disarmed: false,
                r#east: true,
            });
        }
        if state_id == 9484 {
            return Some(Tripwire {
                r#attached: false,
                r#north: true,
                r#east: false,
                r#powered: false,
                r#disarmed: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 9448 {
            return Some(Tripwire {
                r#west: true,
                r#disarmed: false,
                r#north: true,
                r#attached: true,
                r#powered: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 9509 {
            return Some(Tripwire {
                r#attached: false,
                r#disarmed: false,
                r#south: false,
                r#west: false,
                r#east: true,
                r#north: false,
                r#powered: false,
            });
        }
        if state_id == 9460 {
            return Some(Tripwire {
                r#east: false,
                r#powered: false,
                r#south: false,
                r#west: true,
                r#north: false,
                r#attached: true,
                r#disarmed: false,
            });
        }
        if state_id == 9416 {
            return Some(Tripwire {
                r#west: true,
                r#attached: true,
                r#east: false,
                r#north: true,
                r#disarmed: true,
                r#powered: true,
                r#south: false,
            });
        }
        if state_id == 9524 {
            return Some(Tripwire {
                r#attached: false,
                r#disarmed: false,
                r#east: false,
                r#north: false,
                r#powered: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 9481 {
            return Some(Tripwire {
                r#attached: false,
                r#north: true,
                r#powered: true,
                r#south: false,
                r#west: false,
                r#disarmed: true,
                r#east: false,
            });
        }
        if state_id == 9487 {
            return Some(Tripwire {
                r#east: false,
                r#attached: false,
                r#west: false,
                r#powered: true,
                r#disarmed: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 9398 {
            return Some(Tripwire {
                r#east: true,
                r#south: true,
                r#attached: true,
                r#north: true,
                r#west: true,
                r#powered: true,
                r#disarmed: true,
            });
        }
        if state_id == 9517 {
            return Some(Tripwire {
                r#north: true,
                r#south: false,
                r#attached: false,
                r#west: false,
                r#powered: false,
                r#east: false,
                r#disarmed: false,
            });
        }
        if state_id == 9480 {
            return Some(Tripwire {
                r#west: true,
                r#attached: false,
                r#disarmed: true,
                r#north: true,
                r#south: false,
                r#east: false,
                r#powered: true,
            });
        }
        if state_id == 9406 {
            return Some(Tripwire {
                r#attached: true,
                r#south: true,
                r#disarmed: true,
                r#north: false,
                r#west: true,
                r#powered: true,
                r#east: true,
            });
        }
        if state_id == 9427 {
            return Some(Tripwire {
                r#disarmed: true,
                r#powered: false,
                r#east: false,
                r#north: false,
                r#attached: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 9410 {
            return Some(Tripwire {
                r#south: true,
                r#north: false,
                r#west: true,
                r#east: true,
                r#disarmed: true,
                r#attached: true,
                r#powered: false,
            });
        }
        if state_id == 9401 {
            return Some(Tripwire {
                r#attached: true,
                r#disarmed: true,
                r#south: false,
                r#north: true,
                r#east: true,
                r#powered: true,
                r#west: false,
            });
        }
        return None;
    }
}

