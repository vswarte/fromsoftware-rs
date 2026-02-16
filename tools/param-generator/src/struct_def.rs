//! Our own struct definition, post-processed from the paramdefs to make them
//! more accurate to how they're laid out in an actual langauge.

use crate::{FieldDef, FieldType, ParamDef};

#[derive(Debug)]
pub struct StructDef {
    pub name: String,
    pub index: Option<usize>,
    pub layout: Vec<LayoutUnit>,
}

#[derive(Debug)]
pub struct LayoutField {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug)]
pub struct LayoutUnit {
    pub name: String,
    pub offset: usize,
    pub size: usize,
    pub field_type: FieldType,
}

impl LayoutUnit {
    pub fn hidden(&self) -> bool {
        if FieldType::Standard("dummy8".to_string()) == self.field_type {
            return true;
        }

        if let FieldType::Array(inner, _) = &self.field_type
            && FieldType::Standard("dummy8".to_string()) == **inner
        {
            return true;
        }

        let lower = self.name.to_lowercase();
        lower.contains("reserve") || lower.starts_with("pad") || lower.starts_with("unk")
    }
}

impl From<&ParamDef> for StructDef {
    fn from(parsed: &ParamDef) -> Self {
        let fields = parsed
            .fields
            .fields
            .iter()
            .filter(|e| e.removed_version.is_none())
            .map(|f| parse_field(f).unwrap())
            .collect::<Vec<_>>();

        let layout = layout_struct(&fields);

        Self {
            name: parsed.param_type.clone(),
            index: parsed.index,
            layout,
        }
    }
}

fn layout_struct(fields: &[LayoutField]) -> Vec<LayoutUnit> {
    let mut offset = 0;
    let mut layout = Vec::new();
    let mut bit_cursor: Option<(usize, u8)> = None;

    for field in fields {
        let (alignment, size) = field.field_type.alignment_and_size();

        match &field.field_type {
            FieldType::Bitfield(bits) => {
                // Either get the current bit cursor or start a new one
                let (byte_offset, used_bits) = bit_cursor.unwrap_or((offset, 0));

                // Wrap around the 8 bit boundaries.
                if used_bits + bits > 8 {
                    offset += 1;
                    bit_cursor = Some((offset, *bits));
                } else {
                    bit_cursor = Some((byte_offset, used_bits + bits));
                }

                layout.push(LayoutUnit {
                    name: field.name.clone(),
                    offset,
                    size,
                    field_type: field.field_type.clone(),
                });

                // Yeet bit cursor and advance offset if we get to the end of a byte.
                if let Some((_, used)) = bit_cursor
                    && used == 8
                {
                    offset += 1;
                    bit_cursor = None;
                }
            }
            FieldType::Standard(_) | FieldType::Array(_, _) => {
                // Clean bit cursor if we're in the middle of a byte and need to enforce alignment.
                if bit_cursor.is_some() {
                    offset += 1;
                    bit_cursor = None;
                }

                // Align to current types alignment.
                offset = align_offset(offset, alignment);
                layout.push(LayoutUnit {
                    name: field.name.clone(),
                    offset,
                    size,
                    field_type: field.field_type.clone(),
                });
                offset += size;
            }
        }
    }

    layout
}

fn parse_field(field: &FieldDef) -> Option<LayoutField> {
    let (main, _) = if let Some(pos) = field.def.find('=') {
        (
            field.def[..pos].trim(),
            Some(field.def[pos + 1..].trim().to_string()),
        )
    } else {
        (field.def.as_str(), None)
    };

    let mut parts = main.split_whitespace();
    let orig_type = parts.next()?.to_string();
    let remainder = parts.next()?.to_string();
    let mut name = remainder.clone();
    let mut bit_width = None;
    let mut array_size = None;

    if let Some(colon) = remainder.find(':') {
        name = remainder[..colon].to_string();
        let after = &remainder[colon + 1..];
        let (bw_str, rest) = if let Some(bracket) = after.find('[') {
            (&after[..bracket], &after[bracket..])
        } else {
            (after, "")
        };
        bit_width = bw_str.parse::<u32>().ok();
        if rest.starts_with('[') && rest.ends_with(']') {
            array_size = rest[1..rest.len() - 1].parse().ok();
        }
    } else if let Some(bracket) = remainder.find('[') {
        name = remainder[..bracket].to_string();
        let closing = remainder.find(']').unwrap_or(remainder.len());
        array_size = remainder[bracket + 1..closing].parse().ok();
    }

    let field_type = match (bit_width, array_size) {
        (None, None) => FieldType::Standard(orig_type),
        (None, Some(array_size)) => {
            FieldType::Array(Box::new(FieldType::Standard(orig_type)), array_size)
        }
        (Some(bit_width), None) => FieldType::Bitfield(bit_width as u8),
        (Some(_), Some(_)) => unimplemented!(),
    };

    Some(LayoutField { name, field_type })
}

fn align_offset(offset: usize, align: usize) -> usize {
    (offset + align - 1) & !(align - 1)
}
