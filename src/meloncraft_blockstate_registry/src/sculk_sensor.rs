use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SculkSensor {
    pub power: i32,
    pub r#sculk_sensor_phase: SculkSensorPhase,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SculkSensorPhase {
    Inactive,
    Active,
    Cooldown,
}

impl BlockState for SculkSensor {
    fn to_id(&self) -> i32 {
        if self.r#power == 13
            && self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
        {
            return 24570;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 14
            && self.r#waterlogged == true
        {
            return 24574;
        }
        if self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#power == 5
        {
            return 24518;
        }
        if self.r#waterlogged == true
            && self.r#power == 1
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
        {
            return 24494;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#power == 5
            && self.r#waterlogged == false
        {
            return 24519;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == true
            && self.r#power == 12
        {
            return 24564;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == false
            && self.r#power == 3
        {
            return 24511;
        }
        if self.r#power == 7
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == false
        {
            return 24533;
        }
        if self.r#power == 2
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == true
        {
            return 24504;
        }
        if self.r#power == 10
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
        {
            return 24548;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#power == 9
            && self.r#waterlogged == false
        {
            return 24543;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#power == 2
            && self.r#waterlogged == false
        {
            return 24501;
        }
        if self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#power == 14
        {
            return 24573;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == true
            && self.r#power == 10
        {
            return 24550;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == false
            && self.r#power == 11
        {
            return 24559;
        }
        if self.r#power == 10
            && self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
        {
            return 24552;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == false
            && self.r#power == 2
        {
            return 24505;
        }
        if self.r#power == 3
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
        {
            return 24506;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 12
            && self.r#waterlogged == false
        {
            return 24563;
        }
        if self.r#power == 4
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
        {
            return 24512;
        }
        if self.r#power == 4
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == true
        {
            return 24514;
        }
        if self.r#power == 15
            && self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
        {
            return 24580;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 15
            && self.r#waterlogged == false
        {
            return 24581;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 8
            && self.r#waterlogged == false
        {
            return 24541;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == false
            && self.r#power == 15
        {
            return 24579;
        }
        if self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 3
        {
            return 24508;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#power == 0
            && self.r#waterlogged == false
        {
            return 24489;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 1
            && self.r#waterlogged == true
        {
            return 24498;
        }
        if self.r#power == 1
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == false
        {
            return 24499;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == false
            && self.r#power == 6
        {
            return 24529;
        }
        if self.r#power == 3
            && self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
        {
            return 24509;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#power == 7
            && self.r#waterlogged == true
        {
            return 24530;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == true
            && self.r#power == 7
        {
            return 24532;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 6
            && self.r#waterlogged == false
        {
            return 24527;
        }
        if self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 8
        {
            return 24538;
        }
        if self.r#power == 9
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
        {
            return 24542;
        }
        if self.r#power == 11
            && self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
        {
            return 24558;
        }
        if self.r#power == 14
            && self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
        {
            return 24576;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
            && self.r#power == 15
        {
            return 24578;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 5
            && self.r#waterlogged == false
        {
            return 24521;
        }
        if self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 0
        {
            return 24493;
        }
        if self.r#power == 0
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == true
        {
            return 24492;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#power == 10
            && self.r#waterlogged == false
        {
            return 24549;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#power == 8
            && self.r#waterlogged == true
        {
            return 24536;
        }
        if self.r#power == 15
            && self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
        {
            return 24583;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 14
            && self.r#waterlogged == false
        {
            return 24575;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 4
            && self.r#waterlogged == true
        {
            return 24516;
        }
        if self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 7
        {
            return 24534;
        }
        if self.r#power == 12
            && self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
        {
            return 24560;
        }
        if self.r#power == 6
            && self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
        {
            return 24526;
        }
        if self.r#power == 0
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == false
        {
            return 24491;
        }
        if self.r#waterlogged == true
            && self.r#power == 2
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
        {
            return 24502;
        }
        if self.r#power == 8
            && self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
        {
            return 24540;
        }
        if self.r#power == 5
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == false
        {
            return 24523;
        }
        if self.r#waterlogged == true
            && self.r#power == 0
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
        {
            return 24490;
        }
        if self.r#power == 13
            && self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
        {
            return 24567;
        }
        if self.r#power == 1
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == false
        {
            return 24495;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == false
            && self.r#power == 2
        {
            return 24503;
        }
        if self.r#waterlogged == true
            && self.r#power == 14
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
        {
            return 24572;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == false
            && self.r#power == 4
        {
            return 24517;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == true
            && self.r#power == 12
        {
            return 24562;
        }
        if self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 12
        {
            return 24565;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 14
            && self.r#waterlogged == false
        {
            return 24577;
        }
        if self.r#waterlogged == true
            && self.r#power == 15
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
        {
            return 24582;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == true
            && self.r#power == 9
        {
            return 24544;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 10
            && self.r#waterlogged == false
        {
            return 24551;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
            && self.r#power == 6
        {
            return 24524;
        }
        if self.r#power == 10
            && self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
        {
            return 24553;
        }
        if self.r#power == 11
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == false
        {
            return 24555;
        }
        if self.r#power == 4
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == false
        {
            return 24513;
        }
        if self.r#power == 0
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
        {
            return 24488;
        }
        if self.r#power == 9
            && self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
        {
            return 24545;
        }
        if self.r#power == 11
            && self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
        {
            return 24557;
        }
        if self.r#power == 13
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
        {
            return 24566;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == false
            && self.r#power == 1
        {
            return 24497;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == true
            && self.r#power == 5
        {
            return 24522;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 9
            && self.r#waterlogged == true
        {
            return 24546;
        }
        if self.r#waterlogged == false
            && self.r#power == 7
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
        {
            return 24531;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 13
            && self.r#waterlogged == false
        {
            return 24571;
        }
        if self.r#waterlogged == false
            && self.r#power == 4
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
        {
            return 24515;
        }
        if self.r#power == 2
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
        {
            return 24500;
        }
        if self.r#waterlogged == false
            && self.r#power == 3
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
        {
            return 24507;
        }
        if self.r#waterlogged == false
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 9
        {
            return 24547;
        }
        if self.r#power == 3
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#waterlogged == true
        {
            return 24510;
        }
        if self.r#power == 1
            && self.r#waterlogged == true
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
        {
            return 24496;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
            && self.r#power == 7
            && self.r#waterlogged == false
        {
            return 24535;
        }
        if self.r#power == 11
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == true
        {
            return 24554;
        }
        if self.r#power == 11
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == true
        {
            return 24556;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 5
            && self.r#waterlogged == true
        {
            return 24520;
        }
        if self.r#waterlogged == true
            && self.r#power == 6
            && self.r#sculk_sensor_phase == SculkSensorPhase::Cooldown
        {
            return 24528;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == false
            && self.r#power == 8
        {
            return 24537;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#power == 8
            && self.r#waterlogged == false
        {
            return 24539;
        }
        if self.r#power == 12
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
            && self.r#waterlogged == false
        {
            return 24561;
        }
        if self.r#power == 13
            && self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == true
        {
            return 24568;
        }
        if self.r#waterlogged == false
            && self.r#power == 6
            && self.r#sculk_sensor_phase == SculkSensorPhase::Inactive
        {
            return 24525;
        }
        if self.r#sculk_sensor_phase == SculkSensorPhase::Active
            && self.r#waterlogged == false
            && self.r#power == 13
        {
            return 24569;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24570 {
            return Some(SculkSensor {
                r#power: 13,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24574 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 24518 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 5,
            });
        }
        if state_id == 24494 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24519 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 24564 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#power: 12,
            });
        }
        if state_id == 24511 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
                r#power: 3,
            });
        }
        if state_id == 24533 {
            return Some(SculkSensor {
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
            });
        }
        if state_id == 24504 {
            return Some(SculkSensor {
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24548 {
            return Some(SculkSensor {
                r#power: 10,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24543 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 24501 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 24573 {
            return Some(SculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 14,
            });
        }
        if state_id == 24550 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#power: 10,
            });
        }
        if state_id == 24559 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
                r#power: 11,
            });
        }
        if state_id == 24552 {
            return Some(SculkSensor {
                r#power: 10,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24505 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
                r#power: 2,
            });
        }
        if state_id == 24506 {
            return Some(SculkSensor {
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24563 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 24512 {
            return Some(SculkSensor {
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24514 {
            return Some(SculkSensor {
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
            });
        }
        if state_id == 24580 {
            return Some(SculkSensor {
                r#power: 15,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24581 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 24541 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 24579 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#power: 15,
            });
        }
        if state_id == 24508 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 3,
            });
        }
        if state_id == 24489 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 24498 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 24499 {
            return Some(SculkSensor {
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
            });
        }
        if state_id == 24529 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
                r#power: 6,
            });
        }
        if state_id == 24509 {
            return Some(SculkSensor {
                r#power: 3,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24530 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 24532 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#power: 7,
            });
        }
        if state_id == 24527 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 6,
                r#waterlogged: false,
            });
        }
        if state_id == 24538 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 8,
            });
        }
        if state_id == 24542 {
            return Some(SculkSensor {
                r#power: 9,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24558 {
            return Some(SculkSensor {
                r#power: 11,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24576 {
            return Some(SculkSensor {
                r#power: 14,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24578 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#power: 15,
            });
        }
        if state_id == 24521 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 24493 {
            return Some(SculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 0,
            });
        }
        if state_id == 24492 {
            return Some(SculkSensor {
                r#power: 0,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24549 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 24536 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#power: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 24583 {
            return Some(SculkSensor {
                r#power: 15,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24575 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 24516 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 24534 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 7,
            });
        }
        if state_id == 24560 {
            return Some(SculkSensor {
                r#power: 12,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24526 {
            return Some(SculkSensor {
                r#power: 6,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24491 {
            return Some(SculkSensor {
                r#power: 0,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
            });
        }
        if state_id == 24502 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24540 {
            return Some(SculkSensor {
                r#power: 8,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24523 {
            return Some(SculkSensor {
                r#power: 5,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
            });
        }
        if state_id == 24490 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#power: 0,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24567 {
            return Some(SculkSensor {
                r#power: 13,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24495 {
            return Some(SculkSensor {
                r#power: 1,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
            });
        }
        if state_id == 24503 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 2,
            });
        }
        if state_id == 24572 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#power: 14,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24517 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: false,
                r#power: 4,
            });
        }
        if state_id == 24562 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#power: 12,
            });
        }
        if state_id == 24565 {
            return Some(SculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 12,
            });
        }
        if state_id == 24577 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 24582 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#power: 15,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24544 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
                r#power: 9,
            });
        }
        if state_id == 24551 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 24524 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
                r#power: 6,
            });
        }
        if state_id == 24553 {
            return Some(SculkSensor {
                r#power: 10,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24555 {
            return Some(SculkSensor {
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
            });
        }
        if state_id == 24513 {
            return Some(SculkSensor {
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
            });
        }
        if state_id == 24488 {
            return Some(SculkSensor {
                r#power: 0,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24545 {
            return Some(SculkSensor {
                r#power: 9,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24557 {
            return Some(SculkSensor {
                r#power: 11,
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24566 {
            return Some(SculkSensor {
                r#power: 13,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24497 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 1,
            });
        }
        if state_id == 24522 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
                r#power: 5,
            });
        }
        if state_id == 24546 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 24531 {
            return Some(SculkSensor {
                r#waterlogged: false,
                r#power: 7,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24571 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 24515 {
            return Some(SculkSensor {
                r#waterlogged: false,
                r#power: 4,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24500 {
            return Some(SculkSensor {
                r#power: 2,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24507 {
            return Some(SculkSensor {
                r#waterlogged: false,
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24547 {
            return Some(SculkSensor {
                r#waterlogged: false,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 9,
            });
        }
        if state_id == 24510 {
            return Some(SculkSensor {
                r#power: 3,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#waterlogged: true,
            });
        }
        if state_id == 24496 {
            return Some(SculkSensor {
                r#power: 1,
                r#waterlogged: true,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
            });
        }
        if state_id == 24535 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
                r#power: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 24554 {
            return Some(SculkSensor {
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: true,
            });
        }
        if state_id == 24556 {
            return Some(SculkSensor {
                r#power: 11,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
            });
        }
        if state_id == 24520 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 24528 {
            return Some(SculkSensor {
                r#waterlogged: true,
                r#power: 6,
                r#sculk_sensor_phase: SculkSensorPhase::Cooldown,
            });
        }
        if state_id == 24537 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
                r#power: 8,
            });
        }
        if state_id == 24539 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#power: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 24561 {
            return Some(SculkSensor {
                r#power: 12,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
                r#waterlogged: false,
            });
        }
        if state_id == 24568 {
            return Some(SculkSensor {
                r#power: 13,
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: true,
            });
        }
        if state_id == 24525 {
            return Some(SculkSensor {
                r#waterlogged: false,
                r#power: 6,
                r#sculk_sensor_phase: SculkSensorPhase::Inactive,
            });
        }
        if state_id == 24569 {
            return Some(SculkSensor {
                r#sculk_sensor_phase: SculkSensorPhase::Active,
                r#waterlogged: false,
                r#power: 13,
            });
        }
        return None;
    }
}
