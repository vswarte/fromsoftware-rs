use std::ffi::{c_char, c_str::CStr, c_void};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::{mem, ptr, ptr::NonNull, slice};

use shared::{OwnedPtr, util::IncompleteArrayField};

use crate::CxxVec;
use crate::fd4::FD4BasicHashString;
use crate::param::{
    ATK_PARAM_ST, BEHAVIOR_PARAM_ST, EQUIP_PARAM_ACCESSORY_ST, EQUIP_PARAM_GOODS_ST,
    EQUIP_PARAM_PROTECTOR_ST, EQUIP_PARAM_WEAPON_ST, EquipParam, EquipParamStruct,
    EquipParamStructMut, LOD_BANK, MULTI_ESTUS_FLASK_BONUS_PARAM_ST, ParamDef,
};
use crate::sprj::{ItemCategory, ItemId};

#[repr(C)]
#[shared::singleton("CSRegulationManager")]
pub struct CSRegulationManager {
    _vftable: usize,
    _unk8: u64,
    pub params: CxxVec<OwnedPtr<ParamResCap>>,
}

impl CSRegulationManager {
    /// Returns the first parameter table that uses the definition `T`.
    ///
    /// In most cases, each definition has a unique table associated with it.
    /// Structs with multiple tables have explicit accessors defined.
    pub fn get_param<T: ParamDef>(&self) -> &Parameter<T> {
        self.get_param_by_index(T::INDEX)
    }

    /// Returns the first mutable parameter table that uses the definition `T`.
    ///
    /// In most cases, each definition has a unique table associated with it.
    /// Structs with multiple tables have explicit accessors defined.
    pub fn get_mut_param<T: ParamDef>(&mut self) -> &mut Parameter<T> {
        self.get_mut_param_by_index(T::INDEX)
    }

