use std::ptr::NonNull;

use shared::{OwnedPtr, Subclass};

use crate::{
    ArrayWithHeader, Vector,
    cs::BlockId,
    dlkr::DLAllocatorRef,
    fd4::{FD4ParamResCap, FD4ResCap, FD4ResRep},
    param::ParamDef,
    stl::Tree,
};

#[repr(C)]
/// Entry in the weapon upgrade index map.
/// Maps different param row indices to their parent/child/sibling relation for the weapon upgrade levels.
pub struct WeaponUpgradeIndexMapEntry {
    /// Index to the base weapon param id upgrade param row.
    pub parent: i16,
    /// Index to the next sibling param row.
    pub next_sibling: i16,
    /// Index to the first child param row.
    /// -1 if not a parent.
    pub first_child: i16,
}

#[repr(C)]
/// Structure for fast weapon reinforcement param lookups.
/// Uses param row index and allows traversal of the upgrade tree.
///
pub struct CSWepReinforceTree {
    vftable: usize,
    pub allocator: DLAllocatorRef,
    /// Array of map entries, one for each weapon param row.
    /// The index corresponds to the weapon param row index.
    pub index_map: ArrayWithHeader<WeaponUpgradeIndexMapEntry>,
}

impl CSWepReinforceTree {
    /// Get the next upgrade param row index for the given weapon param row index.
    pub fn get_next_upgrade(&self, weapon_param_row_index: usize) -> Option<usize> {
        // Safety: index_map is guaranteed to be valid.
        let upgrade_list = unsafe { self.index_map.as_slice() };
        let next_index = upgrade_list[weapon_param_row_index].first_child;
        if next_index == -1 {
            None
        } else {
            Some(next_index as usize)
        }
    }
    /// Get the base weapon param row index for the given weapon param row index.
    pub fn get_base_weapon(&self, weapon_param_row_index: usize) -> Option<usize> {
        // Safety: index_map is guaranteed to be valid.
        let upgrade_list = unsafe { self.index_map.as_slice() };
        let entry = &upgrade_list[weapon_param_row_index];
        if entry.parent == -1 {
            None
        } else {
            Some(entry.parent as usize)
        }
    }
    /// Get the first child upgrade param row index for the given weapon param row index.
    pub fn get_first_child(&self, weapon_param_row_index: usize) -> Option<usize> {
        // Safety: index_map is guaranteed to be valid.
        let upgrade_list = unsafe { self.index_map.as_slice() };
        let entry = &upgrade_list[weapon_param_row_index];
        let first_child_index = entry.first_child;
        if first_child_index == -1 {
            None
        } else {
            Some(first_child_index as usize)
        }
    }
}

#[repr(C)]
pub struct MatchAreaLimit {
    pub area_id: u32,
    pub multi_play_start_limit_event_flag_id: u32,
}

#[repr(C)]
pub struct BuddyStoneTalkChrEntityId {
    /// Chr entity ID of specific buddy stone.
    pub talk_chr_entity_id: u32,
    /// Index into BuddyStoneParam for the stone associated with this chr entity.
    pub buddy_stone_param_index: u32,
}

#[repr(C)]
pub struct BonfireEntityId {
    pub bonfire_entity_id: u32,
    pub bonfire_warp_param_index: u32,
}

#[repr(C)]
pub struct AssetReplacementParamMapEntry {
    pub block_id: BlockId,
    pub param_row_index: u32,
}

#[repr(C)]
pub struct ChrEquipModelMapEntry {
    /// Packed key = (equip_type << 24) | (gender << 16) | model_id
    pub key: u32,
    /// Row index in [CHR_EQUIP_MODEL_PARAM_ST] (not param ID)
    pub param_row_index: u32,
}

