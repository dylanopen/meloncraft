/// Represents the intensity of rain or thunder in the game world.
/// 
/// ## Fields
/// 0. The intensity of the rain or thunder, as an `f32` value between 0.0 and 1.0.
///    0.0 means no rain/thunder, 1.0 means full intensity.
///
/// ## Packet usage
/// Used in some variants of the `ClientboundGameEvent` packet to represent the intensity of rain or
/// thunder in the game world. See that packet's documentation for more information.
///
/// ## Constraints
/// - The value of `WeatherIntensity` must be between `0.0` and `1.0`, inclusive. Values outside
///   this range may cause packets to fail to send or be ignored by the client.
#[derive(Debug, Clone)]
pub struct WeatherIntensity(pub f32);

