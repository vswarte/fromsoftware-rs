#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TaeAnimEventId {
    ChrActionFlag = 0,
    AttackBehavior = 1,
    BulletBehavior = 2,
    CommonBehavior = 5,
    Event14 = 14,
    Blend = 16,
    Event17 = 17,
    DS1DisabledQueueTransitionAnimation = 24,
    SetWeaponStyle = 32,
    SwitchWeapon = 33,
    UnequipCrossbowBolt = 34,
    EquipCrossbowBolt = 35,
    CastHighlightedMagic = 64,
    ConsumeCurrentGoods = 65,
    AddSpEffectMultiplayer = 66,
    AddSpEffect = 67,
    DS3SpawnOneShotFFXEmber = 95,
    SpawnOneShotFFX = 96,
    DS3SpawnFFX104 = 104,
    SpawnFFXGeneral = 110,
    SpawnFFXFloorDetermined = 112,
    UnkType113 = 113,
    SpawnFFXGoodsAndMagic = 114,
    SpawnFFXGoodsAndMagicEX = 115,
    SpawnFFXThrow = 116,
    SpawnFFXThrowDirection = 117,
    SpawnFFXBlade = 118,
    SpawnFFXBodyForEventDuration = 119,
    SpawnFFXChrType = 120,
    DS3SpawnFFX121 = 121,
    DS3SpawnFFXBySpEffect1 = 122,
    DS3SpawnFFXBySpEffect2 = 123,
    WwisePlaySoundCenterBody = 128,
    WwisePlaySoundBySlot = 129,
    DS3PlaySoundWeapon = 132,
    WwisePlaySoundUnk133 = 133,
    WwisePlaySoundUnk134 = 134,
    DS3DecalParamIDCenterBody = 137,
    DecalParamIDDummyPoly = 138,
    DecalParamIDUnk = 139,
    RumbleCamLocal = 144,
    RumbleCamGlobal = 145,
    SetLockCamParamSelf = 150,
    SetCameraFollowDummyPoly = 151,
    CameraZoomOut = 152,
    ForceCameraDirection = 153,
    SetLockCamParamTarget = 155,
    DS3SekiroCode4 = 160,
    DS3DecalOnLanding = 161,
    DS3DebugFadeOut = 192,
    SetOpacityKeyframe = 193,
    DS3DebugStringPrintCARSNBumpBlendDecal = 196,
    DS3FadeOut = 197,
    ModelParamModifier198 = 198,
    DS3ModelParamModifier199 = 199,
    DS3ModelParamModifier200 = 200,
    SetTurnSpeed = 224,
    SetSPRegenRatePercent = 225,
    SetKnockbackPercent = 226,
    EventEzStateFlagHKSEnv301 = 227,
    RagdollReviveTime = 228,
    SpawnAISoundAlternative = 229,
    DS3SetFPRegenRatePercent = 230,
    RequestMsgMapList = 231,
    PastGamesAllowVerticalTorsoAim = 232,
    ChangeChrDrawMask = 233,
    PastGamesAddOffsetToNextAnimID = 234,
    Event235 = 235,
    RootMotionReduction = 236,
    SpawnAISound = 237,
    SetBulletAimAngle = 238,
    ActivateChrActionFlagEarly = 300,
    DS1Unknown301 = 301,
    AddSpEffectDragonForm = 302,
    DS3Behavior303 = 303,
    ThrowAttackBehavior = 304,
    PCBehavior = 307,
    ChrClothState = 310,
    Event311 = 311,
    Event312 = 312,
    AllowInput = 320,
    WeaponArtFPConsumption = 330,
    AddSpEffectWeaponArts = 331,
    WeaponArtWeaponStyleCheck = 332,
    HavokUnk339 = 339,
    SpawnNpcItemLot = 340,
    ChrSlotSys341 = 341,
    SetSuperArmorDurabilityMultiplier = 342,
    DoSomethingAndDebugDisplay343 = 343,
    Havok344 = 344,
    AddSpEffectMultiplayer401 = 401,
    Event500 = 500,
    SetSpEffectWetConditionDepth = 511,
    SetSpecialLockOnParameter = 522,
    EnableBehaviorFlags = 600,
    SetAdditiveAnim = 601,
    DS3Event602 = 602,
    ExePatchDebugAnimSpeed = 603,
    ExePatchTestParam = 604,
    SetTimeActEditorHavokVariable = 605,
    JiggleModifier = 606,
    DS3AdditiveAnimPlaybackUnk607 = 607,
    AnimSpeedGradient = 608,
    AdditiveAnimPlaybackUnk609 = 609,
    EnableTwistModifier = 700,
    BehaviorDataUnk702 = 702,
    FixedRotationDirection = 703,
    ChrTurnSpeedEX = 704,
    FacingAngleCorrection = 705,
    ChrTurnSpeedForLock = 706,
    ManualAttackAiming = 707,
    HideEquippedWeapon = 710,
    HideModelMask = 711,
    OverrideWeaponModelLocations = 712,
    ShowModelMask = 713,
    StaggerModuleUnk714 = 714,
    DS3WepAbsorpPos = 715,
    DS3HksEngineFlag716 = 716,
    SetJointTurnSpeed = 717,
    Event718 = 718,
    ActionFlagUnk730 = 730,
    ActionFlagUnk731 = 731,
    Unk740 = 740,
    BoostRootMotionToReachTarget = 760,
    AIModule761 = 761,
    DS3ActionFlag770ChangeBonsPos = 770,
    DS3FixBone = 771,
    TurnLowerBody = 781,
    AiReplanningCtrlReset = 782,
    DS3SpawnChrFinderBullet = 785,
    ActionFlag787 = 787,
    Event788 = 788,
    Event789 = 789,
    DisableDefaultWeaponTrail = 790,
    PartDamageAdditiveBlendInvalid = 791,
    FootSfxParamEntity = 792,
    DS3Poise = 795,
    SetMovementMultiplier = 800,
    AttachChrToRidingMount = 900,
    TransferCameraControlToMount = 901,
    UnkRideTurningEvent902 = 902,
    HavokThrowUnk903 = 903,
    UnkHkAi904 = 904,
    UnkHkAiPos905 = 905,
    RideStartUpChrAttachment = 906,
    LimitMaxHorizontalFallSpeed = 907,
    BehaviorDataUnk908 = 908,
    Unk910 = 910,
    Unk911 = 911,
    Sfx = 10096,
    PlaySoundWanderGhostUnused = 10130,
    DebugDecal1 = 10137,
    DebugDecal2 = 10138,
}

