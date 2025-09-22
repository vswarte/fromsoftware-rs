use std::fmt::Display;

use super::{AtkParamLookupResult, BlockId};

/// Used to reference a specific FieldIns managed by its respective (external) domain.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldInsSelector(pub u32);

/// Used throughout the game engine to refer to characters, geometry, bullets, hits and more.
///
/// Source of name: Destructor reveals this being a field in FieldIns and it's used as a means of
/// naming some FieldIns derivant everywhere where raw pointers cannot be shared.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldInsHandle {
    pub selector: FieldInsSelector,
    pub block_id: BlockId,
}

impl FieldInsHandle {
    pub fn is_empty(&self) -> bool {
        self.selector.0 == u32::MAX
    }
}

impl Display for FieldInsHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "FieldIns(None)")
        } else {
            write!(
                f,
                "FieldIns({}, {}, {})",
                self.block_id,
                self.selector.container(),
                self.selector.index()
            )
        }
    }
}

impl FieldInsSelector {
    /// Create a new FieldInsSelector by its components.
    pub fn from_parts(field_ins_type: u32, container: u32, index: u32) -> Self {
        let mapping = &FIELD_INS_TYPE_MAPPING[field_ins_type as usize];
        Self(index & 0xFFFFF | container << mapping.container_shift | field_ins_type << 0x1C)
    }

    /// Extracts the type map index
    pub fn mapping_entry_index(&self) -> u32 {
        0xF & (self.0 >> 0x1C)
    }

    /// Retrieves the type map entry for this selector.
    fn mapping(&self) -> &'static FieldInsMapping {
        &FIELD_INS_TYPE_MAPPING[self.mapping_entry_index() as usize]
    }

    /// Extracts the container for this FieldInsSelector. Used to ex: determine the
    /// appropriate ChrSet to be calling into for a given NPC. Not used for every type.
    pub fn container(&self) -> u32 {
        self.mapping().container_mask & self.0 >> (self.mapping().container_shift & 0b00111111)
    }

    /// Extracts the index within the container for a given FieldIns.
    pub fn index(&self) -> u32 {
        (self.mapping().index_mask & self.0) & 0xFFFFF
    }
}

#[vtable_rs::vtable]
/// Describes the VMT for the FieldInsBase which ChrIns, GeomIns, BulletIns, etc derive from.
pub trait FieldInsBaseVmt {
    /// Part of FieldInsBase, retrieves reflection metadata for FD4Component derivants.
    fn get_runtime_metadata(&self) -> usize;

    fn destructor(&mut self, param_2: u32);

    /// Part of FieldInsBase, ChrIns = 1, CSBulletIns = 3, CSWorldGeomIns = 6, MapIns = 7, CSWorldGeomHitIns = 8,
    fn get_field_ins_type(&self) -> u32;

    fn use_npc_atk_param(&self) -> bool;

    fn get_atk_param_for_behavior(&self, param_2: u32, atk_param: &mut AtkParamLookupResult)
        -> u32;

    /// Part of FieldInsBase. ChrIns = 0, PlayerIns = 1, EnemyIns = 0, ReplayGhostIns = 1,
    /// ReplayEnemyIns = 0, CSBulletIns = 0, CSWorldGeomIns = 0, CSFieldInsBase = 0,
    /// CSHamariSimulateChrIns = 0, MapIns = 0, HitIns = 0, CSWorldGeomStaticIns = 0, HitInsBase =
    /// 0, CSWorldGeomHitIns = 0, CSWorldGeomDynamicIns = 0,
    fn use_player_behavior_param(&self) -> bool;

    /// Obfuscated beyond recognition
    fn unk30(&self);

    /// Obfuscated beyond recognition
    fn unk38(&self);
}

struct FieldInsMapping {
    container_mask: u32,
    container_shift: u32,
    index_mask: u32,
    unkc: u32,
}

const FIELD_INS_TYPE_MAPPING: &[FieldInsMapping] = &[
    // HIT
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0x5,
    },
    // CHR
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0x2,
    },
    // OBJ
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0x1,
    },
    // BULLET
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xE,
    },
    // SFX
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xE,
    },
    // SOUND
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xE,
    },
    // GEOM
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xD,
    },
    // MAP
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0x0,
    },
    // GEOM(Hit)
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xD,
    },
];