    /// Returns the [ATK_PARAM_ST] struct for NPCs.
    ///
    /// There are multiple parameters that use [ATK_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn atk_param_npc(&self) -> &Parameter<ATK_PARAM_ST> {
        self.get_param_by_index(ATK_PARAM_ST::INDEX)
    }

    /// Returns the mutable [ATK_PARAM_ST] struct for NPCs.
    ///
    /// There are multiple parameters that use [ATK_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn atk_param_npc_mut(&mut self) -> &mut Parameter<ATK_PARAM_ST> {
        self.get_mut_param_by_index(ATK_PARAM_ST::INDEX)
    }

    /// Returns the [ATK_PARAM_ST] struct for PCs.
    ///
    /// There are multiple parameters that use [ATK_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn atk_param_pc(&self) -> &Parameter<ATK_PARAM_ST> {
        self.get_param_by_index(ATK_PARAM_ST::INDEX + 1)
    }

    /// Returns the mutable [ATK_PARAM_ST] struct for PCs.
    ///
    /// There are multiple parameters that use [ATK_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn atk_param_pc_mut(&mut self) -> &mut Parameter<ATK_PARAM_ST> {
        self.get_mut_param_by_index(ATK_PARAM_ST::INDEX + 1)
    }

    /// Returns the [BEHAVIOR_PARAM_ST] struct for NPCs.
    ///
    /// There are multiple parameters that use [BEHAVIOR_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn behavior_param_npc(&self) -> &Parameter<BEHAVIOR_PARAM_ST> {
        self.get_param_by_index(BEHAVIOR_PARAM_ST::INDEX)
    }

    /// Returns the mutable [BEHAVIOR_PARAM_ST] struct for NPCs.
    ///
    /// There are multiple parameters that use [BEHAVIOR_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn behavior_param_npc_mut(&mut self) -> &mut Parameter<BEHAVIOR_PARAM_ST> {
        self.get_mut_param_by_index(BEHAVIOR_PARAM_ST::INDEX)
    }

    /// Returns the [BEHAVIOR_PARAM_ST] struct for PCs.
    ///
    /// There are multiple parameters that use [BEHAVIOR_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn behavior_param_pc(&self) -> &Parameter<BEHAVIOR_PARAM_ST> {
        self.get_param_by_index(BEHAVIOR_PARAM_ST::INDEX + 1)
    }

    /// Returns the mutable [BEHAVIOR_PARAM_ST] struct for PCs.
    ///
    /// There are multiple parameters that use [BEHAVIOR_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn behavior_param_pc_mut(&mut self) -> &mut Parameter<BEHAVIOR_PARAM_ST> {
        self.get_mut_param_by_index(BEHAVIOR_PARAM_ST::INDEX + 1)
    }

    /// Returns the generic [LOD_BANK] struct.
    ///
    /// There are multiple parameters that use [LOD_BANK], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn lod_param(&self) -> &Parameter<LOD_BANK> {
        self.get_param_by_index(LOD_BANK::INDEX)
    }

    /// Returns the mutable generic [LOD_BANK] struct.
    ///
    /// There are multiple parameters that use [LOD_BANK], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn lod_param_mut(&mut self) -> &mut Parameter<LOD_BANK> {
        self.get_mut_param_by_index(LOD_BANK::INDEX)
    }

    /// Returns the [LOD_BANK] struct for PS4.
    ///
    /// There are multiple parameters that use [LOD_BANK], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn lod_param_ps4(&self) -> &Parameter<LOD_BANK> {
        self.get_param_by_index(LOD_BANK::INDEX + 1)
    }

    /// Returns the mutable [LOD_BANK] struct for PS4.
    ///
    /// There are multiple parameters that use [LOD_BANK], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn lod_param_ps4_mut(&mut self) -> &mut Parameter<LOD_BANK> {
        self.get_mut_param_by_index(LOD_BANK::INDEX + 1)
    }

    /// Returns the [LOD_BANK] struct for XBox.
    ///
    /// There are multiple parameters that use [LOD_BANK], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn lod_param_xbl(&self) -> &Parameter<LOD_BANK> {
        self.get_param_by_index(LOD_BANK::INDEX + 2)
    }

    /// Returns the mutable [LOD_BANK] struct for XBox.
    ///
    /// There are multiple parameters that use [LOD_BANK], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn lod_param_xbl_mut(&mut self) -> &mut Parameter<LOD_BANK> {
        self.get_mut_param_by_index(LOD_BANK::INDEX + 2)
    }

    /// Returns the [MULTI_ESTUS_FLASK_BONUS_PARAM_ST] struct for the normal
    /// estus flask.
    ///
    /// There are multiple parameters that use [MULTI_ESTUS_FLASK_BONUS_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn multi_hp_estus_flask_bonus_param(&self) -> &Parameter<MULTI_ESTUS_FLASK_BONUS_PARAM_ST> {
        self.get_param_by_index(MULTI_ESTUS_FLASK_BONUS_PARAM_ST::INDEX)
    }

    /// Returns the mutable [MULTI_ESTUS_FLASK_BONUS_PARAM_ST] struct for the
    /// normal estus flask.
    ///
    /// There are multiple parameters that use [MULTI_ESTUS_FLASK_BONUS_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn multi_hp_estus_flask_bonus_param_mut(
        &mut self,
    ) -> &mut Parameter<MULTI_ESTUS_FLASK_BONUS_PARAM_ST> {
        self.get_mut_param_by_index(MULTI_ESTUS_FLASK_BONUS_PARAM_ST::INDEX)
    }

    /// Returns the [MULTI_ESTUS_FLASK_BONUS_PARAM_ST] struct for the ashen
    /// estus flask.
    ///
    /// There are multiple parameters that use [MULTI_ESTUS_FLASK_BONUS_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn multi_mp_estus_flask_bonus_param(&self) -> &Parameter<MULTI_ESTUS_FLASK_BONUS_PARAM_ST> {
        self.get_param_by_index(MULTI_ESTUS_FLASK_BONUS_PARAM_ST::INDEX + 1)
    }

    /// Returns the mutable [MULTI_ESTUS_FLASK_BONUS_PARAM_ST] struct for the
    /// ashen estus flask.
    ///
    /// There are multiple parameters that use [MULTI_ESTUS_FLASK_BONUS_PARAM_ST], so it's
    /// unreliable to use it with [get_param](Self::get_param).
    pub fn multi_mp_estus_flask_bonus_param_mut(
        &mut self,
    ) -> &mut Parameter<MULTI_ESTUS_FLASK_BONUS_PARAM_ST> {
        self.get_mut_param_by_index(MULTI_ESTUS_FLASK_BONUS_PARAM_ST::INDEX + 1)
    }

    /// Returns the parameter at the given [index]. Panics if it doesn't match
    /// `T`.
    fn get_param_by_index<T: ParamDef>(&self, index: usize) -> &Parameter<T> {
        let table = &self.params[index].param.table;
        table.as_param().unwrap_or_else(|| {
            panic!(
                "Expected param index {} to be {}, was {}",
                index,
                T::NAME,
                table.name()
            )
        })
    }

    /// Returns the parameter at the given [index]. Panics if it doesn't match
    /// `T`.
    fn get_mut_param_by_index<T: ParamDef>(&mut self, index: usize) -> &mut Parameter<T> {
        let table = &mut self.params[index].param.table;
        table
            .as_mut_param()
            // The borrow checker won't let us include the  actual name ere
            .unwrap_or_else(|| panic!("Expected param index {} to be {}", index, T::NAME))
    }

    /// Returns a dynamically-dispatched equipment parameter row for the given
    /// item ID, or `None` if the row doesn't exit.
    pub fn get_equip_param(&self, id: ItemId) -> Option<EquipParamStruct<'_>> {
        use ItemCategory::*;
        match id.category() {
            Weapon => self
                .get_param::<EQUIP_PARAM_WEAPON_ST>()
                // Round to the nearest 100 in case the ID is for an upgraded
                // weapon.
                .get((u64::from(id.param_id()) / 100) * 100)
                .map(|p| p.as_enum()),
            Protector => self
                .get_param::<EQUIP_PARAM_PROTECTOR_ST>()
                .get(id.param_id().into())
                .map(|p| p.as_enum()),
            Accessory => self
                .get_param::<EQUIP_PARAM_ACCESSORY_ST>()
                .get(id.param_id().into())
                .map(|p| p.as_enum()),
            Goods => self
                .get_param::<EQUIP_PARAM_GOODS_ST>()
                .get(id.param_id().into())
                .map(|p| p.as_enum()),
        }
    }

    /// Returns a dynamically-dispatched mutable equipment parameter row for the
    /// given item ID, or `None` if the row doesn't exit.
    pub fn get_equip_param_mut(&mut self, id: ItemId) -> Option<EquipParamStructMut<'_>> {
        use ItemCategory::*;
        match id.category() {
            Weapon => self
                .get_mut_param::<EQUIP_PARAM_WEAPON_ST>()
                // Round to the nearest 100 in case the ID is for an upgraded
                // weapon.
                .get_mut((u64::from(id.param_id()) / 100) * 100)
                .map(|p| p.as_enum_mut()),
            Protector => self
                .get_mut_param::<EQUIP_PARAM_PROTECTOR_ST>()
                .get_mut(id.param_id().into())
                .map(|p| p.as_enum_mut()),
            Accessory => self
                .get_mut_param::<EQUIP_PARAM_ACCESSORY_ST>()
                .get_mut(id.param_id().into())
                .map(|p| p.as_enum_mut()),
            Goods => self
                .get_mut_param::<EQUIP_PARAM_GOODS_ST>()
                .get_mut(id.param_id().into())
                .map(|p| p.as_enum_mut()),
        }
    }
}