#[repr(C)]
#[derive(Subclass)]
#[subclass(base = FD4ResCap)]
pub struct ParamResCap {
    pub res_cap: FD4ResCap,
    /// Type of ParamResCap, should correspond to where you got it from.
    ///
    /// Eg. [SoloParamRepository] solo param holders will always have
    /// [ParamResCapType::SoloParam]
    /// while CSEventFlagUsageParamManager will have
    /// [ParamResCapType::EventFlagUsageParam]
    pub param_type: ParamResCapType,
    /// Actual res cap is owned by ParamRepository
    pub param_res_cap: NonNull<FD4ParamResCap>,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParamResCapType {
    SoloParam = 0,
    EventFlagUsageParam = 1,
    SystemParam = 2,
    GConfigParam = 3,
    PerformanceCheckParam = 4,
}

#[repr(C)]
/// Holder for solo param res caps.
pub struct SoloParamHolder {
    pub res_cap_count: u32,
    /// The list can hold up to 8 param res caps, but the game only seems to use first.
    /// Supposedly this is used for some versioning.
    res_cap_list: [Option<OwnedPtr<ParamResCap>>; 8],
}

impl SoloParamHolder {
    pub fn get_res_cap(&self, index: usize) -> Option<&ParamResCap> {
        self.res_cap_list.get(index)?.as_deref()
    }
    pub fn get_res_cap_mut(&mut self, index: usize) -> Option<&mut ParamResCap> {
        self.res_cap_list.get_mut(index)?.as_deref_mut()
    }
    pub fn get_res_caps(&self) -> impl Iterator<Item = &ParamResCap> {
        self.res_cap_list.iter().filter_map(|opt| opt.as_deref())
    }
    pub fn get_res_caps_mut(&mut self) -> impl Iterator<Item = &mut ParamResCap> {
        self.res_cap_list
            .iter_mut()
            .filter_map(|opt| opt.as_deref_mut())
    }
}

#[repr(C)]
#[shared::singleton("SoloParamRepository")]
pub struct SoloParamRepository {
    pub res_rep: FD4ResRep,
    unk78: u32,
    /// Array of solo param holders, one for each solo param type.
    pub solo_param_holders: [SoloParamHolder; 194],
    /// Structure for fast weapon reinforcement lookups.
    /// Uses param row of WeaponEquipParam as index
    /// and allows traversal of the upgrade tree.
    pub wep_reinforce_tree: CSWepReinforceTree,
    /// Ordered list of buddy stone entity IDs and their associated buddy stone param indices.
    ///
    /// Can be used to search [crate::param::BUDDY_STONE_PARAM_ST] rows based on chr entity ID.
    pub buddy_stone_entity_ids: Vector<BuddyStoneTalkChrEntityId>,
    /// Ordered list of bonfire entity IDs and their associated bonfire warp param indices.
    ///
    /// Can be used to search [crate::param::BONFIRE_WARP_PARAM_ST] rows based on bonfire entity ID.
    pub bonfire_warp_list: Vector<BonfireEntityId>,
    /// Tree groupping [WEATHER_ASSET_REPLACE_PARAM_ST] param rows by [BlockId].
    pub weather_asset_replace_tree: Tree<AssetReplacementParamMapEntry>,
    /// Tree groupping [LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM] param rows by [BlockId].
    pub legacy_distant_view_parts_replace_tree: Tree<AssetReplacementParamMapEntry>,
    /// Tree mapping for the [CHR_EQUIP_MODEL_PARAM_ST] param rows.
    /// The usage of this param is unknown.
    pub chr_equip_model_tree: Tree<ChrEquipModelMapEntry>,
    /// Map of all area IDs to their multiplay event flag limits.
    pub match_area_limits: Tree<MatchAreaLimit>,
}

impl SoloParamRepository {
    pub fn get_by_buddy_stone_param_by_entity_id(
        &self,
        talk_chr_entity_id: u32,
    ) -> Option<&crate::param::BUDDY_STONE_PARAM_ST> {
        let entry_index = self
            .buddy_stone_entity_ids
            .items()
            .binary_search_by_key(&talk_chr_entity_id, |e| e.talk_chr_entity_id)
            .ok()?;
        let entry = &self.buddy_stone_entity_ids.items()[entry_index];
        self.get_by_row_index::<BuddyStoneParam>(entry.buddy_stone_param_index as usize)
    }

