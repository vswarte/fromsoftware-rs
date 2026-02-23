use crate::fd4::{FD4ParamResCap, FD4ResCap};

use super::{ItemCategory, ItemId};
use crate::{fd4::ParamFile, param::*};
use shared::{OwnedPtr, Subclass, UnknownStruct};

#[repr(C)]
#[derive(Subclass)]
pub struct ParamResCap {
    pub res_cap: FD4ResCap,

    _unk68: u32,

    /// The underlying res cap.
    // This is technically owned by [FD4ParamRepository], in the sense that
    // that's the code responsible for creating and destroying it. However, for
    // efficiency reasons, we don't allow access through [FD4ParamRepository]
    // without an `unsafe` block, which makes it safe for us to expose this as
    // an [OwnedPtr] here. See the comment on
    // [eldenring::fd4::FD4ParamRepository] for details.
    pub param_res_cap: OwnedPtr<FD4ParamResCap>,
}

impl ParamResCap {
    /// In debug mode, if this parameter's name and type doesn't `P`.
    ///
    /// In release mode, this is a no-op.
    fn assert_matches_param<P: SoloParam>(&self) {
        debug_assert!(
            self.res_cap.name == P::NAME,
            "Expected param {}, was {}",
            P::NAME,
            self.res_cap.name,
        );

        let struct_name = self.param_res_cap.data.struct_name();
        debug_assert!(
            struct_name == P::StructType::NAME,
            "Expected param struct {}, was {}",
            P::StructType::NAME,
            struct_name,
        );
    }
}

#[repr(C)]
/// Holder for solo param res caps.
pub struct SoloParamHolder {
    /// The number of parameters that are actually in this holder.
    pub res_cap_count: u32,

    /// The list can hold up to 8 param res caps, but the game only seems to use first.
    /// Supposedly this is used for some versioning.
    res_caps: [Option<OwnedPtr<ParamResCap>>; 8],
}

impl SoloParamHolder {
    /// The res cap at `index` in this holder, if it exists.
    pub fn get_res_cap(&self, index: usize) -> Option<&ParamResCap> {
        self.res_caps.get(index)?.as_deref()
    }

    /// The mutable res cap at `index` in this holder, if it exists.
    pub fn get_res_cap_mut(&mut self, index: usize) -> Option<&mut ParamResCap> {
        self.res_caps.get_mut(index)?.as_deref_mut()
    }

    /// An iterator over all res caps in this holder.
    pub fn res_caps(&self) -> impl Iterator<Item = &ParamResCap> {
        self.res_caps.iter().filter_map(|opt| opt.as_deref())
    }

    /// An iterator over all mutable res caps in this holder.
    pub fn res_caps_mut(&mut self) -> impl Iterator<Item = &mut ParamResCap> {
        self.res_caps
            .iter_mut()
            .filter_map(|opt| opt.as_deref_mut())
    }
}

#[repr(C)]
#[shared::singleton("SoloParamRepository")]
#[derive(Subclass)]
pub struct SoloParamRepository {
    pub res_cap: FD4ResCap,

    _unk68: u32,

    /// Array of solo param holders, one for each solo param type.
    pub solo_param_holders: [SoloParamHolder; 136],

    _wep_reinforces: UnknownStruct<0x18>,
}

impl SoloParamRepository {
    /// An iterator over all solo parameters.
    pub fn params(&self) -> impl Iterator<Item = &FD4ParamResCap> {
        self.solo_param_holders
            .iter()
            .flat_map(|h| h.res_caps())
            .map(|rc| rc.param_res_cap.as_ref())
    }

    /// An iterator over all mutable solo parameters.
    pub fn params_mut(&mut self) -> impl Iterator<Item = &mut FD4ParamResCap> {
        self.solo_param_holders
            .iter_mut()
            .flat_map(|h| h.res_caps_mut())
            .map(|rc| rc.param_res_cap.as_mut())
    }