#[repr(C)]
pub struct ParamResCap {
    _vftable: usize,

    /// The camel-case name of the parameter.
    pub name: FD4BasicHashString,

    _unk48: [u8; 0x20],
    pub param: OwnedPtr<FD4ParamResCap>,
}

#[repr(C)]
pub struct FD4ParamResCap {
    _vftable: usize,

    /// The camel-case name of the parameter.
    pub name: FD4BasicHashString,

    _unk48: [u8; 0x18],

    /// The total size of [table](Self.table) in bytes.
    pub table_size: usize,

    pub table: OwnedPtr<ParamTable>,
}

#[repr(C)]
pub struct ParamTable {
    _unk0: [u8; 0xa],
    pub length: u16,
    _unkc: [u8; 0x4],

    /// The offset of the parameter's snake-case name from the beginning of this
    /// struct.
    pub name_offset: usize,

    _unk18: [u8; 0x28],

    row_info: IncompleteArrayField<ParamRowInfo>,
    // Note: After the row_info is an incomplete array of the actual parameter
    // data.
}

impl ParamTable {
    /// The parameter's snake-case name.
    ///
    /// ## Panic
    ///
    /// Panics if this string isn't valid UTF-8.
    pub fn name(&self) -> &str {
        let name_ptr = ptr::from_ref(self)
            .map_addr(|addr| addr + self.name_offset)
            .cast::<c_char>();
        // Safety: We trust the game's memory layout.
        unsafe { CStr::from_ptr(name_ptr) }.to_str().unwrap()
    }

    /// Returns a pointer to the beginning of the section of the table that
    /// contains the actual parameter data.
    pub fn data(&self) -> NonNull<c_void> {
        let offset = (self.length as usize) * mem::size_of::<ParamRowInfo>();
        NonNull::from_ref(self)
            .map_addr(|addr| addr.saturating_add(offset))
            .cast::<c_void>()
    }

    /// Returns the header information about each row as a slice.
    pub fn row_info(&self) -> &[ParamRowInfo] {
        // Safety: We trust the game to report lengths accurately.
        unsafe { self.row_info.as_slice(self.length.into()) }
    }

    /// If [name](Self::name) matches `T`'s [ParamDef::NAME], converts this to a [Parameter].
    pub fn as_param<T: ParamDef>(&self) -> Option<&Parameter<T>> {
        if self.name() == T::NAME {
            // Safety: [Parameter] is a transparent wrapper around [ParamTable].
            Some(unsafe { mem::transmute::<&Self, &Parameter<T>>(self) })
        } else {
            None
        }
    }