    pub fn get_by_bonfire_warp_param_by_entity_id(
        &self,
        bonfire_entity_id: u32,
    ) -> Option<&crate::param::BONFIRE_WARP_PARAM_ST> {
        let entry_index = self
            .bonfire_warp_list
            .items()
            .binary_search_by_key(&bonfire_entity_id, |e| e.bonfire_entity_id)
            .ok()?;
        let entry = &self.bonfire_warp_list.items()[entry_index];
        self.get_by_row_index::<BonfireWarpParam>(entry.bonfire_warp_param_index as usize)
    }

    pub fn weather_asset_replace_params_by_block_id(
        &self,
        block_id: BlockId,
    ) -> impl Iterator<Item = &crate::param::WEATHER_ASSET_REPLACE_PARAM_ST> {
        self.weather_asset_replace_tree
            .filtered_iter(move |e| e.block_id.0.cmp(&block_id.0))
            .filter_map(|e| {
                self.get_by_row_index::<WeatherAssetReplaceParam>(e.param_row_index as usize)
            })
    }

    /// Get solo param (regulation.bin) row by param type and param ID.
    pub fn get<P: SoloParam>(&self, param_id: u32) -> Option<&P::UnderlyingType> {
        let holder = self.solo_param_holders.get(P::INDEX as usize)?;
        let res_cap = holder.get_res_cap(0)?;
        // SAFETY: we shouldn't run into invalid casts because of the code gen dictating underlying type.
        unsafe {
            res_cap
                .param_res_cap
                .as_ref()
                .data
                .get_row_by_id::<P::UnderlyingType>(param_id)
        }
    }
    /// Get mutable solo param (regulation.bin) row by param type and param ID.
    ///
    /// # Safety
    ///
    /// [SoloParamRepository], technically, doesn't own underlying param data,
    /// but it owns the [ParamResCap] which is tied to the [FD4ParamResCap] lifetime.
    pub unsafe fn get_mut<P: SoloParam>(
        &mut self,
        param_id: u32,
    ) -> Option<&mut P::UnderlyingType> {
        let holder = self.solo_param_holders.get_mut(P::INDEX as usize)?;
        let res_cap = holder.get_res_cap_mut(0)?;
        // SAFETY: we shouldn't run into invalid casts because of the code gen dictating underlying type.
        unsafe {
            res_cap
                .param_res_cap
                .as_mut()
                .data
                .get_row_by_id_mut::<P::UnderlyingType>(param_id)
        }
    }

    /// Get solo param (regulation.bin) row by param type and row index.
    ///
    /// **IMPORTANT**: row index is NOT the same as param ID, use this when you already know the index with mapping like
    /// [SoloParamRepository::wep_reinforce_tree] or [SoloParamRepository::buddy_stone_entity_ids].
    pub fn get_by_row_index<P: SoloParam>(&self, row_index: usize) -> Option<&P::UnderlyingType> {
        let holder = self.solo_param_holders.get(P::INDEX as usize)?;
        let res_cap = holder.get_res_cap(0)?;
        // SAFETY: we shouldn't run into invalid casts because of the code gen dictating underlying type.
        unsafe {
            res_cap
                .param_res_cap
                .as_ref()
                .data
                .get_by_row_index::<P::UnderlyingType>(row_index)
        }
    }

    /// Get mutable solo param (regulation.bin) row by param type and row index.
    ///
    /// **IMPORTANT**: row index is NOT the same as param ID, use this when you already know the index with mapping like
    /// [SoloParamRepository::wep_reinforce_tree] or [SoloParamRepository::buddy_stone_entity_ids].
    ///
    /// # Safety
    ///
    /// [SoloParamRepository], technically, doesn't own underlying param data,
    /// but it owns the [ParamResCap] which is tied to the [FD4ParamResCap] lifetime.
    pub unsafe fn get_by_row_index_mut<P: SoloParam>(
        &mut self,
        row_index: usize,
    ) -> Option<&mut P::UnderlyingType> {
        let holder = self.solo_param_holders.get_mut(P::INDEX as usize)?;
        let res_cap = holder.get_res_cap_mut(0)?;
        // SAFETY: we shouldn't run into invalid casts because of the code gen dictating underlying type.
        unsafe {
            res_cap
                .param_res_cap
                .as_mut()
                .data
                .get_by_row_index_mut::<P::UnderlyingType>(row_index)
        }
    }

