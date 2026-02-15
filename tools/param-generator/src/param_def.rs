//! Deserializable paramdef structures as provided by the Smithbox XML.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "PARAMDEF")]
pub struct ParamDef {
    #[serde(rename = "ParamType")]
    pub param_type: String,
    #[serde(rename = "Index")]
    pub index: Option<usize>,
    #[serde(rename = "Fields")]
    pub fields: Fields,
}

#[derive(Debug, Deserialize)]
pub struct Fields {
    #[serde(rename = "Field")]
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Deserialize)]
pub struct FieldDef {
    #[serde(rename = "@Def")]
    pub def: String,
    #[serde(rename = "@RemovedVersion")]
    pub removed_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FieldType {
    Bitfield(u8),
    Standard(String),
    Array(Box<FieldType>, usize),
}

impl FieldType {
    pub fn alignment_and_size(&self) -> (usize, usize) {
        match self {
            FieldType::Bitfield(_) => (1, 1),
            FieldType::Standard(ty) => match ty.as_str() {
                "u8" | "s8" | "dummy8" | "fixstr" => (1, 1),
                "u16" | "s16" | "fixstrW" => (2, 2),
                "u32" | "b32" | "angle32" | "s32" | "f32" => (4, 4),
                _ => panic!("Unknown type: {ty}"),
            },
            FieldType::Array(inner_type, repetitions) => (
                inner_type.alignment_and_size().0,
                inner_type.alignment_and_size().1 * repetitions,
            ),
        }
    }

    pub fn native_type(&self) -> &str {
        match self {
            FieldType::Standard(ty) => match ty.as_str() {
                "u8" | "dummy8" | "fixstr" => "u8",
                "s8" => "i8",
                "u16" | "fixstrW" => "u16",
                "s16" => "i16",
                "u32" | "b32" => "u32",
                "f32" | "angle32" => "f32",
                "s32" => "i32",
                _ => panic!("Unknown type: {ty}"),
            },
            _ => unimplemented!(),
        }
    }

    pub fn c_type(&self) -> &str {
        match self {
            FieldType::Standard(ty) => match ty.as_str() {
                "u8" | "dummy8" | "fixstr" => "uint8_t",
                "s8" => "int8_t",
                "u16" | "fixstrW" => "uint16_t",
                "s16" => "int16_t",
                "u32" | "b32" => "uint32_t",
                "f32" | "angle32" => "float",
                "s32" => "int32_t",
                _ => panic!("Unknown type: {ty}"),
            },
            _ => unimplemented!(),
        }
    }
}