/// The TAE file header.
///
/// All offsets are relative to the start of this structure.
#[repr(C)]
pub struct TaeHeader {
    /// "TAE "
    pub magic: [u8; 4],
    pub big_endian: u8,
    unk5: u8,
    unk6: u8,
    pub is_64bit: u8,
    pub version: u32,
    pub file_size: u32,
    pub section10: *mut TaeBlock10,
    pub anim_file_count: u32,
    pub anim_files: *mut TaeFileInfo,
    pub anim_file_groups_info: *mut TaeAnimFileGroupsInfo,
    pub content_version: u8,
    pub unused: u64,
}

#[repr(C)]
pub struct TaeBlock10 {
    unk0: [u8; 10],
    unka: u8,
    unkb: u8,
    unkc: u32,
}

/// A mapping between file IDs and their [`TaeFileInfo`] entries.
#[repr(C)]
pub struct TaeAnimFileGroupsInfo {
    pub anim_file_group_count: u64,
    pub anim_file_groups: *mut TaeAnimFileGroup,
}

#[repr(C)]
pub struct TaeAnimFileGroup {
    pub start_file_id: u32,
    pub end_file_id: u32,
    pub file_infos: *mut TaeFileInfo,
}

#[repr(C)]
pub struct TaeFileInfo {
    pub file_id: i32,
    pub anim_count: i32,
    pub animations: *mut TaeAnimation,
    pub anim_groups: *mut TaeAnimGroups,
    pub strings_info: *mut TaeStringsInfo,
    pub anim_count2: i32,
    /// Offset to the first [`TaeAnimData`] relative to the start of [`TaeHeader`].
    pub tae_data_start_offset: u64,
}

/// A mapping between skeleton names and SIB names.
#[repr(C)]
pub struct TaeStringsInfo {
    unke00: u64,
    pub tae_header_strings: *mut TaeStrings,
}

#[repr(C)]
pub struct TaeStrings {
    /// Offset to the skeleton hkt name unicode string, relative to the start of [`TaeHeader`].
    pub skeleton_name_offset: u64,
    /// Offset to the sib name unicode string, relative to the start of [`TaeHeader`].
    pub sib_name_offset: u64,
    unkc0: u64,
    unkc8: u64,
}

#[repr(C)]
pub struct TaeAnimGroups {
    pub group_count: u64,
    pub groups: *mut TaeAnimGroup,
}

/// Maps a contiguous range of animation IDs to their entries in the animations array.
#[repr(C)]
pub struct TaeAnimGroup {
    pub start_id: i32,
    pub end_id: i32,
    pub animations: *mut TaeAnimation,
}

#[repr(C)]
pub struct TaeAnimation {
    pub id: u64,
    pub anim_data: *mut TaeAnimData,
}

/// The core per-animation payload.
#[repr(C)]
pub struct TaeAnimData {
    pub events: *mut TaeEvent,
    pub event_groups: *mut TaeEventGroup,
    pub times: *mut f32,
    pub anim_file: *mut TaeAnimFile,
    pub event_count: u16,
    pub content_version: u8,
    pub event_group_count: u32,
    pub time_count: u32,
}

/// An HKT animation file.
#[repr(C)]
pub struct TaeAnimFile {
    pub reference: u64,
    /// Pointer to [`Self::hkt_name_offset`].
    pub hkt_name_offset_ptr: *mut u64,
    /// Offset to hkt file name unicode string, relative to the start of [`TaeHeader`].
    pub hkt_name_offset: u64,
    unk18: u32,
    unk1c: i32,
    unk20: u64,
    unk28: u64,
}

