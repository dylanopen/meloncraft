use crate::NbtArrayU8;
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
        match value {
            JsonValue::Null => Err(()),
            JsonValue::Bool(val) => Ok(NbtValue::U8(if val { NbtU8(1) } else { NbtU8(0) })),
            JsonValue::String(val) => Ok(NbtValue::String(NbtString(val))),
            JsonValue::Number(val) => {
                if let Some(i) = val.as_i64() {
                    if i <= u8::MAX as i64 && i >= u8::MIN as i64 {
                        return Ok(NbtValue::U8(NbtU8(i as u8)));
                    }
                    if i <= i16::MAX as i64 && i >= i16::MIN as i64 {
                        return Ok(NbtValue::I16(NbtI16(i as i16)));
                    }
                    if i <= i32::MAX as i64 && i >= i32::MIN as i64 {
                        return Ok(NbtValue::I32(crate::NbtI32(i as i32)));
                    }
                    return Ok(NbtValue::I64(crate::NbtI64(i)));
                }
                if let Some(f) = val.as_f64() {
                    if f as f32 as f64 == f {
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

                let mut u8_vals = Vec::new();
                for nbt_val in &nbt_vals {
                    if let NbtValue::U8(u8_val) = nbt_val {
                        u8_vals.push(**u8_val);
                    } else {
                        break;
                    }
                }
                if u8_vals.len() == nbt_vals.len() {
                    return Ok(NbtValue::ArrayU8(NbtArrayU8(u8_vals)));
                }

                let mut i32_vals = Vec::new();
                for nbt_val in &nbt_vals {
                    if let NbtValue::I32(i32_val) = nbt_val {
                        i32_vals.push(**i32_val);
                    } else {
                        break;
                    }
                }
                if i32_vals.len() == nbt_vals.len() {
                    return Ok(NbtValue::ArrayI32(crate::NbtArrayI32(i32_vals)));
                }

                let mut i64_vals = Vec::new();
                for nbt_val in &nbt_vals {
                    if let NbtValue::I64(i64_val) = nbt_val {
                        i64_vals.push(**i64_val);
                    } else {
                        break;
                    }
                }
                if i64_vals.len() == nbt_vals.len() {
                    return Ok(NbtValue::ArrayI64(crate::NbtArrayI64(i64_vals)));
                }

                Ok(NbtValue::List(NbtList(nbt_vals)))
            }
            JsonValue::Object(obj) => {
                let mut children = Vec::new();
                for (key, val) in obj {
                    children.push(NbtTag {
                        key: key.clone(),
                        value: NbtValue::try_from(val)?,
                    });
                }
                Ok(NbtValue::Compound(NbtCompound(children)))
            }
        }
    }
}

impl From<NbtValue> for JsonValue {
    fn from(value: NbtValue) -> Self {
        match value {
            NbtValue::U8(nbt_u8) => JsonValue::Bool(*nbt_u8 != 0),
            NbtValue::I16(nbt_i16) => JsonValue::Number(serde_json::Number::from(*nbt_i16)),
            NbtValue::I32(nbt_i32) => JsonValue::Number(serde_json::Number::from(*nbt_i32)),
            NbtValue::I64(nbt_i64) => JsonValue::Number(serde_json::Number::from(*nbt_i64)),
            NbtValue::F32(nbt_f32) => {
                JsonValue::Number(serde_json::Number::from_f64(*nbt_f32 as f64).unwrap())
            }
            NbtValue::F64(nbt_f64) => {
                JsonValue::Number(serde_json::Number::from_f64(*nbt_f64).unwrap())
            }
            NbtValue::String(nbt_string) => JsonValue::String((*nbt_string).clone()),
            NbtValue::ArrayU8(nbt_array_u8) => {
                let vals: Vec<JsonValue> = nbt_array_u8.iter().map(|v| JsonValue::Number(serde_json::Number::from(*v))).collect();
                JsonValue::Array(vals)
            }
            NbtValue::ArrayI32(nbt_array_i32) => {
                let vals: Vec<JsonValue> = nbt_array_i32.iter().map(|v| JsonValue::Number(serde_json::Number::from(*v))).collect();
                JsonValue::Array(vals)
            }
            NbtValue::ArrayI64(nbt_array_i64) => {
                let vals: Vec<JsonValue> = nbt_array_i64.iter().map(|v| JsonValue::Number(serde_json::Number::from(*v))).collect();
                JsonValue::Array(vals)
            }
            NbtValue::List(nbt_list) => {
                let vals: Vec<JsonValue> = nbt_list.iter().map(|v| JsonValue::from(v.clone())).collect();
                JsonValue::Array(vals)
            }
            NbtValue::Compound(nbt_compound) => {
                let mut map = serde_json::Map::new();
                for tag in nbt_compound.iter() {
                    map.insert(tag.key.clone(), JsonValue::from(tag.value.clone()));
                }
                JsonValue::Object(map)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nbt_to_json_and_back() {
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
}

