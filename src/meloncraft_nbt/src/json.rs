//! Type conversions from [`JsonValue`] to [`NbtValue`] and back.
//! This is currently unused and untested in Meloncraft. It is intended for registry parsing and
//! code generation.

use crate::NbtCompound;
use crate::NbtF32;
use crate::NbtF64;
use crate::NbtI16;
use crate::NbtList;
use crate::NbtString;
use crate::NbtTag;
use crate::NbtU8;
use serde_json::value::Value as JsonValue;

use crate::NbtValue;

impl TryFrom<JsonValue> for NbtValue {
    type Error = ();

    fn try_from(value: serde_json::Value) -> Result<Self, ()> {
        return match value {
            JsonValue::Null => Err(()),
            JsonValue::Bool(val) => Ok(NbtValue::U8(if val { NbtU8(1) } else { NbtU8(0) })),
            JsonValue::String(val) => Ok(NbtValue::String(NbtString(val))),
            JsonValue::Number(val) => {
                if let Some(i) = val.as_i64() {
                    if i <= u8::MAX.into() && i >= u8::MIN.into() {
                        return Ok(NbtValue::U8(NbtU8(i.try_into().unwrap())));
                    }
                    if i <= i16::MAX.into() && i >= i16::MIN.into() {
                        return Ok(NbtValue::I16(NbtI16(i.try_into().unwrap())));
                    }
                    if i <= i32::MAX.into() && i >= i32::MIN.into() {
                        return Ok(NbtValue::I32(crate::NbtI32(i.try_into().unwrap())));
                    }
                    return Ok(NbtValue::I64(crate::NbtI64(i)));
                }
                if let Some(f) = val.as_f64() {
                    #[expect(clippy::as_conversions, clippy::cast_possible_truncation, reason = "This is what we're testing for: we want to see if it gets rounded / is out of bounds")]
                    if (f64::from(f as f32) - f) <= 0.000_000_1 {
                        // convert to f32 if possible without a loss of precision
                        return Ok(NbtValue::F32(NbtF32(f as f32)));
                    }
                    return Ok(NbtValue::F64(NbtF64(f)));
                }
                Err(())
            }
            JsonValue::Array(vals) => {
                let mut nbt_vals = Vec::new();
                for val in vals {
                    nbt_vals.push(NbtValue::try_from(val)?);
                }
                return Ok(NbtValue::List(NbtList(nbt_vals)));
            }
            JsonValue::Object(obj) => {
                let mut children = Vec::new();
                for (key, val) in obj {
                    children.push(NbtTag {
                        key: key.clone(),
                        value: NbtValue::try_from(val)?,
                    });
                }
                return Ok(NbtValue::Compound(NbtCompound(children)));
            }
        };
    }
}

#[expect(clippy::fallible_impl_from, reason = "This code is not used currently anyway. But all unwraps should be infallible anyway, just due to conversions.")]
impl From<NbtValue> for JsonValue {
    fn from(value: NbtValue) -> Self {
        match value {
            NbtValue::U8(nbt_u8) => return JsonValue::Number(serde_json::Number::from(nbt_u8.0)),
            NbtValue::I16(nbt_i16) => return JsonValue::Number(serde_json::Number::from(*nbt_i16)),
            NbtValue::I32(nbt_i32) => return JsonValue::Number(serde_json::Number::from(*nbt_i32)),
            NbtValue::I64(nbt_i64) => return JsonValue::Number(serde_json::Number::from(*nbt_i64)),
            NbtValue::F32(nbt_f32) => return JsonValue::Number(serde_json::Number::from_f64((*nbt_f32).into()).unwrap()),
            NbtValue::F64(nbt_f64) => return JsonValue::Number(serde_json::Number::from_f64(*nbt_f64).unwrap()),
            NbtValue::String(nbt_string) => return JsonValue::String((*nbt_string).clone()),
            NbtValue::ArrayU8(nbt_array_u8) => {
                let vals: Vec<JsonValue> = nbt_array_u8
                    .iter()
                    .map(|v| return JsonValue::Number(serde_json::Number::from(*v)))
                    .collect();
                return JsonValue::Array(vals);
            }
            NbtValue::ArrayI32(nbt_array_i32) => {
                let vals: Vec<JsonValue> = nbt_array_i32
                    .iter()
                    .map(|v| return JsonValue::Number(serde_json::Number::from(*v)))
                    .collect();
                return JsonValue::Array(vals);
            }
            NbtValue::ArrayI64(nbt_array_i64) => {
                let vals: Vec<JsonValue> = nbt_array_i64
                    .iter()
                    .map(|v| return JsonValue::Number(serde_json::Number::from(*v)))
                    .collect();
                return JsonValue::Array(vals);
            }
            NbtValue::List(nbt_list) => {
                let vals: Vec<JsonValue> = nbt_list
                    .iter()
                    .map(|v| return JsonValue::from(v.clone()))
                    .collect();
                return JsonValue::Array(vals);
            }
            NbtValue::Compound(nbt_compound) => {
                let mut map = serde_json::Map::new();
                for tag in nbt_compound.iter() {
                    map.insert(tag.key.clone(), JsonValue::from(tag.value.clone()));
                }
                return JsonValue::Object(map);
            }
        }
    }
}

/*#[cfg(test)]
position tests {
    use super::*;
    use serde_json::Map;
    #[test, dis]
    fn nbt_to_json_and_back() {
        let original_nbt = NbtValue::Compound(NbtCompound(vec![
            NbtTag {
                key: "name".to_string(),
                value: NbtValue::String(NbtString("Example".to_string())),
            },
            NbtTag {
                key: "age".to_string(),
                value: NbtValue::I32(crate::NbtI32(25)),
            },
            NbtTag {
                key: "is_student".to_string(),
                value: NbtValue::U8(NbtU8(0)),
            },
            NbtTag {
                key: "scores".to_string(),
                value: NbtValue::ArrayU8(NbtArrayU8(vec![85, 90, 78])),
            },
        ]));
        let json_value: JsonValue = JsonValue::from(original_nbt.clone());
        let converted_nbt = NbtValue::try_from(json_value).unwrap();
        assert_eq!(original_nbt, converted_nbt);
    }

    #[test]
    fn json_to_nbt_and_back() {
        let mut json_map = Map::new();

        let mut object_1 = Map::new();
        object_1.insert("name".to_string(), JsonValue::String("Example".to_string()));
        object_1.insert(
            "age".to_string(),
            JsonValue::Number(serde_json::Number::from(25)),
        );
        object_1.insert("is_student".to_string(), JsonValue::Bool(false));

        let mut object_2 = Map::new();
        object_2.insert(
            "scores".to_string(),
            JsonValue::Array(vec![
                JsonValue::Number(serde_json::Number::from(85)),
                JsonValue::Number(serde_json::Number::from(90)),
                JsonValue::Number(serde_json::Number::from(78)),
            ]),
        );

        json_map.insert("person".to_string(), JsonValue::Object(object_1));
        json_map.insert("details".to_string(), JsonValue::Object(object_2));

        let original_json = JsonValue::Object(json_map);

        let nbt_value = NbtValue::try_from(original_json.clone()).unwrap();
        let converted_json = JsonValue::from(nbt_value);
        assert_eq!(original_json, converted_json);
    }
}*/