    /// Get a solo param (regulation.bin) row by its parameter type and ID.
    pub fn get<P: SoloParam>(&self, param_id: u32) -> Option<&P::StructType> {
        // SAFETY: By construction, [SoloParam] only applies to parameters whose
        // indices are guaranteed by the game to be consistent.
        unsafe {
            self.get_param_file::<P>()
                .get_row_by_id::<P::StructType>(param_id)
        }
    }

    /// Get a mutable solo param (regulation.bin) row by its parameter type and
    /// ID.
    pub fn get_mut<P: SoloParam>(&mut self, param_id: u32) -> Option<&mut P::StructType> {
        // SAFETY: By construction, [SoloParam] only applies to parameters whose
        // indices are guaranteed by the game to be consistent.
        unsafe {
            self.get_param_file_mut::<P>()
                .get_row_by_id_mut::<P::StructType>(param_id)
        }
    }

    /// Get a solo param (regulation.bin) row by its parameter type and row
    /// index.
    ///
    /// **IMPORTANT**: The row index is *not* the same as the parameter ID. Use
    /// this when you already know the index.
    pub fn get_row_by_index<P: SoloParam>(&self, row_index: usize) -> Option<&P::StructType> {
        // SAFETY: By construction, [SoloParam] only applies to parameters whose
        // indices are guaranteed by the game to be consistent.
        unsafe {
            self.get_param_file::<P>()
                .get_row_by_index::<P::StructType>(row_index)
        }
    }

    /// Get a mutable solo param (regulation.bin) row by its parameter type and
    /// row index.
    ///
    /// **IMPORTANT**: The row index is *not* the same as the parameter ID. Use
    /// this when you already know the index.
    pub fn get_row_by_index_mut<P: SoloParam>(
        &mut self,
        row_index: usize,
    ) -> Option<&mut P::StructType> {
        // SAFETY: By construction, [SoloParam] only applies to parameters whose
        // indices are guaranteed by the game to be consistent.
        unsafe {
            self.get_param_file_mut::<P>()
                .get_row_by_index_mut::<P::StructType>(row_index)
        }
    }

    /// Returns the index of a solo param (regulation.bin) row by its parameter
    /// type and ID.
    pub fn get_index_by_param_id<P: SoloParam>(&self, param_id: u32) -> Option<usize> {
        self.get_param_file::<P>().find_index(param_id)
    }