    pub fn get_index_by_param_id<P: SoloParam>(&self, param_id: u32) -> Option<usize> {
        let holder = self.solo_param_holders.get(P::INDEX as usize)?;
        let res_cap = holder.get_res_cap(0)?;
        // SAFETY: we shouldn't run into invalid casts because of the code gen dictating underlying type.
        unsafe {
            res_cap
                .param_res_cap
                .as_ref()
                .data
                .metadata()
                .find_index(param_id)
        }
    }
}

pub trait SoloParam {
    const NAME: &'static str;
    const INDEX: u32;
    type UnderlyingType: ParamDef;
}

use crate::param::*;

macro_rules! solo_params {
    ( $( ($ParamType:ident, $UnderlyingType:ty, $Index:expr) ),* $(,)? ) => {
        $(
            #[allow(non_camel_case_types)]
            pub struct $ParamType;
            impl SoloParam for $ParamType {
                const NAME: &'static str = stringify!($ParamType);
                const INDEX: u32 = $Index;
                type UnderlyingType = $UnderlyingType;
            }
        )*
    };
}

solo_params!(
    (EquipParamWeapon, EQUIP_PARAM_WEAPON_ST, 0),
    (EquipParamProtector, EQUIP_PARAM_PROTECTOR_ST, 1),
    (EquipParamAccessory, EQUIP_PARAM_ACCESSORY_ST, 2),
    (EquipParamGoods, EQUIP_PARAM_GOODS_ST, 3),
    (ReinforceParamWeapon, REINFORCE_PARAM_WEAPON_ST, 4),
    (ReinforceParamProtector, REINFORCE_PARAM_PROTECTOR_ST, 5),
    (NpcParam, NPC_PARAM_ST, 6),
    (AtkParam_Npc, ATK_PARAM_ST, 7),
    (AtkParam_Pc, ATK_PARAM_ST, 8),
    (NpcThinkParam, NPC_THINK_PARAM_ST, 9),
    (Bullet, BULLET_PARAM_ST, 10),
    (BulletCreateLimitParam, BULLET_CREATE_LIMIT_PARAM_ST, 11),
    (BehaviorParam, BEHAVIOR_PARAM_ST, 12),
    (BehaviorParam_PC, BEHAVIOR_PARAM_ST, 13),
    (Magic, MAGIC_PARAM_ST, 14),
    (SpEffectParam, SP_EFFECT_PARAM_ST, 15),
    (SpEffectVfxParam, SP_EFFECT_VFX_PARAM_ST, 16),
    (SpEffectSetParam, SP_EFFECT_SET_PARAM_ST, 17),
    (TalkParam, TALK_PARAM_ST, 18),
    (MenuColorTableParam, MENU_PARAM_COLOR_TABLE_ST, 19),
    (ItemLotParam_enemy, ITEMLOT_PARAM_ST, 20),
    (ItemLotParam_map, ITEMLOT_PARAM_ST, 21),
    (MoveParam, MOVE_PARAM_ST, 22),
    (CharaInitParam, CHARACTER_INIT_PARAM, 23),
    (EquipMtrlSetParam, EQUIP_MTRL_SET_PARAM_ST, 24),
    (FaceParam, FACE_PARAM_ST, 25),
    (FaceRangeParam, FACE_RANGE_PARAM_ST, 26),
    (ShopLineupParam, SHOP_LINEUP_PARAM, 27),
    (ShopLineupParam_Recipe, SHOP_LINEUP_PARAM, 28),
    (GameAreaParam, GAME_AREA_PARAM_ST, 29),
    (CalcCorrectGraph, CACL_CORRECT_GRAPH_ST, 30),
    (LockCamParam, LOCK_CAM_PARAM_ST, 31),
    (ObjActParam, OBJ_ACT_PARAM_ST, 32),
    (HitMtrlParam, HIT_MTRL_PARAM_ST, 33),
    (KnockBackParam, KNOCKBACK_PARAM_ST, 34),
    (DecalParam, DECAL_PARAM_ST, 35),
    (ActionButtonParam, ACTIONBUTTON_PARAM_ST, 36),
    (AiSoundParam, AI_SOUND_PARAM_ST, 37),
    (PlayRegionParam, PLAY_REGION_PARAM_ST, 38),
    (NetworkAreaParam, NETWORK_AREA_PARAM_ST, 39),
    (NetworkParam, NETWORK_PARAM_ST, 40),
    (NetworkMsgParam, NETWORK_MSG_PARAM_ST, 41),
    (BudgetParam, BUDGET_PARAM_ST, 42),
    (BonfireWarpParam, BONFIRE_WARP_PARAM_ST, 43),
    (BonfireWarpTabParam, BONFIRE_WARP_TAB_PARAM_ST, 44),
    (
        BonfireWarpSubCategoryParam,
        BONFIRE_WARP_SUB_CATEGORY_PARAM_ST,
        45
    ),
    (MenuPropertySpecParam, MENUPROPERTY_SPEC, 46),
    (MenuPropertyLayoutParam, MENUPROPERTY_LAYOUT, 47),
    (MenuValueTableParam, MENU_VALUE_TABLE_SPEC, 48),
    (Ceremony, CEREMONY_PARAM_ST, 49),
    (PhantomParam, PHANTOM_PARAM_ST, 50),
    (CharMakeMenuTopParam, CHARMAKEMENUTOP_PARAM_ST, 51),
    (
        CharMakeMenuListItemParam,
        CHARMAKEMENU_LISTITEM_PARAM_ST,
        52
    ),
    (
        HitEffectSfxConceptParam,
        HIT_EFFECT_SFX_CONCEPT_PARAM_ST,
        53
    ),
    (HitEffectSfxParam, HIT_EFFECT_SFX_PARAM_ST, 54),
    (WepAbsorpPosParam, WEP_ABSORP_POS_PARAM_ST, 55),
    (ToughnessParam, TOUGHNESS_PARAM_ST, 56),
    (SeMaterialConvertParam, SE_MATERIAL_CONVERT_PARAM_ST, 57),
    (ThrowDirectionSfxParam, THROW_DIRECTION_SFX_PARAM_ST, 58),
    (DirectionCameraParam, DIRECTION_CAMERA_PARAM_ST, 59),
    (RoleParam, ROLE_PARAM_ST, 60),
    (WaypointParam, WAYPOINT_PARAM_ST, 61),
    (ThrowParam, THROW_PARAM_ST, 62),
    (GrassTypeParam, GRASS_TYPE_PARAM_ST, 63),
    (GrassTypeParam_Lv1, GRASS_TYPE_PARAM_ST, 64),
    (GrassTypeParam_Lv2, GRASS_TYPE_PARAM_ST, 65),
    (GrassLodRangeParam, GRASS_LOD_RANGE_PARAM_ST, 66),
    (NpcAiActionParam, NPC_AI_ACTION_PARAM_ST, 67),
    (PartsDrawParam, PARTS_DRAW_PARAM_ST, 68),
    (AssetEnvironmentGeometryParam, ASSET_GEOMETORY_PARAM_ST, 69),
    (AssetModelSfxParam, ASSET_MODEL_SFX_PARAM_ST, 70),
    (AssetMaterialSfxParam, ASSET_MATERIAL_SFX_PARAM_ST, 71),
    (
        AttackElementCorrectParam,
        ATTACK_ELEMENT_CORRECT_PARAM_ST,
        72
    ),
    (FootSfxParam, FOOT_SFX_PARAM_ST, 73),
    (MaterialExParam, MATERIAL_EX_PARAM_ST, 74),
    (HPEstusFlaskRecoveryParam, ESTUS_FLASK_RECOVERY_PARAM_ST, 75),
    (MPEstusFlaskRecoveryParam, ESTUS_FLASK_RECOVERY_PARAM_ST, 76),
    (MultiPlayCorrectionParam, MULTI_PLAY_CORRECTION_PARAM_ST, 77),
    (MenuOffscrRendParam, MENU_OFFSCR_REND_PARAM_ST, 78),
    (ClearCountCorrectParam, CLEAR_COUNT_CORRECT_PARAM_ST, 79),
    (
        MapMimicryEstablishmentParam,
        MAP_MIMICRY_ESTABLISHMENT_PARAM_ST,
        80
    ),
    (WetAspectParam, WET_ASPECT_PARAM_ST, 81),
    (SwordArtsParam, SWORD_ARTS_PARAM_ST, 82),
    (
        KnowledgeLoadScreenItemParam,
        KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST,
        83
    ),
    (
        MultiHPEstusFlaskBonusParam,
        MULTI_ESTUS_FLASK_BONUS_PARAM_ST,
        84
    ),
    (
        MultiMPEstusFlaskBonusParam,
        MULTI_ESTUS_FLASK_BONUS_PARAM_ST,
        85
    ),
    (MultiSoulBonusRateParam, MULTI_SOUL_BONUS_RATE_PARAM_ST, 86),
    (WorldMapPointParam, WORLD_MAP_POINT_PARAM_ST, 87),
    (WorldMapPieceParam, WORLD_MAP_PIECE_PARAM_ST, 88),
    (WorldMapLegacyConvParam, WORLD_MAP_LEGACY_CONV_PARAM_ST, 89),
    (WorldMapPlaceNameParam, WORLD_MAP_PLACE_NAME_PARAM_ST, 90),
    (ChrModelParam, CHR_MODEL_PARAM_ST, 91),
    (LoadBalancerParam, LOAD_BALANCER_PARAM_ST, 92),
    (
        LoadBalancerDrawDistScaleParam,
        LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
        93
    ),
    (
        LoadBalancerDrawDistScaleParam_ps4,
        LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
        94
    ),
    (
        LoadBalancerDrawDistScaleParam_ps5,
        LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
        95
    ),
    (
        LoadBalancerDrawDistScaleParam_xb1,
        LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
        96
    ),
    (
        LoadBalancerDrawDistScaleParam_xb1x,
        LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
        97
    ),
    (
        LoadBalancerDrawDistScaleParam_xss,
        LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
        98
    ),
    (
        LoadBalancerDrawDistScaleParam_xsx,
        LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
        99
    ),
    (
        LoadBalancerNewDrawDistScaleParam_win64,
        LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST,
        100
    ),
    (
        LoadBalancerNewDrawDistScaleParam_ps4,
        LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST,
        101
    ),
    (
        LoadBalancerNewDrawDistScaleParam_ps5,
        LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST,
        102
    ),
    (
        LoadBalancerNewDrawDistScaleParam_xb1,
        LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST,
        103
    ),
    (
        LoadBalancerNewDrawDistScaleParam_xb1x,
        LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST,
        104
    ),
    (
        LoadBalancerNewDrawDistScaleParam_xss,
        LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST,
        105
    ),
    (
        LoadBalancerNewDrawDistScaleParam_xsx,
        LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST,
        106
    ),
    (
        WwiseValueToStrParam_Switch_AttackType,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        107
    ),
    (
        WwiseValueToStrParam_Switch_DamageAmount,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        108
    ),
    (
        WwiseValueToStrParam_Switch_DeffensiveMaterial,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        109
    ),
    (
        WwiseValueToStrParam_Switch_HitStop,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        110
    ),
    (
        WwiseValueToStrParam_Switch_OffensiveMaterial,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        111
    ),
    (
        WwiseValueToStrParam_Switch_GrassHitType,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        112
    ),
    (
        WwiseValueToStrParam_Switch_PlayerShoes,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        113
    ),
    (
        WwiseValueToStrParam_Switch_PlayerEquipmentTops,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        114
    ),
    (
        WwiseValueToStrParam_Switch_PlayerEquipmentBottoms,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        115
    ),
    (
        WwiseValueToStrParam_Switch_PlayerVoiceType,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        116
    ),
    (
        WwiseValueToStrParam_Switch_AttackStrength,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        117
    ),
    (
        WwiseValueToStrParam_EnvPlaceType,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        118
    ),
    (WeatherParam, WEATHER_PARAM_ST, 119),
    (WeatherLotParam, WEATHER_LOT_PARAM_ST, 120),
    (WeatherAssetCreateParam, WEATHER_ASSET_CREATE_PARAM_ST, 121),
    (
        WeatherAssetReplaceParam,
        WEATHER_ASSET_REPLACE_PARAM_ST,
        122
    ),
    (SpeedtreeParam, SPEEDTREE_MODEL_PARAM_ST, 123),
    (RideParam, RIDE_PARAM_ST, 124),
    (SeActivationRangeParam, SE_ACTIVATION_RANGE_PARAM_ST, 125),
    (RollingObjLotParam, ROLLING_OBJ_LOT_PARAM_ST, 126),
    (
        NpcAiBehaviorProbability,
        NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST,
        127
    ),
    (BuddyParam, BUDDY_PARAM_ST, 128),
    (GparamRefSettings, GPARAM_REF_SETTINGS_PARAM_ST, 129),
    (RandomAppearParam, RANDOM_APPEAR_PARAM_ST, 130),
    (
        MapGridCreateHeightLimitInfoParam,
        MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST,
        131
    ),
    (EnvObjLotParam, ENV_OBJ_LOT_PARAM_ST, 132),
    (MapDefaultInfoParam, MAP_DEFAULT_INFO_PARAM_ST, 133),
    (BuddyStoneParam, BUDDY_STONE_PARAM_ST, 134),
    (
        LegacyDistantViewPartsReplaceParam,
        LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM,
        135
    ),
    (SoundCommonIngameParam, SOUND_COMMON_INGAME_PARAM_ST, 136),
    (
        SoundAutoEnvSoundGroupParam,
        SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST,
        137
    ),
    (
        SoundAutoReverbEvaluationDistParam,
        SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST,
        138
    ),
    (
        SoundAutoReverbSelectParam,
        SOUND_AUTO_REVERB_SELECT_PARAM_ST,
        139
    ),
    (EnemyCommonParam, ENEMY_COMMON_PARAM_ST, 140),
    (GameSystemCommonParam, GAME_SYSTEM_COMMON_PARAM_ST, 141),
    (GraphicsCommonParam, GRAPHICS_COMMON_PARAM_ST, 142),
    (MenuCommonParam, MENU_COMMON_PARAM_ST, 143),
    (PlayerCommonParam, PLAYER_COMMON_PARAM_ST, 144),
    (
        CutsceneGparamWeatherParam,
        CUTSCENE_GPARAM_WEATHER_PARAM_ST,
        145
    ),
    (CutsceneGparamTimeParam, CUTSCENE_GPARAM_TIME_PARAM_ST, 146),
    (
        CutsceneTimezoneConvertParam,
        CUTSCENE_TIMEZONE_CONVERT_PARAM_ST,
        147
    ),
    (
        CutsceneWeatherOverrideGparamConvertParam,
        CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST,
        148
    ),
    (SoundCutsceneParam, SOUND_CUTSCENE_PARAM_ST, 149),
    (
        ChrActivateConditionParam,
        CHR_ACTIVATE_CONDITION_PARAM_ST,
        150
    ),
    (CutsceneMapIdParam, CUTSCENE_MAP_ID_PARAM_ST, 151),
    (
        CutSceneTextureLoadParam,
        CUTSCENE_TEXTURE_LOAD_PARAM_ST,
        152
    ),
    (GestureParam, GESTURE_PARAM_ST, 153),
    (EquipParamGem, EQUIP_PARAM_GEM_ST, 154),
    (EquipParamCustomWeapon, EQUIP_PARAM_CUSTOM_WEAPON_ST, 155),
    (GraphicsConfig, CS_GRAPHICS_CONFIG_PARAM_ST, 156),
    (SoundChrPhysicsSeParam, SOUND_CHR_PHYSICS_SE_PARAM_ST, 157),
    (FeTextEffectParam, FE_TEXT_EFFECT_PARAM_ST, 158),
    (CoolTimeParam, COOL_TIME_PARAM_ST, 159),
    (WhiteSignCoolTimeParam, WHITE_SIGN_COOL_TIME_PARAM_ST, 160),
    (MapPieceTexParam, MAP_PIECE_TEX_PARAM_ST, 161),
    (MapNameTexParam, MAP_NAME_TEX_PARAM_ST, 162),
    (WeatherLotTexParam, WEATHER_LOT_TEX_PARAM_ST, 163),
    (KeyAssignParam_TypeA, KEY_ASSIGN_PARAM_ST, 164),
    (KeyAssignParam_TypeB, KEY_ASSIGN_PARAM_ST, 165),
    (KeyAssignParam_TypeC, KEY_ASSIGN_PARAM_ST, 166),
    (MapGdRegionInfoParam, MAP_GD_REGION_ID_PARAM_ST, 167),
    (MapGdRegionDrawParam, MAP_GD_REGION_DRAW_PARAM, 168),
    (KeyAssignMenuItemParam, CS_KEY_ASSIGN_MENUITEM_PARAM, 169),
    (
        SoundAssetSoundObjEnableDistParam,
        SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST,
        170
    ),
    (SignPuddleParam, SIGN_PUDDLE_PARAM_ST, 171),
    (AutoCreateEnvSoundParam, AUTO_CREATE_ENV_SOUND_PARAM_ST, 172),
    (
        WwiseValueToStrParam_BgmBossChrIdConv,
        WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
        173
    ),
    (ResistCorrectParam, RESIST_CORRECT_PARAM_ST, 174),
    (
        PostureControlParam_WepRight,
        POSTURE_CONTROL_PARAM_WEP_RIGHT_ST,
        175
    ),
    (
        PostureControlParam_WepLeft,
        POSTURE_CONTROL_PARAM_WEP_LEFT_ST,
        176
    ),
    (
        PostureControlParam_Gender,
        POSTURE_CONTROL_PARAM_GENDER_ST,
        177
    ),
    (PostureControlParam_Pro, POSTURE_CONTROL_PARAM_PRO_ST, 178),
    (RuntimeBoneControlParam, RUNTIME_BONE_CONTROL_PARAM_ST, 179),
    (TutorialParam, TUTORIAL_PARAM_ST, 180),
    (BaseChrSelectMenuParam, BASECHR_SELECT_MENU_PARAM_ST, 181),
    (
        MimicryEstablishmentTexParam,
        MIMICRY_ESTABLISHMENT_TEX_PARAM_ST,
        182
    ),
    (SfxBlockResShareParam, SFX_BLOCK_RES_SHARE_PARAM, 183),
    (FinalDamageRateParam, FINAL_DAMAGE_RATE_PARAM_ST, 184),
    (SignPuddleTabParam, SIGN_PUDDLE_TAB_PARAM_ST, 185),
    (
        SignPuddleSubCategoryParam,
        SIGN_PUDDLE_SUB_CATEGORY_PARAM_ST,
        186
    ),
    (
        MapGridCreateHeightDetailLimitInfo,
        MAP_GRID_CREATE_HEIGHT_LIMIT_DETAIL_INFO_PARAM_ST,
        187
    ),
    (MapPieceTexParam_m61, MAP_PIECE_TEX_PARAM_ST_DLC02, 188),
    (MapNameTexParam_m61, MAP_NAME_TEX_PARAM_ST_DLC02, 189),
    (WeatherLotTexParam_m61, WEATHER_LOT_TEX_PARAM_ST_DLC02, 190),
    (
        MimicryEstablishmentTexParam_m61,
        MIMICRY_ESTABLISHMENT_TEX_PARAM_ST_DLC02,
        191
    ),
    (ChrEquipModelParam, CHR_EQUIP_MODEL_PARAM_ST, 192),
    (HitEffectSeParam, HIT_EFFECT_SE_PARAM_ST, 193),
);