    /// If [name](Self::name) matches `T`'s [ParamDef::NAME], converts this to a
    /// mutable [Parameter].
    pub fn as_mut_param<T: ParamDef>(&mut self) -> Option<&mut Parameter<T>> {
        if self.name() == T::NAME {
            // Safety: [Parameter] is a transparent wrapper around [ParamTable].
            Some(unsafe { mem::transmute::<&mut Self, &mut Parameter<T>>(self) })
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct ParamRowInfo {
    /// The ID of the parameter row this describes.
    pub id: u64,

    /// The offset (in bytes) from the beginning of the [ParamTable] that
    /// contains this to the data for the parameter this represents.
    pub offset: usize,

    _unk10: u64,
}

/// A safe and usable view of a single parameter table, associated with a
/// particular parameter type.
#[repr(transparent)]
pub struct Parameter<T: ParamDef> {
    pub table: ParamTable,
    _phantom: PhantomData<T>,
}

impl<T: ParamDef> Parameter<T> {
    /// Returns a slice of all the rows in this parameter.
    ///
    /// Note that these **do not** contain the row indexes. For that, you must
    /// use [iter](Self::iter).
    pub fn as_slice(&self) -> &[T] {
        // Safety: We trust the game to report lengths accurately.
        unsafe {
            slice::from_raw_parts(self.table.data().cast().as_ptr(), self.table.length.into())
        }
    }

    /// Returns a mutable slice of all the rows in this parameter.
    ///
    /// Note that these **do not** contain the row indexes. For that, you must
    /// use [iter](Self::iter).
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // Safety: We trust the game to report lengths accurately.
        unsafe {
            slice::from_raw_parts_mut(self.table.data().cast().as_ptr(), self.table.length.into())
        }
    }

    /// If this parameter has a row with the given `id`, returns it. Otherwise
    /// returns `None`.
    pub fn get(&self, id: u64) -> Option<&T> {
        // Safety: We trust DS3's memory layout
        Some(unsafe { self.ptr_for_id(id)?.as_ref() })
    }

    /// If this parameter has a row with the given `id`, returns a mutable
    /// reference to it. Otherwise returns `None`.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut T> {
        // Safety: We trust DS3's memory layout
        Some(unsafe { self.ptr_for_id(id)?.as_mut() })
    }

    /// Returns the pointer to the row with the given [id], or null if no such
    /// row exists.
    fn ptr_for_id(&self, id: u64) -> Option<NonNull<T>> {
        let infos = self.table.row_info();
        let index = infos.binary_search_by_key(&id, |info| info.id).ok()?;
        Some(
            NonNull::from_ref(&self.table)
                .map_addr(|addr| addr.saturating_add(infos[index].offset))
                .cast(),
        )
    }

    /// Returns an iterator that emits `(id, row)` pairs for each row in this
    /// parameter.
    pub fn iter(&self) -> ParamIter<'_, T> {
        ParamIter {
            param: self,
            inner: self.table.row_info().iter(),
        }
    }

    /// Returns an iterator that emits mutable `(id, row)` pairs for each row in
    /// this parameter.
    pub fn iter_mut(&mut self) -> ParamIterMut<'_, T> {
        ParamIterMut {
            param: self,
            inner: self.table.row_info().iter(),
        }
    }
}

impl<T: ParamDef> Index<u64> for Parameter<T> {
    type Output = T;

    fn index(&self, index: u64) -> &T {
        self.get(index).expect("no row found for ID")
    }
}

impl<T: ParamDef> IndexMut<u64> for Parameter<T> {
    fn index_mut(&mut self, index: u64) -> &mut T {
        self.get_mut(index).expect("no row found for ID")
    }
}

/// An iterator over parameters and their IDs.
pub struct ParamIter<'a, T: ParamDef> {
    param: &'a Parameter<T>,
    inner: slice::Iter<'a, ParamRowInfo>,
}

impl<'a, T: ParamDef> Iterator for ParamIter<'a, T> {
    type Item = (u64, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let info = self.inner.next()?;
        let ptr = NonNull::from_ref(&self.param.table)
            .map_addr(|addr| addr.saturating_add(info.offset))
            .cast();
        // Safety: We trust DS3's memory layout.
        unsafe { Some((info.id, ptr.as_ref())) }
    }
}

/// An iterator over mutable parameters and their IDs.
pub struct ParamIterMut<'a, T: ParamDef> {
    param: &'a Parameter<T>,
    inner: slice::Iter<'a, ParamRowInfo>,
}

impl<'a, T: ParamDef> Iterator for ParamIterMut<'a, T> {
    type Item = (u64, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let info = self.inner.next()?;
        let mut ptr = NonNull::from_ref(&self.param.table)
            .map_addr(|addr| addr.saturating_add(info.offset))
            .cast();
        // Safety: We trust DS3's memory layout.
        unsafe { Some((info.id, ptr.as_mut())) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x70, size_of::<ParamResCap>());
        assert_eq!(0x70, size_of::<FD4ParamResCap>());
        assert_eq!(0x40, size_of::<ParamTable>());
    }
}
