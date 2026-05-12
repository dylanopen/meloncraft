use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibratedSculkSensor {
    pub power: i32,
    pub r#facing: Facing,
    pub r#sculk_sensor_phase: SculkSensorPhase,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SculkSensorPhase {
    Inactive,
    Active,
    Cooldown,
}

impl BlockState for CalibratedSculkSensor {
    fn to_id(self) -> i32 {
        if block_state.r#power == 14 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 24861; }
        if block_state.r#facing == Facing::East && block_state.r#power == 1 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false { return 24883; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 6 && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 24908; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::East && block_state.r#power == 3 && block_state.r#waterlogged == false { return 24895; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North && block_state.r#power == 9 && block_state.r#waterlogged == true { return 24642; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#power == 12 { return 24753; }
        if block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 6 && block_state.r#waterlogged == true { return 24812; }
        if block_state.r#facing == Facing::South && block_state.r#power == 13 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24760; }
        if block_state.r#power == 11 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::South { return 24749; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#power == 10 { return 24644; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 0 && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 24776; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 6 && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24623; }
        if block_state.r#facing == Facing::West && block_state.r#power == 7 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24821; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 10 { return 24935; }
        if block_state.r#power == 8 && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24925; }
        if block_state.r#power == 7 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false { return 24727; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#power == 13 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24955; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 6 { return 24622; }
        if block_state.r#power == 5 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24810; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 5 { return 24614; }
        if block_state.r#power == 6 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::East { return 24910; }
        if block_state.r#waterlogged == false && block_state.r#power == 13 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24855; }
        if block_state.r#waterlogged == false && block_state.r#power == 11 && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24939; }
        if block_state.r#waterlogged == true && block_state.r#power == 5 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North { return 24618; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#power == 1 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24785; }
        if block_state.r#power == 3 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 24894; }
        if block_state.r#power == 12 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false { return 24849; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#power == 15 { return 24867; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#power == 15 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24679; }
        if block_state.r#power == 14 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 24958; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#power == 11 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24750; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24697; }
        if block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 24887; }
        if block_state.r#power == 10 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 24841; }
        if block_state.r#power == 9 && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24930; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#power == 15 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24870; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 9 && block_state.r#waterlogged == true { return 24638; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#power == 1 { return 24593; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 5 && block_state.r#waterlogged == false { return 24615; }
        if block_state.r#power == 13 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false { return 24759; }
        if block_state.r#facing == Facing::South && block_state.r#power == 14 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true { return 24764; }
        if block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 24889; }
        if block_state.r#facing == Facing::North && block_state.r#power == 6 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false { return 24625; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#power == 14 { return 24863; }
        if block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 24885; }
        if block_state.r#facing == Facing::East && block_state.r#power == 4 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true { return 24896; }
        if block_state.r#facing == Facing::West && block_state.r#power == 10 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24839; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#power == 11 { return 24844; }
        if block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 12 && block_state.r#waterlogged == true { return 24948; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true && block_state.r#power == 15 && block_state.r#facing == Facing::South { return 24770; }
        if block_state.r#facing == Facing::West && block_state.r#power == 11 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24843; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 7 && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 24918; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South && block_state.r#power == 5 && block_state.r#waterlogged == true { return 24714; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 6 && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 24913; }
        if block_state.r#facing == Facing::West && block_state.r#power == 14 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24864; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#power == 13 { return 24950; }
        if block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 2 && block_state.r#waterlogged == true { return 24692; }
        if block_state.r#power == 6 && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24912; }
        if block_state.r#waterlogged == false && block_state.r#power == 5 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South { return 24711; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#power == 4 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24705; }
        if block_state.r#power == 7 && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24823; }
        if block_state.r#power == 15 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true { return 24866; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 4 && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 24897; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East && block_state.r#power == 5 && block_state.r#waterlogged == false { return 24903; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East && block_state.r#power == 7 && block_state.r#waterlogged == true { return 24914; }
        if block_state.r#facing == Facing::North && block_state.r#power == 11 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24654; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#power == 0 { return 24685; }
        if block_state.r#power == 12 && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24660; }
        if block_state.r#power == 0 && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24779; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 1 { return 24689; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#power == 12 { return 24850; }
        if block_state.r#power == 0 && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24584; }
        if block_state.r#waterlogged == true && block_state.r#power == 0 && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24876; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 7 && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 24916; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#power == 6 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24621; }
        if block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 1 && block_state.r#facing == Facing::West { return 24787; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 11 && block_state.r#facing == Facing::South { return 24748; }
        if block_state.r#power == 12 && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false { return 24949; }
        if block_state.r#power == 2 && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24601; }
        if block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 6 && block_state.r#waterlogged == true { return 24716; }
        if block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 24726; }
        if block_state.r#power == 1 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24591; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#power == 4 { return 24609; }
        if block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 24626; }
        if block_state.r#waterlogged == true && block_state.r#power == 7 && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24628; }
        if block_state.r#power == 11 && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false { return 24655; }
        if block_state.r#waterlogged == true && block_state.r#power == 15 && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24676; }
        if block_state.r#facing == Facing::South && block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24696; }
        if block_state.r#power == 6 && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24720; }
        if block_state.r#power == 12 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South { return 24752; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#power == 3 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24606; }
        if block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24643; }
        if block_state.r#power == 0 && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24781; }
        if block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::West && block_state.r#power == 3 { return 24795; }
        if block_state.r#power == 1 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 24881; }
        if block_state.r#facing == Facing::East && block_state.r#power == 1 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24882; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#power == 5 && block_state.r#facing == Facing::East { return 24904; }
        if block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 24927; }
        if block_state.r#power == 0 && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24872; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 7 && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 24917; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#power == 10 { return 24936; }
        if block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 10 && block_state.r#waterlogged == false { return 24937; }
        if block_state.r#power == 3 && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24700; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true && block_state.r#power == 12 && block_state.r#facing == Facing::West { return 24848; }
        if block_state.r#waterlogged == true && block_state.r#power == 10 && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24648; }
        if block_state.r#power == 8 && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24824; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::East && block_state.r#power == 11 && block_state.r#waterlogged == false { return 24941; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#power == 14 { return 24956; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#power == 15 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24964; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East && block_state.r#power == 6 && block_state.r#waterlogged == false { return 24909; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 13 { return 24666; }
        if block_state.r#power == 8 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false { return 24825; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 2 { return 24790; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North && block_state.r#power == 10 && block_state.r#waterlogged == false { return 24649; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North && block_state.r#power == 14 { return 24672; }
        if block_state.r#power == 7 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North { return 24627; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 1 { return 24880; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#power == 3 && block_state.r#facing == Facing::East { return 24891; }
        if block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 24919; }
        if block_state.r#power == 9 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East { return 24926; }
        if block_state.r#power == 14 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 24765; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#power == 11 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24652; }
        if block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 24629; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#power == 0 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24777; }
        if block_state.r#waterlogged == false && block_state.r#power == 1 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24687; }
        if block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::West && block_state.r#power == 3 { return 24799; }
        if block_state.r#waterlogged == false && block_state.r#power == 5 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::West { return 24811; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 7 { return 24819; }
        if block_state.r#power == 9 && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24833; }
        if block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 1 && block_state.r#waterlogged == true { return 24782; }
        if block_state.r#power == 12 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24852; }
        if block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24639; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::North && block_state.r#power == 13 && block_state.r#waterlogged == false { return 24665; }
        if block_state.r#facing == Facing::South && block_state.r#power == 12 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24754; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#power == 15 { return 24774; }
        if block_state.r#power == 0 && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24778; }
        if block_state.r#power == 10 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 24645; }
        if block_state.r#power == 1 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 24688; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South && block_state.r#power == 9 && block_state.r#waterlogged == true { return 24738; }
        if block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 8 && block_state.r#waterlogged == true { return 24920; }
        if block_state.r#power == 2 && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24791; }
        if block_state.r#power == 8 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 24923; }
        if block_state.r#power == 2 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false { return 24695; }
        if block_state.r#power == 11 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 24942; }
        if block_state.r#waterlogged == true && block_state.r#power == 4 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::South { return 24706; }
        if block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 9 && block_state.r#waterlogged == true { return 24832; }
        if block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 14 && block_state.r#waterlogged == true { return 24862; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::West && block_state.r#power == 13 && block_state.r#waterlogged == false { return 24859; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#power == 5 { return 24806; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 14 && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24673; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24599; }
        if block_state.r#power == 10 && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24836; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 5 && block_state.r#facing == Facing::North { return 24616; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#power == 6 { return 24721; }
        if block_state.r#power == 4 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 24709; }
        if block_state.r#waterlogged == true && block_state.r#power == 12 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North { return 24656; }
        if block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 24834; }
        if block_state.r#waterlogged == true && block_state.r#power == 3 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East { return 24890; }
        if block_state.r#waterlogged == false && block_state.r#power == 12 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::North { return 24659; }
        if block_state.r#power == 15 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 24965; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North && block_state.r#power == 15 && block_state.r#waterlogged == true { return 24678; }
        if block_state.r#facing == Facing::South && block_state.r#power == 13 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24761; }
        if block_state.r#facing == Facing::South && block_state.r#power == 14 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true { return 24766; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#power == 6 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24816; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 14 { return 24769; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East && block_state.r#power == 2 { return 24884; }
        if block_state.r#power == 8 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South { return 24733; }
        if block_state.r#facing == Facing::East && block_state.r#power == 8 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24924; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#power == 10 { return 24840; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North && block_state.r#power == 6 && block_state.r#waterlogged == true { return 24620; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#power == 13 { return 24763; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 0 { return 24877; }
        if block_state.r#facing == Facing::North && block_state.r#power == 15 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false { return 24677; }
        if block_state.r#waterlogged == true && block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North { return 24630; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#power == 4 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24803; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 3 && block_state.r#waterlogged == true { return 24602; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#power == 4 && block_state.r#facing == Facing::North { return 24610; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::West && block_state.r#power == 5 { return 24808; }
        if block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 13 && block_state.r#facing == Facing::North { return 24663; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::West && block_state.r#power == 11 && block_state.r#waterlogged == false { return 24845; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 2 { return 24693; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North && block_state.r#power == 15 { return 24674; }
        if block_state.r#power == 10 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 24934; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#power == 14 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24671; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#power == 12 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24756; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::West && block_state.r#power == 4 { return 24802; }
        if block_state.r#power == 3 && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24702; }
        if block_state.r#power == 0 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 24684; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true && block_state.r#power == 11 && block_state.r#facing == Facing::South { return 24746; }
        if block_state.r#power == 7 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24822; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#power == 13 { return 24664; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#power == 3 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24892; }
        if block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 3 && block_state.r#waterlogged == true { return 24794; }
        if block_state.r#power == 1 && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24878; }
        if block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 3 && block_state.r#waterlogged == false { return 24701; }
        if block_state.r#waterlogged == true && block_state.r#power == 14 && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24670; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::South && block_state.r#power == 15 && block_state.r#waterlogged == true { return 24772; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 15 && block_state.r#facing == Facing::East { return 24966; }
        if block_state.r#power == 5 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true { return 24710; }
        if block_state.r#facing == Facing::North && block_state.r#power == 13 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24662; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South && block_state.r#power == 1 { return 24686; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#power == 12 && block_state.r#facing == Facing::South { return 24755; }
        if block_state.r#power == 1 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 24783; }
        if block_state.r#waterlogged == true && block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::West { return 24818; }
        if block_state.r#facing == Facing::East && block_state.r#power == 13 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false { return 24951; }
        if block_state.r#power == 0 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 24874; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#power == 8 { return 24634; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 4 && block_state.r#facing == Facing::East { return 24900; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24835; }
        if block_state.r#waterlogged == true && block_state.r#power == 8 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24828; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#power == 11 { return 24651; }
        if block_state.r#power == 5 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24617; }
        if block_state.r#power == 1 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 24879; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 12 && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 24757; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 0 && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24587; }
        if block_state.r#facing == Facing::West && block_state.r#power == 15 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true { return 24868; }
        if block_state.r#power == 0 && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24873; }
        if block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 8 && block_state.r#facing == Facing::North { return 24633; }
        if block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#power == 3 { return 24699; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 4 { return 24612; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#power == 13 { return 24953; }
        if block_state.r#power == 5 && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24713; }
        if block_state.r#power == 11 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 24938; }
        if block_state.r#facing == Facing::South && block_state.r#power == 4 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24707; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 15 && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 24771; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 6 { return 24718; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24789; }
        if block_state.r#facing == Facing::West && block_state.r#power == 10 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24837; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 14 { return 24961; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true && block_state.r#power == 1 { return 24594; }
        if block_state.r#waterlogged == false && block_state.r#power == 6 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South { return 24717; }
        if block_state.r#waterlogged == true && block_state.r#power == 4 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24708; }
        if block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 8 && block_state.r#waterlogged == false { return 24921; }
        if block_state.r#power == 8 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true { return 24826; }
        if block_state.r#power == 2 && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24886; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#power == 5 { return 24807; }
        if block_state.r#facing == Facing::West && block_state.r#power == 1 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24786; }
        if block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true && block_state.r#power == 14 { return 24860; }
        if block_state.r#facing == Facing::East && block_state.r#power == 10 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true { return 24932; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24737; }
        if block_state.r#power == 6 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::South { return 24719; }
        if block_state.r#waterlogged == false && block_state.r#power == 15 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East { return 24963; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 0 && block_state.r#waterlogged == false { return 24585; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 8 { return 24731; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#power == 11 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24653; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 0 { return 24780; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 2 && block_state.r#facing == Facing::East { return 24888; }
        if block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 9 && block_state.r#waterlogged == true { return 24928; }
        if block_state.r#waterlogged == true && block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::West { return 24820; }
        if block_state.r#power == 5 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 24712; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 10 { return 24745; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#power == 14 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24669; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 14 { return 24959; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::North && block_state.r#power == 1 && block_state.r#waterlogged == true { return 24592; }
        if block_state.r#power == 1 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true { return 24784; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South && block_state.r#power == 8 { return 24728; }
        if block_state.r#facing == Facing::West && block_state.r#power == 3 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false { return 24797; }
        if block_state.r#power == 11 && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24846; }
        if block_state.r#waterlogged == false && block_state.r#power == 10 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24743; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::North && block_state.r#power == 2 && block_state.r#waterlogged == true { return 24598; }
        if block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 24734; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#power == 3 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24603; }
        if block_state.r#power == 12 && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24658; }
        if block_state.r#waterlogged == true && block_state.r#power == 6 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::West { return 24814; }
        if block_state.r#power == 1 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24690; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#power == 11 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24751; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#power == 4 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24804; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::East && block_state.r#power == 9 && block_state.r#waterlogged == false { return 24931; }
        if block_state.r#facing == Facing::West && block_state.r#power == 12 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false { return 24851; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 2 && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 24793; }
        if block_state.r#facing == Facing::South && block_state.r#power == 15 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false { return 24775; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North && block_state.r#power == 2 && block_state.r#waterlogged == true { return 24600; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 13 { return 24857; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#power == 13 { return 24858; }
        if block_state.r#power == 5 && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24907; }
        if block_state.r#power == 11 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 24940; }
        if block_state.r#power == 5 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North { return 24619; }
        if block_state.r#facing == Facing::East && block_state.r#power == 6 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false { return 24911; }
        if block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#power == 10 { return 24742; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#power == 4 { return 24608; }
        if block_state.r#waterlogged == false && block_state.r#power == 6 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24817; }
        if block_state.r#power == 12 && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false { return 24947; }
        if block_state.r#power == 8 && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24632; }
        if block_state.r#power == 8 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true { return 24730; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#power == 4 && block_state.r#facing == Facing::North { return 24611; }
        if block_state.r#waterlogged == false && block_state.r#power == 8 && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24637; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#power == 3 { return 24607; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 10 { return 24741; }
        if block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#power == 4 { return 24898; }
        if block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South && block_state.r#power == 1 { return 24691; }
        if block_state.r#facing == Facing::East && block_state.r#power == 15 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24962; }
        if block_state.r#facing == Facing::North && block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == true { return 24596; }
        if block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::South && block_state.r#power == 7 { return 24725; }
        if block_state.r#power == 8 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 24922; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::West && block_state.r#power == 4 && block_state.r#waterlogged == true { return 24800; }
        if block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 15 && block_state.r#waterlogged == false { return 24871; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#power == 4 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24613; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::East && block_state.r#power == 15 && block_state.r#waterlogged == false { return 24967; }
        if block_state.r#power == 7 && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24631; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South && block_state.r#power == 0 && block_state.r#waterlogged == true { return 24680; }
        if block_state.r#power == 9 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South { return 24739; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 3 && block_state.r#facing == Facing::West { return 24796; }
        if block_state.r#facing == Facing::South && block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false { return 24723; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24694; }
        if block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 24722; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#power == 10 { return 24740; }
        if block_state.r#power == 2 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24792; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#power == 13 { return 24952; }
        if block_state.r#waterlogged == false && block_state.r#power == 9 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24831; }
        if block_state.r#power == 13 && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24954; }
        if block_state.r#power == 11 && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24847; }
        if block_state.r#power == 12 && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24945; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#power == 3 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24605; }
        if block_state.r#facing == Facing::West && block_state.r#power == 12 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false { return 24853; }
        if block_state.r#power == 8 && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == true { return 24636; }
        if block_state.r#waterlogged == false && block_state.r#power == 0 && block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24681; }
        if block_state.r#waterlogged == true && block_state.r#power == 1 && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24590; }
        if block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24641; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#power == 5 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24809; }
        if block_state.r#power == 3 && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24798; }
        if block_state.r#power == 7 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 24724; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 1 && block_state.r#waterlogged == false { return 24595; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::West && block_state.r#power == 11 { return 24842; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 14 && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 24668; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 14 && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 24865; }
        if block_state.r#waterlogged == false && block_state.r#power == 13 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North { return 24667; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East && block_state.r#power == 12 { return 24944; }
        if block_state.r#facing == Facing::South && block_state.r#power == 0 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24682; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#power == 0 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24586; }
        if block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 24736; }
        if block_state.r#power == 9 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 24929; }
        if block_state.r#power == 11 && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24650; }
        if block_state.r#power == 5 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::East { return 24906; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#power == 12 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24946; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 11 && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 24747; }
        if block_state.r#power == 13 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South { return 24758; }
        if block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#power == 14 { return 24767; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South && block_state.r#power == 4 { return 24704; }
        if block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#power == 0 { return 24875; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 3 && block_state.r#facing == Facing::North { return 24604; }
        if block_state.r#waterlogged == true && block_state.r#power == 5 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::East { return 24902; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::North && block_state.r#power == 12 && block_state.r#waterlogged == false { return 24657; }
        if block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 14 && block_state.r#waterlogged == true { return 24768; }
        if block_state.r#power == 3 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South { return 24698; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24788; }
        if block_state.r#waterlogged == false && block_state.r#power == 4 && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24899; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 10 && block_state.r#waterlogged == true { return 24646; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 3 && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 24703; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::South && block_state.r#power == 10 && block_state.r#waterlogged == true { return 24744; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#power == 6 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24813; }
        if block_state.r#power == 10 && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24933; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 5 { return 24905; }
        if block_state.r#waterlogged == false && block_state.r#power == 4 && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24805; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 9 && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 24735; }
        if block_state.r#power == 6 && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24815; }
        if block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 15 && block_state.r#facing == Facing::West { return 24869; }
        if block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 7 && block_state.r#waterlogged == false { return 24915; }
        if block_state.r#facing == Facing::South && block_state.r#power == 5 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24715; }
        if block_state.r#power == 8 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#facing == Facing::South { return 24729; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#power == 8 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24732; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::North && block_state.r#power == 8 && block_state.r#waterlogged == false { return 24635; }
        if block_state.r#power == 13 && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24854; }
        if block_state.r#waterlogged == true && block_state.r#power == 10 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::West { return 24838; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#power == 15 { return 24675; }
        if block_state.r#facing == Facing::South && block_state.r#power == 13 && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24762; }
        if block_state.r#facing == Facing::East && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#power == 14 { return 24957; }
        if block_state.r#facing == Facing::South && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#power == 0 && block_state.r#waterlogged == false { return 24683; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#power == 6 { return 24624; }
        if block_state.r#facing == Facing::North && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#power == 10 { return 24647; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#power == 3 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24893; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#waterlogged == false && block_state.r#power == 15 && block_state.r#facing == Facing::South { return 24773; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#power == 8 { return 24827; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#waterlogged == false && block_state.r#power == 4 && block_state.r#facing == Facing::East { return 24901; }
        if block_state.r#power == 4 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 24801; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#power == 11 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24943; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 0 { return 24588; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#power == 14 && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 24960; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive && block_state.r#power == 9 && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 24830; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#power == 2 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Inactive { return 24597; }
        if block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#power == 0 { return 24589; }
        if block_state.r#waterlogged == true && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active && block_state.r#facing == Facing::North && block_state.r#power == 9 { return 24640; }
        if block_state.r#facing == Facing::West && block_state.r#power == 8 && block_state.r#waterlogged == false && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24829; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#power == 12 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Cooldown { return 24661; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#power == 13 && block_state.r#sculk_sensor_phase == SculkSensorPhase::Active { return 24856; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24861 {
            return Some(CalibratedSculkSensor {
                r#power: 14,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 24883 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
            });
        }
        if state_id == 24908 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 6,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24895 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::East,
                r#power: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 24642 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
                r#power: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 24753 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#power: 12,
            });
        }
        if state_id == 24812 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 24760 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 13,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24749 {
            return Some(CalibratedSculkSensor {
                r#power: 11,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::South,
            });
        }
        if state_id == 24644 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#power: 10,
            });
        }
        if state_id == 24776 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 0,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 24623 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 6,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24821 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 7,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24935 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 10,
            });
        }
        if state_id == 24925 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24727 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
            });
        }
        if state_id == 24955 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#power: 13,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24622 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 6,
            });
        }
        if state_id == 24810 {
            return Some(CalibratedSculkSensor {
                r#power: 5,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24614 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 5,
            });
        }
        if state_id == 24910 {
            return Some(CalibratedSculkSensor {
                r#power: 6,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::East,
            });
        }
        if state_id == 24855 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 13,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24939 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 11,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24618 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 5,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
            });
        }
        if state_id == 24785 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24894 {
            return Some(CalibratedSculkSensor {
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24849 {
            return Some(CalibratedSculkSensor {
                r#power: 12,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
            });
        }
        if state_id == 24867 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#power: 15,
            });
        }
        if state_id == 24679 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#power: 15,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24958 {
            return Some(CalibratedSculkSensor {
                r#power: 14,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24750 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24697 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24887 {
            return Some(CalibratedSculkSensor {
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 24841 {
            return Some(CalibratedSculkSensor {
                r#power: 10,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 24930 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24870 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#power: 15,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24638 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 24593 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 1,
            });
        }
        if state_id == 24615 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 24759 {
            return Some(CalibratedSculkSensor {
                r#power: 13,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
            });
        }
        if state_id == 24764 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 14,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24889 {
            return Some(CalibratedSculkSensor {
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 24625 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#power: 6,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
            });
        }
        if state_id == 24863 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#power: 14,
            });
        }
        if state_id == 24885 {
            return Some(CalibratedSculkSensor {
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 24896 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24839 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 10,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24844 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#power: 11,
            });
        }
        if state_id == 24948 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 24770 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#power: 15,
                r#facing: Facing::South,
            });
        }
        if state_id == 24843 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 11,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24918 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 7,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 24714 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
                r#power: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 24913 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 6,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 24864 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 14,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24950 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#power: 13,
            });
        }
        if state_id == 24692 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 24912 {
            return Some(CalibratedSculkSensor {
                r#power: 6,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24711 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 5,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
            });
        }
        if state_id == 24705 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24823 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24866 {
            return Some(CalibratedSculkSensor {
                r#power: 15,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24897 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 4,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 24903 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
                r#power: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 24914 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
                r#power: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 24654 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#power: 11,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24685 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#power: 0,
            });
        }
        if state_id == 24660 {
            return Some(CalibratedSculkSensor {
                r#power: 12,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24779 {
            return Some(CalibratedSculkSensor {
                r#power: 0,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24689 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 1,
            });
        }
        if state_id == 24850 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#power: 12,
            });
        }
        if state_id == 24584 {
            return Some(CalibratedSculkSensor {
                r#power: 0,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24876 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 0,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24916 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 7,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 24621 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#power: 6,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24787 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 1,
                r#facing: Facing::West,
            });
        }
        if state_id == 24748 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 11,
                r#facing: Facing::South,
            });
        }
        if state_id == 24949 {
            return Some(CalibratedSculkSensor {
                r#power: 12,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
            });
        }
        if state_id == 24601 {
            return Some(CalibratedSculkSensor {
                r#power: 2,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24716 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 24726 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 24591 {
            return Some(CalibratedSculkSensor {
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24609 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#power: 4,
            });
        }
        if state_id == 24626 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 24628 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 7,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24655 {
            return Some(CalibratedSculkSensor {
                r#power: 11,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
            });
        }
        if state_id == 24676 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 15,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24696 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24720 {
            return Some(CalibratedSculkSensor {
                r#power: 6,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24752 {
            return Some(CalibratedSculkSensor {
                r#power: 12,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
            });
        }
        if state_id == 24606 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24643 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24781 {
            return Some(CalibratedSculkSensor {
                r#power: 0,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24795 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::West,
                r#power: 3,
            });
        }
        if state_id == 24881 {
            return Some(CalibratedSculkSensor {
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 24882 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24904 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#power: 5,
                r#facing: Facing::East,
            });
        }
        if state_id == 24927 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 24872 {
            return Some(CalibratedSculkSensor {
                r#power: 0,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24917 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 7,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 24936 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#power: 10,
            });
        }
        if state_id == 24937 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 24700 {
            return Some(CalibratedSculkSensor {
                r#power: 3,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24848 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#power: 12,
                r#facing: Facing::West,
            });
        }
        if state_id == 24648 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 10,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24824 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24941 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::East,
                r#power: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 24956 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#power: 14,
            });
        }
        if state_id == 24964 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#power: 15,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24909 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
                r#power: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 24666 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 13,
            });
        }
        if state_id == 24825 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
            });
        }
        if state_id == 24790 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 2,
            });
        }
        if state_id == 24649 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
                r#power: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 24672 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
                r#power: 14,
            });
        }
        if state_id == 24627 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
            });
        }
        if state_id == 24880 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 1,
            });
        }
        if state_id == 24891 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#power: 3,
                r#facing: Facing::East,
            });
        }
        if state_id == 24919 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 24926 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
            });
        }
        if state_id == 24765 {
            return Some(CalibratedSculkSensor {
                r#power: 14,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 24652 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24629 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 24777 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#power: 0,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24687 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 1,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24799 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::West,
                r#power: 3,
            });
        }
        if state_id == 24811 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 5,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::West,
            });
        }
        if state_id == 24819 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 7,
            });
        }
        if state_id == 24833 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24782 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 24852 {
            return Some(CalibratedSculkSensor {
                r#power: 12,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24639 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24665 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::North,
                r#power: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 24754 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 12,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24774 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#power: 15,
            });
        }
        if state_id == 24778 {
            return Some(CalibratedSculkSensor {
                r#power: 0,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24645 {
            return Some(CalibratedSculkSensor {
                r#power: 10,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 24688 {
            return Some(CalibratedSculkSensor {
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 24738 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
                r#power: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 24920 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 24791 {
            return Some(CalibratedSculkSensor {
                r#power: 2,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24923 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 24695 {
            return Some(CalibratedSculkSensor {
                r#power: 2,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
            });
        }
        if state_id == 24942 {
            return Some(CalibratedSculkSensor {
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24706 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::South,
            });
        }
        if state_id == 24832 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 24862 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 24859 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::West,
                r#power: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 24806 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#power: 5,
            });
        }
        if state_id == 24673 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 14,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24599 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24836 {
            return Some(CalibratedSculkSensor {
                r#power: 10,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24616 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 5,
                r#facing: Facing::North,
            });
        }
        if state_id == 24721 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#power: 6,
            });
        }
        if state_id == 24709 {
            return Some(CalibratedSculkSensor {
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 24656 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 12,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
            });
        }
        if state_id == 24834 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 24890 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
            });
        }
        if state_id == 24659 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 12,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::North,
            });
        }
        if state_id == 24965 {
            return Some(CalibratedSculkSensor {
                r#power: 15,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 24678 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
                r#power: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 24761 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 13,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24766 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 14,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
            });
        }
        if state_id == 24816 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#power: 6,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24769 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 14,
            });
        }
        if state_id == 24884 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
                r#power: 2,
            });
        }
        if state_id == 24733 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
            });
        }
        if state_id == 24924 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#power: 8,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24840 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#power: 10,
            });
        }
        if state_id == 24620 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
                r#power: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 24763 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#power: 13,
            });
        }
        if state_id == 24877 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 0,
            });
        }
        if state_id == 24677 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#power: 15,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
            });
        }
        if state_id == 24630 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
            });
        }
        if state_id == 24803 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24602 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 24610 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#power: 4,
                r#facing: Facing::North,
            });
        }
        if state_id == 24808 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::West,
                r#power: 5,
            });
        }
        if state_id == 24663 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 13,
                r#facing: Facing::North,
            });
        }
        if state_id == 24845 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::West,
                r#power: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 24693 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 2,
            });
        }
        if state_id == 24674 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
                r#power: 15,
            });
        }
        if state_id == 24934 {
            return Some(CalibratedSculkSensor {
                r#power: 10,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24671 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#power: 14,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24756 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#power: 12,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24802 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::West,
                r#power: 4,
            });
        }
        if state_id == 24702 {
            return Some(CalibratedSculkSensor {
                r#power: 3,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24684 {
            return Some(CalibratedSculkSensor {
                r#power: 0,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 24746 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#power: 11,
                r#facing: Facing::South,
            });
        }
        if state_id == 24822 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24664 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#power: 13,
            });
        }
        if state_id == 24892 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24794 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 24878 {
            return Some(CalibratedSculkSensor {
                r#power: 1,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24701 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 24670 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 14,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24772 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::South,
                r#power: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 24966 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 15,
                r#facing: Facing::East,
            });
        }
        if state_id == 24710 {
            return Some(CalibratedSculkSensor {
                r#power: 5,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24662 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#power: 13,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24686 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
                r#power: 1,
            });
        }
        if state_id == 24755 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 12,
                r#facing: Facing::South,
            });
        }
        if state_id == 24783 {
            return Some(CalibratedSculkSensor {
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 24818 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::West,
            });
        }
        if state_id == 24951 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#power: 13,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
            });
        }
        if state_id == 24874 {
            return Some(CalibratedSculkSensor {
                r#power: 0,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 24634 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#power: 8,
            });
        }
        if state_id == 24900 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 4,
                r#facing: Facing::East,
            });
        }
        if state_id == 24835 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24828 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 8,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24651 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#power: 11,
            });
        }
        if state_id == 24617 {
            return Some(CalibratedSculkSensor {
                r#power: 5,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24879 {
            return Some(CalibratedSculkSensor {
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 24757 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 12,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 24587 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 0,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24868 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 15,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
            });
        }
        if state_id == 24873 {
            return Some(CalibratedSculkSensor {
                r#power: 0,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24633 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 8,
                r#facing: Facing::North,
            });
        }
        if state_id == 24699 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#power: 3,
            });
        }
        if state_id == 24612 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 4,
            });
        }
        if state_id == 24953 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#power: 13,
            });
        }
        if state_id == 24713 {
            return Some(CalibratedSculkSensor {
                r#power: 5,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24938 {
            return Some(CalibratedSculkSensor {
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24707 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 4,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24771 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 15,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 24718 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 6,
            });
        }
        if state_id == 24789 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24837 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 10,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24961 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 14,
            });
        }
        if state_id == 24594 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#power: 1,
            });
        }
        if state_id == 24717 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 6,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
            });
        }
        if state_id == 24708 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 4,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24921 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 24826 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
            });
        }
        if state_id == 24886 {
            return Some(CalibratedSculkSensor {
                r#power: 2,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24807 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#power: 5,
            });
        }
        if state_id == 24786 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24860 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#power: 14,
            });
        }
        if state_id == 24932 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#power: 10,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24737 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24719 {
            return Some(CalibratedSculkSensor {
                r#power: 6,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::South,
            });
        }
        if state_id == 24963 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 15,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
            });
        }
        if state_id == 24585 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 24731 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 8,
            });
        }
        if state_id == 24653 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24780 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 0,
            });
        }
        if state_id == 24888 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 2,
                r#facing: Facing::East,
            });
        }
        if state_id == 24928 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 24820 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::West,
            });
        }
        if state_id == 24712 {
            return Some(CalibratedSculkSensor {
                r#power: 5,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 24745 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 10,
            });
        }
        if state_id == 24669 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#power: 14,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24959 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 14,
            });
        }
        if state_id == 24592 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::North,
                r#power: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 24784 {
            return Some(CalibratedSculkSensor {
                r#power: 1,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
            });
        }
        if state_id == 24728 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
                r#power: 8,
            });
        }
        if state_id == 24797 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
            });
        }
        if state_id == 24846 {
            return Some(CalibratedSculkSensor {
                r#power: 11,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24743 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 10,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24598 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::North,
                r#power: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 24734 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 24603 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24658 {
            return Some(CalibratedSculkSensor {
                r#power: 12,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24814 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 6,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::West,
            });
        }
        if state_id == 24690 {
            return Some(CalibratedSculkSensor {
                r#power: 1,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24751 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24804 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24931 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::East,
                r#power: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 24851 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 12,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
            });
        }
        if state_id == 24793 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 2,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 24775 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 15,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
            });
        }
        if state_id == 24600 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
                r#power: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 24857 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 13,
            });
        }
        if state_id == 24858 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#power: 13,
            });
        }
        if state_id == 24907 {
            return Some(CalibratedSculkSensor {
                r#power: 5,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24940 {
            return Some(CalibratedSculkSensor {
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 24619 {
            return Some(CalibratedSculkSensor {
                r#power: 5,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
            });
        }
        if state_id == 24911 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#power: 6,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
            });
        }
        if state_id == 24742 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#power: 10,
            });
        }
        if state_id == 24608 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#power: 4,
            });
        }
        if state_id == 24817 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 6,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24947 {
            return Some(CalibratedSculkSensor {
                r#power: 12,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
            });
        }
        if state_id == 24632 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24730 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
            });
        }
        if state_id == 24611 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 4,
                r#facing: Facing::North,
            });
        }
        if state_id == 24637 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 8,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24607 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#power: 3,
            });
        }
        if state_id == 24741 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 10,
            });
        }
        if state_id == 24898 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#power: 4,
            });
        }
        if state_id == 24691 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
                r#power: 1,
            });
        }
        if state_id == 24962 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#power: 15,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24596 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24725 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::South,
                r#power: 7,
            });
        }
        if state_id == 24922 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24800 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::West,
                r#power: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 24871 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 24613 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24967 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::East,
                r#power: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 24631 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24680 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
                r#power: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 24739 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
            });
        }
        if state_id == 24796 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 3,
                r#facing: Facing::West,
            });
        }
        if state_id == 24723 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
            });
        }
        if state_id == 24694 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24722 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 24740 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#power: 10,
            });
        }
        if state_id == 24792 {
            return Some(CalibratedSculkSensor {
                r#power: 2,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24952 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#power: 13,
            });
        }
        if state_id == 24831 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 9,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24954 {
            return Some(CalibratedSculkSensor {
                r#power: 13,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24847 {
            return Some(CalibratedSculkSensor {
                r#power: 11,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24945 {
            return Some(CalibratedSculkSensor {
                r#power: 12,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24605 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24853 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 12,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
            });
        }
        if state_id == 24636 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24681 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 0,
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24590 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 1,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24641 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24809 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#power: 5,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24798 {
            return Some(CalibratedSculkSensor {
                r#power: 3,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24724 {
            return Some(CalibratedSculkSensor {
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 24595 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 24842 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::West,
                r#power: 11,
            });
        }
        if state_id == 24668 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 14,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 24865 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 14,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 24667 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 13,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
            });
        }
        if state_id == 24944 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
                r#power: 12,
            });
        }
        if state_id == 24682 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 0,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24586 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#power: 0,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24736 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 24929 {
            return Some(CalibratedSculkSensor {
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 24650 {
            return Some(CalibratedSculkSensor {
                r#power: 11,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24906 {
            return Some(CalibratedSculkSensor {
                r#power: 5,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::East,
            });
        }
        if state_id == 24946 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#power: 12,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24747 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 11,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 24758 {
            return Some(CalibratedSculkSensor {
                r#power: 13,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
            });
        }
        if state_id == 24767 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 14,
            });
        }
        if state_id == 24704 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
                r#power: 4,
            });
        }
        if state_id == 24875 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 0,
            });
        }
        if state_id == 24604 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 3,
                r#facing: Facing::North,
            });
        }
        if state_id == 24902 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 5,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::East,
            });
        }
        if state_id == 24657 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::North,
                r#power: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 24768 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 24698 {
            return Some(CalibratedSculkSensor {
                r#power: 3,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
            });
        }
        if state_id == 24788 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24899 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 4,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24646 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 24703 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 3,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 24744 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::South,
                r#power: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 24813 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#power: 6,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24933 {
            return Some(CalibratedSculkSensor {
                r#power: 10,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24905 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 5,
            });
        }
        if state_id == 24805 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#power: 4,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24735 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 9,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 24815 {
            return Some(CalibratedSculkSensor {
                r#power: 6,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24869 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 15,
                r#facing: Facing::West,
            });
        }
        if state_id == 24915 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 24715 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 5,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24729 {
            return Some(CalibratedSculkSensor {
                r#power: 8,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#facing: Facing::South,
            });
        }
        if state_id == 24732 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#power: 8,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24635 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::North,
                r#power: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 24854 {
            return Some(CalibratedSculkSensor {
                r#power: 13,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24838 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#power: 10,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::West,
            });
        }
        if state_id == 24675 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#power: 15,
            });
        }
        if state_id == 24762 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#power: 13,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24957 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::East,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#power: 14,
            });
        }
        if state_id == 24683 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::South,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 24624 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#power: 6,
            });
        }
        if state_id == 24647 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 10,
            });
        }
        if state_id == 24893 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24773 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 15,
                r#facing: Facing::South,
            });
        }
        if state_id == 24827 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#power: 8,
            });
        }
        if state_id == 24901 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
                r#power: 4,
                r#facing: Facing::East,
            });
        }
        if state_id == 24801 {
            return Some(CalibratedSculkSensor {
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 24943 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24588 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 0,
            });
        }
        if state_id == 24960 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 14,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24830 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 9,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 24597 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24589 {
            return Some(CalibratedSculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#power: 0,
            });
        }
        if state_id == 24640 {
            return Some(CalibratedSculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#facing: Facing::North,
                r#power: 9,
            });
        }
        if state_id == 24829 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#power: 8,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24661 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#power: 12,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24856 {
            return Some(CalibratedSculkSensor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#power: 13,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        return None;
    }
}

