use shared::multi_param;

mod generated;

pub use generated::*;

/// A trait that contains the fields shared across all four equipment
/// parameters.
#[multi_param(
    EQUIP_PARAM_ACCESSORY_ST,
    EQUIP_PARAM_GEM_ST,
    EQUIP_PARAM_GOODS_ST,
    EQUIP_PARAM_PROTECTOR_ST,
    EQUIP_PARAM_WEAPON_ST
)]
pub trait EquipParam {
    fields! {
        sell_value: i32,
        sort_id: i32,
        sort_group_id: u8,
        rarity: u8,
        sale_value: i32,
    }
}

/// A trait that contains the fields shared across all equipment parameters that
/// aren't armor.
#[multi_param(
    EQUIP_PARAM_ACCESSORY_ST,
    EQUIP_PARAM_GEM_ST,
    EQUIP_PARAM_GOODS_ST,
    EQUIP_PARAM_WEAPON_ST
)]
pub trait EquipParamNonProtector: EquipParam {
    fields! {
        icon_id: u16,
        trophy_seq_id: i16,
    }
}

/// A trait that contains the fields shared across the four equipment parameters
/// that typically represent physical objects (everything but ashes of war).
#[multi_param(
    EQUIP_PARAM_ACCESSORY_ST,
    EQUIP_PARAM_GOODS_ST,
    EQUIP_PARAM_PROTECTOR_ST,
    EQUIP_PARAM_WEAPON_ST
)]
pub trait EquipParamPhysical: EquipParam {
    fields! {
        weight: f32,
    }
}

/// A trait that contains the fields shared across the equipment parameters that
/// the player can wear as equipment (talismans, armor, and weapons).
#[multi_param(
    EQUIP_PARAM_ACCESSORY_ST,
    EQUIP_PARAM_PROTECTOR_ST,
    EQUIP_PARAM_WEAPON_ST
)]
pub trait EquipParamWearable: EquipParamPhysical {
    fields! {
        equip_model_id: u16,
        trophy_s_grade_id: i16,
        equip_model_category: u8,
        equip_model_gender: u8,
        #[multi_param(
            rename(param = EQUIP_PARAM_PROTECTOR_ST, name = "resident_sp_effect_id"),
            rename(param = EQUIP_PARAM_WEAPON_ST, name = "resident_sp_effect_id"),
        )]
        resident_sp_effect_id1: i32,
        #[multi_param(rename(param = EQUIP_PARAM_WEAPON_ST, name = "resident_sp_effect_id1"))]
        resident_sp_effect_id2: i32,
        #[multi_param(rename(param = EQUIP_PARAM_WEAPON_ST, name = "resident_sp_effect_id2"))]
        resident_sp_effect_id3: i32,
    }
}

/// A trait that contains the fields shared across the equipment parameters that
/// don't involve weapons (talismans, armor, and goods).
#[multi_param(
    EQUIP_PARAM_ACCESSORY_ST,
    EQUIP_PARAM_GOODS_ST,
    EQUIP_PARAM_PROTECTOR_ST
)]
pub trait EquipParamNonWeapon: EquipParamPhysical {
    fields! {
        basic_price: i32,
        shop_lv: i16,
    }
}

/// A trait that contains the fields shared across the equipment parameters that
/// commonly provide passive effects (goods and talismans).
#[multi_param(EQUIP_PARAM_ACCESSORY_ST, EQUIP_PARAM_GOODS_ST)]
pub trait EquipParamPassive: EquipParamPhysical {
    fields! {
        sfx_variation_id: i32,
        behavior_id: i32,
        basic_price: i32,
        ref_category: u8,
        sp_effect_category: u8,
        vagrant_item_lot_id: i32,
        vagrant_bonus_ene_drop_item_lot_id: i32,
        vagrant_item_ene_drop_item_lot_id: i32,
    }
}
