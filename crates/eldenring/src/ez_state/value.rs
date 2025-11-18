use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
/// An argument or return value for an ESD event or query, represented as a safe enum. Use
/// from()/into() to convert between this and the raw `EzStateRawValue` representation.
pub enum EzStateValue {
    Float32(f32),
    Int32(i32),
    Unk64(u64),
}

impl From<EzStateValue> for f32 {
    fn from(value: EzStateValue) -> Self {
        match value {
            EzStateValue::Float32(float) => float,
            EzStateValue::Int32(int) => int as f32,
            _ => 0f32,
        }
    }
}

impl From<EzStateValue> for i32 {
    fn from(value: EzStateValue) -> Self {
        match value {
            EzStateValue::Float32(float) => float as i32,
            EzStateValue::Int32(int) => int,
            _ => 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
union EzStateRawValueValue {
    float32: f32,
    int32: i32,
    unk64: u64,
}

#[repr(i32)]
#[derive(Clone, Copy)]
enum EzStateRawValueType {
    Float32 = 1,
    Int32 = 2,
    Unk64 = 3,
}

impl Default for EzStateRawValue {
    fn default() -> Self {
        EzStateValue::Int32(0).into()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EzStateRawValue {
    value: EzStateRawValueValue,
    value_type: EzStateRawValueType,
}

impl Debug for EzStateRawValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        EzStateValue::from(*self).fmt(f)
    }
}

impl From<EzStateRawValue> for EzStateValue {
    fn from(raw_value: EzStateRawValue) -> Self {
        match raw_value.value_type {
            // Safety: can only be constucted by `From<EzStateValue>::from` which maintains the
            // invariant that `value` is the type specified by `value_type`, or the game, which
            // also holds this invariant.
            EzStateRawValueType::Float32 => Self::Float32(unsafe { raw_value.value.float32 }),
            EzStateRawValueType::Int32 => Self::Int32(unsafe { raw_value.value.int32 }),
            EzStateRawValueType::Unk64 => Self::Unk64(unsafe { raw_value.value.unk64 }),
        }
    }
}

impl From<EzStateValue> for EzStateRawValue {
    fn from(value: EzStateValue) -> Self {
        match value {
            EzStateValue::Float32(float32) => EzStateRawValue {
                value: EzStateRawValueValue { float32 },
                value_type: EzStateRawValueType::Float32,
            },
            EzStateValue::Int32(int32) => EzStateRawValue {
                value: EzStateRawValueValue { int32 },
                value_type: EzStateRawValueType::Int32,
            },
            EzStateValue::Unk64(unk64) => EzStateRawValue {
                value: EzStateRawValueValue { unk64 },
                value_type: EzStateRawValueType::Unk64,
            },
        }
    }
}