#[repr(C)]
pub struct TaeEvent {
    pub start_time: f32,
    _pad4: [u8; 4],
    pub end_time: f32,
    _padc: [u8; 4],
    pub event_data: *mut TaeEventData,
}

#[repr(C)]
pub struct TaeEventData {
    pub event_id: TaeAnimEventId,
    /// Event-specific type
    pub args: *mut (),
}

/// A group of all events of a single type within one animation.
#[repr(C)]
pub struct TaeEventGroup {
    pub event_count: u16,
    pub content_version: u8,
    pub event_data_offsets: *mut u32,
    pub event_group_data: *mut TaeEventGroupData,
    pub main_header: *mut TaeHeader,
}

#[repr(C)]
pub struct TaeEventGroupData {
    pub event_id: TaeAnimEventId,
    /// Offset to some unknown, never used in real files structure. Always 0.
    pub unk8_offset: u64,
}

impl TaeHeader {
    /// All FileInfo blocks (one per embedded TAE file).
    pub fn anim_files(&self) -> &[TaeFileInfo] {
        if self.anim_files.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.anim_files, self.anim_file_count as usize) }
    }

    /// All FileInfo blocks (one per embedded TAE file).
    pub fn anim_files_mut(&mut self) -> &mut [TaeFileInfo] {
        if self.anim_files.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.anim_files, self.anim_file_count as usize) }
    }
}

impl TaeAnimFileGroupsInfo {
    pub fn anim_file_groups(&self) -> &[TaeAnimFileGroup] {
        if self.anim_file_groups.is_null() {
            return &[];
        }
        unsafe {
            std::slice::from_raw_parts(self.anim_file_groups, self.anim_file_group_count as usize)
        }
    }

    pub fn anim_file_groups_mut(&mut self) -> &mut [TaeAnimFileGroup] {
        if self.anim_file_groups.is_null() {
            return &mut [];
        }
        unsafe {
            std::slice::from_raw_parts_mut(
                self.anim_file_groups,
                self.anim_file_group_count as usize,
            )
        }
    }
}

impl TaeFileInfo {
    /// All animations in this file.
    pub fn animations(&self) -> &[TaeAnimation] {
        if self.animations.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.animations, self.anim_count as usize) }
    }

    pub fn animations_mut(&mut self) -> &mut [TaeAnimation] {
        if self.animations.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.animations, self.anim_count as usize) }
    }
}

impl TaeAnimGroups {
    pub fn groups(&self) -> &[TaeAnimGroup] {
        if self.groups.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.groups, self.group_count as usize) }
    }

    pub fn groups_mut(&mut self) -> &mut [TaeAnimGroup] {
        if self.groups.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.groups, self.group_count as usize) }
    }
}

impl TaeAnimData {
    pub fn events(&self) -> &[TaeEvent] {
        if self.events.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.events, self.event_count as usize) }
    }

    pub fn events_mut(&mut self) -> &mut [TaeEvent] {
        if self.events.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.events, self.event_count as usize) }
    }

    pub fn event_groups(&self) -> &[TaeEventGroup] {
        if self.event_groups.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.event_groups, self.event_group_count as usize) }
    }

    pub fn event_groups_mut(&mut self) -> &mut [TaeEventGroup] {
        if self.event_groups.is_null() {
            return &mut [];
        }
        unsafe {
            std::slice::from_raw_parts_mut(self.event_groups, self.event_group_count as usize)
        }
    }

    /// Shared time pool; startTime/endTime in every [`TaeEvent`] were copied from here.
    pub fn times(&self) -> &[f32] {
        if self.times.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.times, self.time_count as usize) }
    }

    pub fn times_mut(&mut self) -> &mut [f32] {
        if self.times.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.times, self.time_count as usize) }
    }
}

impl TaeEventGroup {
    pub fn event_offset(&self) -> &[u32] {
        if self.event_data_offsets.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.event_data_offsets, self.event_count as usize) }
    }

    pub fn event_offsets_mut(&mut self) -> &mut [u32] {
        if self.event_data_offsets.is_null() {
            return &mut [];
        }
        unsafe {
            std::slice::from_raw_parts_mut(self.event_data_offsets, self.event_count as usize)
        }
    }

    pub fn get_event(&self, index: usize) -> Option<&TaeEvent> {
        let offsets = self.event_offset();
        let offset = *offsets.get(index)? as usize;
        let base = self.main_header as usize;
        (base + offset)
            .ne(&0)
            .then(|| unsafe { &*((base + offset) as *const TaeEvent) })
    }

    pub fn get_event_mut(&mut self, index: usize) -> Option<&mut TaeEvent> {
        let offsets = self.event_offset();
        let offset = *offsets.get(index)? as usize;
        let base = self.main_header as usize;
        (base + offset)
            .ne(&0)
            .then(|| unsafe { &mut *((base + offset) as *mut TaeEvent) })
    }
}
