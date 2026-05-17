use bevy::ecs::component::Component;


#[derive(Component, Debug, Clone)]
pub struct EntityPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub flags: EntityPositionFlags,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityPositionFlags {
    pub on_ground: bool,
    pub pushing_against_wall: bool,
}

impl From<u8> for EntityPositionFlags {
    fn from(value: u8) -> Self {
        return Self {
            on_ground: value & 0b_0000_0001 != 0,
            pushing_against_wall: value & 0b_0000_0010 != 0,
        }
    }
}

impl From<EntityPositionFlags> for u8 {
    fn from(value: EntityPositionFlags) -> Self {
        let mut result = 0;
        if value.on_ground {
            result |= 0b_0000_0001;
        }
        if value.pushing_against_wall {
            result |= 0b_0000_0010;
        }
        return result;
    }
}
