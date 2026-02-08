use shared::multi_param;

mod generated;

pub use generated::*;

/// A trait that contains the fields shared across all four equipment
/// parameters.
#[multi_param(
    EQUIP_PARAM_ACCESSORY_ST,
    EQUIP_PARAM_GOODS_ST,
    EQUIP_PARAM_PROTECTOR_ST,
    EQUIP_PARAM_WEAPON_ST
)]
pub trait EquipParam {
    fields! {
        weight: f32,
        basic_price: i32,
        sell_value: i32,
        sort_id: i32,
        vagrant_item_lot_id: i32,
        #[multi_param(
            rename(param = EQUIP_PARAM_PROTECTOR_ST, name = "vagrant_bonusene_drop_item_lot_id"),
            rename(param = EQUIP_PARAM_WEAPON_ST, name = "vagrant_bonusene_drop_item_lot_id"),
        )]
        vagrant_bonus_ene_drop_item_lot_id: i32,
        vagrant_item_ene_drop_item_lot_id: i32,
    }

    /// Returns this as an [EQUIP_PARAM_ACCESSORY_ST], if it is one.
    fn as_accessory(&self) -> Option<&EQUIP_PARAM_ACCESSORY_ST> {
        if let EquipParamStruct::EQUIP_PARAM_ACCESSORY_ST(s) = self.as_enum() {
            Some(s)
        } else {
            None
        }
    }

    /// Returns this as a mutable [EQUIP_PARAM_ACCESSORY_ST], if it is one.
    fn as_accessory_mut(&mut self) -> Option<&mut EQUIP_PARAM_ACCESSORY_ST> {
        if let EquipParamStructMut::EQUIP_PARAM_ACCESSORY_ST(s) = self.as_enum_mut() {
            Some(s)
        } else {
            None
        }
    }

    /// Returns this as an [EQUIP_PARAM_GOODS_ST], if it is one.
    fn as_goods(&self) -> Option<&EQUIP_PARAM_GOODS_ST> {
        if let EquipParamStruct::EQUIP_PARAM_GOODS_ST(s) = self.as_enum() {
            Some(s)
        } else {
            None
        }
    }

    /// Returns this as a mutable [EQUIP_PARAM_GOODS_ST], if it is one.
    fn as_goods_mut(&mut self) -> Option<&mut EQUIP_PARAM_GOODS_ST> {
        if let EquipParamStructMut::EQUIP_PARAM_GOODS_ST(s) = self.as_enum_mut() {
            Some(s)
        } else {
            None
        }
    }

    /// Returns this as an [EQUIP_PARAM_PROTECTOR_ST], if it is one.
    fn as_protector(&self) -> Option<&EQUIP_PARAM_PROTECTOR_ST> {
        if let EquipParamStruct::EQUIP_PARAM_PROTECTOR_ST(s) = self.as_enum() {
            Some(s)
        } else {
            None
        }
    }

    /// Returns this as a mutable [EQUIP_PARAM_PROTECTOR_ST], if it is one.
    fn as_protector_mut(&mut self) -> Option<&mut EQUIP_PARAM_PROTECTOR_ST> {
        if let EquipParamStructMut::EQUIP_PARAM_PROTECTOR_ST(s) = self.as_enum_mut() {
            Some(s)
        } else {
            None
        }
    }

    /// Returns this as an [EQUIP_PARAM_WEAPON_ST], if it is one.
    fn as_weapon(&self) -> Option<&EQUIP_PARAM_WEAPON_ST> {
        if let EquipParamStruct::EQUIP_PARAM_WEAPON_ST(s) = self.as_enum() {
            Some(s)
        } else {
            None
        }
    }

    /// Returns this as a mutable [EQUIP_PARAM_WEAPON_ST], if it is one.
    fn as_weapon_mut(&mut self) -> Option<&mut EQUIP_PARAM_WEAPON_ST> {
        if let EquipParamStructMut::EQUIP_PARAM_WEAPON_ST(s) = self.as_enum_mut() {
            Some(s)
        } else {
            None
        }
    }
}

/// A trait that contains the fields shared across the equipment parameters that
/// the player can wear as equipment (rings, armor, and weapons).
#[multi_param(
    EQUIP_PARAM_ACCESSORY_ST,
    EQUIP_PARAM_PROTECTOR_ST,
    EQUIP_PARAM_WEAPON_ST
)]
pub trait EquipParamWearable: EquipParam {
    fields! {
        equip_model_id: i16,
        equip_model_category: u8,
        equip_model_gender: u8,
        sale_value: i32,
        #[multi_param(
            rename(param = EQUIP_PARAM_PROTECTOR_ST, name = "resident_sp_effect_id"),
            rename(param = EQUIP_PARAM_WEAPON_ST, name = "resident_sp_effect_id0"),
        )]
        resident_sp_effect_id1: i32,
        #[multi_param(rename(param = EQUIP_PARAM_WEAPON_ST, name = "resident_sp_effect_id1"))]
        resident_sp_effect_id2: i32,
        #[multi_param(rename(param = EQUIP_PARAM_WEAPON_ST, name = "resident_sp_effect_id2"))]
        resident_sp_effect_id3: i32,
    }
}

/// A trait that contains the fields shared across the equipment parameters that
/// commonly provide passive effects (goods and rings).
#[multi_param(EQUIP_PARAM_ACCESSORY_ST, EQUIP_PARAM_GOODS_ST)]
pub trait EquipParamPassive: EquipParam {
    fields! {
        sfx_variation_id: i32,
        ref_category: u8,
        sp_effect_category: u8,
        shop_lv: i16,
    }
}