    /// Returns an equipment parameter row enum for the given item ID, or `None`
    /// if the row doesn't exit.
    pub fn get_equip_param(&self, id: ItemId) -> Option<EquipParamStruct<'_>> {
        use ItemCategory::*;
        match id.category() {
            Weapon => self
                // Round to the nearest 100 in case the ID is for an upgraded
                // weapon.
                .get::<EquipParamWeapon>((id.param_id() / 100) * 100)
                .map(|p| EquipParam::as_enum(p)),
            Protector => self
                .get::<EquipParamProtector>(id.param_id())
                .map(|p| EquipParam::as_enum(p)),
            Accessory => panic!("Sekiro doesn't support accessories"),
            Goods => self
                .get::<EquipParamGoods>(id.param_id())
                .map(|p| EquipParam::as_enum(p)),
        }
    }

    /// Returns a mutable equipment parameter row enum for the given item ID, or `None`
    /// if the row doesn't exit.
    pub fn get_equip_param_mut(&mut self, id: ItemId) -> Option<EquipParamStructMut<'_>> {
        use ItemCategory::*;
        match id.category() {
            Weapon => self
                // Round to the nearest 100 in case the ID is for an upgraded
                // weapon.
                .get_mut::<EquipParamWeapon>((id.param_id() / 100) * 100)
                .map(|p| EquipParam::as_enum_mut(p)),
            Protector => self
                .get_mut::<EquipParamProtector>(id.param_id())
                .map(|p| EquipParam::as_enum_mut(p)),
            Accessory => panic!("Sekiro doesn't support accessories"),
            Goods => self
                .get_mut::<EquipParamGoods>(id.param_id())
                .map(|p| EquipParam::as_enum_mut(p)),
        }
    }

    /// Returns an iterator over each row in parameter `P` along with their
    /// parameter IDs, in ID order.
    pub fn rows<'a, P: SoloParam + 'a>(
        &'a self,
    ) -> impl Iterator<Item = (u32, &'a P::StructType)> + 'a {
        unsafe { self.get_param_file::<P>().rows() }
    }

    /// Returns an iterator over each mutable row in parameter `P` along with
    /// their parameter IDs, in ID order.
    pub fn rows_mut<'a, P: SoloParam + 'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (u32, &'a mut P::StructType)> + 'a {
        unsafe { self.get_param_file_mut::<P>().rows_mut() }
    }

    /// Returns the [ParamFile] associated with `P`, if it exists at the
    /// expected index. This should never return `None` for a vanilla game,
    /// because the only [SoloParam]s this library defines are ones that are
    /// found in the game.
    fn get_param_file<P: SoloParam>(&self) -> &ParamFile {
        let holder = self
            .solo_param_holders
            .get(P::INDEX as usize)
            .unwrap_or_else(|| {
                panic!(
                    "Param {} should exist at index {}, but it does not",
                    P::NAME,
                    P::INDEX
                )
            });
        let res_cap = holder
            .get_res_cap(0)
            .expect("Expected param holder to have exactly one res cap");

        res_cap.assert_matches_param::<P>();
        &res_cap.param_res_cap.data
    }

    /// Returns the mutable [ParamFile] associated with `P`, if it exists at the
    /// expected index. This should never return `None` for a vanilla game,
    /// because the only [SoloParam]s this library defines are ones that are
    /// found in the game.
    fn get_param_file_mut<P: SoloParam>(&mut self) -> &mut ParamFile {
        let holder = self
            .solo_param_holders
            .get_mut(P::INDEX as usize)
            .unwrap_or_else(|| {
                panic!(
                    "Param {} should exist at index {}, but it does not",
                    P::NAME,
                    P::INDEX
                )
            });
        let res_cap = holder
            .get_res_cap_mut(0)
            .expect("Expected param holder to have exactly one res cap");

        res_cap.assert_matches_param::<P>();
        &mut res_cap.param_res_cap.data
    }
}

/// A shared trait for parameters that are part of [SoloParamRepository], used
/// to ensure that they can be accessed in a type-safe way.
pub trait SoloParam {
    /// The parameter name. This corresponds to `ParamResCap.res_cap.name` and
    /// `FD4ParamResCap.res_cap.name`, not to [ParamFile::struct_name] and
    /// [ParamDef::NAME].
    const NAME: &'static str;

    /// The index of this parameter in [SoloParamRepository].
    const INDEX: u32;

    /// The type of the data that this parameter contains.
    type StructType: ParamDef;
}

macro_rules! solo_params {
    ( $( ($ParamType:ident, $StructType:ty, $Index:expr) ),* $(,)? ) => {
        $(
            #[doc="The"]
            #[doc=stringify!($ParamType)]
            #[doc="parameter. This can be used with [SoloParamRepository::get] and similar methods"]
            #[doc="to load parameter data."]
            #[allow(non_camel_case_types)]
            pub struct $ParamType;

            impl SoloParam for $ParamType {
                const NAME: &'static str = stringify!($ParamType);
                const INDEX: u32 = $Index;
                type StructType = $StructType;
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
    (ObjectParam, OBJECT_PARAM_ST, 10),
    (Bullet, BULLET_PARAM_ST, 11),
    (BehaviorParam, BEHAVIOR_PARAM_ST, 12),
    (BehaviorParam_PC, BEHAVIOR_PARAM_ST, 13),
    (Magic, MAGIC_PARAM_ST, 14),
    (SpEffectParam, SP_EFFECT_PARAM_ST, 15),
    (SpEffectVfxParam, SP_EFFECT_VFX_PARAM_ST, 16),
    (TalkParam, TALK_PARAM_ST, 17),
    (MenuColorTableParam, MENU_PARAM_COLOR_TABLE_ST, 18),
    (ItemLotParam, ITEMLOT_PARAM_ST, 19),
    (MoveParam, MOVE_PARAM_ST, 20),
    (CharaInitParam, CHARACTER_INIT_PARAM, 21),
    (EquipMtrlSetParam, EQUIP_MTRL_SET_PARAM_ST, 22),
    (FaceGenParam, FACE_GEN_PARAM_ST, 23),
    (FaceParam, FACE_PARAM_ST, 24),
    (FaceRangeParam, FACE_RANGE_PARAM_ST, 25),
    (RagdollParam, RAGDOLL_PARAM_ST, 26),
    (ShopLineupParam, SHOP_LINEUP_PARAM, 27),
    (GameAreaParam, GAME_AREA_PARAM_ST, 28),
    (SkeletonParam, SKELETON_PARAM_ST, 29),
    (CalcCorrectGraph, CACL_CORRECT_GRAPH_ST, 30),
    (LockCamParam, LOCK_CAM_PARAM_ST, 31),
    (ObjActParam, OBJ_ACT_PARAM_ST, 32),
    (HitMtrlParam, HIT_MTRL_PARAM_ST, 33),
    (KnockBackParam, KNOCKBACK_PARAM_ST, 34),
    (DecalParam, DECAL_PARAM_ST, 35),
    (ActionButtonParam, ACTIONBUTTON_PARAM_ST, 36),
    (WeaponGenParam, WEAPON_GEN_PARAM_ST, 37),
    (ProtectorGenParam, PROTECTOR_GEN_PARAM_ST, 38),
    (GemGenParam, GEM_GEN_PARAM_ST, 39),
    (GemeffectParam, GEMEFFECT_PARAM_ST, 40),
    (GemCategoryParam, GEM_CATEGORY_PARAM_ST, 41),
    (GemDropDopingParam, GEM_DROP_DOPING_PARAM_ST, 42),
    (GemDropModifyParam, GEM_DROP_MODIFY_PARAM_ST, 43),
    (ModelSfxParam, MODEL_SFX_PARAM_ST, 44),
    (AiSoundParam, AI_SOUND_PARAM_ST, 45),
    (PlayRegionParam, PLAY_REGION_PARAM_ST, 46),
    (NetworkAreaParam, NETWORK_AREA_PARAM_ST, 47),
    (NetworkParam, NETWORK_PARAM_ST, 48),
    (NetworkMsgParam, NETWORK_MSG_PARAM_ST, 49),
    (BudgetParam, BUDGET_PARAM_ST, 50),
    (BonfireWarpParam, BONFIRE_WARP_PARAM_ST, 51),
    (MenuPropertySpecParam, MENUPROPERTY_SPEC, 52),
    (MenuPropertyLayoutParam, MENUPROPERTY_LAYOUT, 53),
    (MenuValueTableParam, MENU_VALUE_TABLE_SPEC, 54),
    (Ceremony, CEREMONY_PARAM_ST, 55),
    (PhantomParam, PHANTOM_PARAM_ST, 56),
    (CharMakeMenuTopParam, CHARMAKEMENUTOP_PARAM_ST, 57),
    (
        CharMakeMenuListItemParam,
        CHARMAKEMENU_LISTITEM_PARAM_ST,
        58
    ),
    (NewMenuColorTableParam, MENU_PARAM_COLOR_TABLE_ST, 59),
    (HitEffectSfxAngleParam, HIT_EFFECT_SFX_ANGLE_PARAM_ST, 60),
    (
        HitEffectSfxConceptParam,
        HIT_EFFECT_SFX_CONCEPT_PARAM_ST,
        61
    ),
    (
        HitEffectSfxConceptJustGuardParam,
        HIT_EFFECT_SFX_CONCEPT_PARAM_ST,
        62
    ),
    (HitEffectSfxParam, HIT_EFFECT_SFX_PARAM_ST, 63),
    (HitEffectSeParam, HIT_EFFECT_SE_PARAM_ST, 64),
    (HitEffectSeJustGuardParam, HIT_EFFECT_SE_PARAM_ST, 65),
    (HitEffectSeHitWallParam, HIT_EFFECT_SE_PARAM_ST, 66),
    (WepAbsorpPosParam, WEP_ABSORP_POS_PARAM_ST, 67),
    (ToughnessParam, TOUGHNESS_PARAM_ST, 68),
    (DirectionCameraParam, DIRECTION_CAMERA_PARAM_ST, 69),
    (RoleParam, ROLE_PARAM_ST, 70),
    (WetAspectParam, WET_ASPECT_PARAM_ST, 71),
    (CultSettingParam, CULT_SETTING_PARAM_ST, 72),
    (SwordArtsParam, SWORD_ARTS_PARAM_ST, 73),
    (HPEstusFlaskRecoveryParam, ESTUS_FLASK_RECOVERY_PARAM_ST, 74),
    (MPEstusFlaskRecoveryParam, ESTUS_FLASK_RECOVERY_PARAM_ST, 75),
    (MultiPlayCorrectionParam, MULTI_PLAY_CORRECTION_PARAM_ST, 76),
    (
        MapMimicryEstablishmentParam,
        MAP_MIMICRY_ESTABLISHMENT_PARAM_ST,
        77
    ),
    (UpperArmParam, UPPER_ARM_PARAM_ST, 78),
    (
        AttackElementCorrectParam,
        ATTACK_ELEMENT_CORRECT_PARAM_ST,
        79
    ),
    (ThrowDirectionSfxParam, THROW_DIRECTION_SFX_PARAM_ST, 80),
    (ThrowDirectionSeParam, THROW_DIRECTION_SE_PARAM_ST, 81),
    (ThrowDirectionDecalParam, THROW_DIRECTION_DECAL_PARAM_ST, 82),
    (FootSfxParam, FOOT_SFX_PARAM_ST, 83),
    (NpcAiActionParam, NPC_AI_ACTION_PARAM_ST, 84),
    (BulletCreateLimitParam, BULLET_CREATE_LIMIT_PARAM_ST, 85),
    (ClearCountCorrectParam, CLEAR_COUNT_CORRECT_PARAM_ST, 86),
    (GameProgressParam, GAME_PROGRESS_PARAM_ST, 87),
    (LoadBalancerParam, LOAD_BALANCER_PARAM_ST, 88),
    (ObjectMaterialSfxParam, OBJECT_MATERIAL_SFX_PARAM_ST, 89),
    (
        KnowledgeLoadScreenItemParam,
        KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST,
        90
    ),
    (MenuOffscrRendParam, MENU_OFFSCR_REND_PARAM_ST, 91),
    (
        MultiHPEstusFlaskBonusParam,
        MULTI_ESTUS_FLASK_BONUS_PARAM_ST,
        92
    ),
    (
        MultiMPEstusFlaskBonusParam,
        MULTI_ESTUS_FLASK_BONUS_PARAM_ST,
        93
    ),
    (MultiSoulBonusRateParam, MULTI_SOUL_BONUS_RATE_PARAM_ST, 94),
    (
        LoadBalancerDrawDistScaleParam,
        LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
        95
    ),
    (CoolTimeParam, COOL_TIME_PARAM_ST, 96),
    (GrassTypeParam, GRASS_TYPE_PARAM_ST, 97),
    (GrassLodRangeParam, GRASS_LOD_RANGE_PARAM_ST, 98),
    (DyingEffectParam, DYING_EFFECT_PARAM_ST, 99),
    (CameraParam, CAMERA_PARAM_ST, 100),
    (CameraSetParam, CAMERA_SET_PARAM_ST, 101),
    (WirePointSearchParam, WIRE_POINT_SEARCH_PARAM_ST, 102),
    (WireSetParam, WIRE_SET_PARAM_ST, 103),
    (
        ChrPhysicsVelocityChangeParam,
        CHR_PHYSICS_VELOCITY_CHANGE_ST,
        104
    ),
    (StaminaControlParam, STAMINA_CONTROL_PARAM_ST, 105),
    (MaterialExParam, MATERIAL_EX_PARAM_ST, 106),
    (ThrowKindParam, THROW_KIND_PARAM_ST, 107),
    (ResourceItemLotParam, RESOURCEITEMLOT_PARAM_ST, 108),
    (ResourceItemParam, RESOURCEITEM_PARAM_ST, 109),
    (WhiteSignCoolTimeParam, WHITE_SIGN_COOL_TIME_PARAM_ST, 110),
    (ChrPhysicsHomingParam, CHR_PHYSICS_HOMING_ST, 111),
    (WireVariationParam, WIRE_VARIATION_ST, 112),
    (ActionUnlockParam, ACTION_UNLOCK_PARAM_ST, 113),
    (
        HitMaterialSpecialSettingParam,
        CS_HIT_MATERIAL_SPECIAL_SETTING_PARAM_ST,
        114
    ),
    (CutsceneParam, CUTSCENE_PARAM_ST, 115),
    (DefaultKeyAssignParam00, DEFAULT_KEY_ASSIGN_PARAM_ST, 116),
    (DefaultKeyAssignParam01, DEFAULT_KEY_ASSIGN_PARAM_ST, 117),
    (DefaultKeyAssignParam02, DEFAULT_KEY_ASSIGN_PARAM_ST, 118),
    (DefaultKeyAssignParam03, DEFAULT_KEY_ASSIGN_PARAM_ST, 119),
    (DefaultKeyAssignParam04, DEFAULT_KEY_ASSIGN_PARAM_ST, 120),
    (MenuParam, MENU_PARAM_ST, 121),
    (EnemyCommonParam, ENEMY_COMMOM_PARAM_ST, 122),
    (TentativePlayerParam, TENTATIVE_PLAYER_PARAM_ST, 123),
    (GraphicsParam, GRAPHICS_PARAM_ST, 124),
    (GameSystemParam, GAME_SYSTEM_PARAM_ST, 125),
    (GraphicsConfig_ver2, CS_GRAPHICS_CONFIG_PARAM_ST, 126),
    (ActionGuideParam, ACTION_GUIDE_PARAM_ST, 127),
    (MapPartsParam, MAP_PARTS_PARAM_ST, 128),
    (SkillParam, SKILL_PARAM_ST, 129),
    (MenuTutorialParam, MENU_TUTORIAL_PARAM_ST, 130),
    (RematchWarpParam, REMATCH_WARP_PARAM_ST, 131),
    (Gconfig_AAQuality, CS_AA_QUALITY_DETAIL, 132),
    (Gconfig_AAQuality_ps4, CS_AA_QUALITY_DETAIL, 133),
    (Gconfig_AAQuality_stadia, CS_AA_QUALITY_DETAIL, 134),
    (Gconfig_AAQuality_xboxone, CS_AA_QUALITY_DETAIL, 135),
    (Gconfig_DecalQuality, CS_DECAL_QUALITY_DETAIL, 136),
    (Gconfig_DOFQuality, CS_DOF_QUALITY_DETAIL, 137),
    (Gconfig_DOFQuality_ps4, CS_DOF_QUALITY_DETAIL, 138),
    (Gconfig_DOFQuality_stadia, CS_DOF_QUALITY_DETAIL, 139),
);
