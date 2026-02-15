use shared::multi_param;

mod generated;

pub use generated::*;

/// A trait that contains the fields shared across all three used equipment
/// parameters.
#[multi_param(EQUIP_PARAM_GOODS_ST, EQUIP_PARAM_PROTECTOR_ST, EQUIP_PARAM_WEAPON_ST)]
pub trait EquipParam {
    fields! {
        weight: f32,
        sell_value: i32,
        sort_id: i32,
        vagrant_item_lot_id: i32,
        vagrant_bonus_ene_drop_item_lot_id: i32,
        vagrant_item_ene_drop_item_lot_id: i32,
        sale_value: i32,
        first_get_event_flag_id: i32,
        item_ui_display_type: u8,
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
/// the can appear in-game as items (goods and weapons).
#[multi_param(EQUIP_PARAM_GOODS_ST, EQUIP_PARAM_WEAPON_ST)]
pub trait EquipParamItem: EquipParam {
    fields! {
        action_unlock_param_id: i32,
        icon_id: u16,
        trophy_seq_id: i16,
    }
}
