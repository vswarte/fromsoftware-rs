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
    PhysicsClassUnk907 = 907,
    BehaviorDataUnk908 = 908,
    Unk910 = 910,
    Unk911 = 911,
    Sfx = 10096,
    PlaySoundWanderGhostUnused = 10130,
    DebugDecal1 = 10137,
    DebugDecal2 = 10138,
}

/// Root TAE file header. `magic` is the base address used for pointer fixups.
#[repr(C)]
pub struct TAE_Header_Main {
    /// "TAE "
    pub magic: [u8; 4],
    pub big_endian: u8,
    pub unk05: u8,
    pub unk06: u8,
    pub is_64bit: u8,
    pub version: u32,
    pub file_size: u32,
    pub section10: *mut TAE_Header_Block10,
    pub anim_file_count: u32,
    pub anim_files: *mut TAE_Header_FileInfo,
    pub anim_file_groups_info: *mut TAE_Header_AnimFileGroupsInfo,
    pub tae_content_version: u8,
    pub unused: u64,
}

#[repr(C)]
pub struct TAE_Header_Block10 {
    pub unk_a00: [u8; 10],
    pub unk_a0a: u8,
    pub unk_a0b: u8,
    pub unk_a0c: u32,
}

/// Groups a range of file IDs to their [`TAE_Header_FileInfo`] entries.
#[repr(C)]
pub struct TAE_Header_AnimFileGroupsInfo {
    pub anim_file_group_count: u64,
    pub anim_file_groups: *mut TAE_Header_AnimFileGroup,
}

#[repr(C)]
pub struct TAE_Header_AnimFileGroup {
    pub start_file_id: u32,
    pub end_file_id: u32,
    pub file_infos: *mut TAE_Header_FileInfo,
}

#[repr(C)]
pub struct TAE_Header_FileInfo {
    pub file_id: i32,
    pub anim_count: i32,
    pub animations: *mut TAE_Animation,
    pub anim_groups: *mut TAE_AnimGroups,
    pub strings_info: *mut TAE_Header_StringsInfo,
    pub anim_count2: i32,
    /// Offset to the first [`TAE_AnimData`]
    pub tae_data_start_offset: u64,
}

/// Skeleton / SIB name string table
#[repr(C)]
pub struct TAE_Header_StringsInfo {
    pub unk_e00: u64,
    pub tae_header_strings: *mut TAE_Header_Strings,
}

#[repr(C)]
pub struct TAE_Header_Strings {
    pub skeleton_name_offset: u64,
    pub sib_name_offset: u64,
    pub unk_c0: u64,
    pub unk_c8: u64,
}

#[repr(C)]
pub struct TAE_AnimGroups {
    pub group_count: u64,
    pub groups: *mut TAE_AnimGroup,
}

/// Maps a contiguous range of animation IDs to their entries in the animations array.
#[repr(C)]
pub struct TAE_AnimGroup {
    pub start_id: i32,
    pub end_id: i32,
    pub animations: *mut TAE_Animation,
}

#[repr(C)]
pub struct TAE_Animation {
    pub id: u64,
    pub anim_data: *mut TAE_AnimData,
}

/// Core per-animation payload: events, event groups, time pool, and HKT reference.
#[repr(C)]
pub struct TAE_AnimData {
    pub events: *mut TAE_Event,
    pub event_groups: *mut TAE_EventGroup,
    pub times: *mut f32,
    pub anim_file: *mut TAE_AnimFile,
    pub event_count: u16,
    pub content_version: u8,
    pub event_group_count: u32,
    pub time_count: u32,
}

/// HKT animation file reference.
#[repr(C)]
pub struct TAE_AnimFile {
    pub reference: u64,
    pub unk8_offset: *mut u64,
    pub hkt_name_offset: u64,
    pub field3_0x18: u32,
    pub field4_0x1c: i32,
    pub field5_0x20: u64,
    pub field6_0x28: u64,
}

#[repr(C)]
pub struct TAE_Event {
    pub start_time: f32,
    pub _pad4: [u8; 4],
    pub end_time: f32,
    pub _padc: [u8; 4],
    pub event_data: *mut TAE_EventData,
}

#[repr(C)]
pub struct TAE_EventData {
    pub event_id: TaeAnimEventId,
    /// Event-specific type
    pub args: *mut (),
}

/// Groups all events of a single type within one animation.
#[repr(C)]
pub struct TAE_EventGroup {
    pub event_count: u16,
    pub content_version: u8,
    pub event_data_offsets: *mut u32,
    pub event_group_data: *mut TAE_EventGroupData,
    pub main_header: *mut TAE_Header_Main,
}

#[repr(C)]
pub struct TAE_EventGroupData {
    pub event_id: TaeAnimEventId,
    pub unk8_offset: u64,
}

impl TAE_Header_Main {
    /// All FileInfo blocks (one per embedded TAE file).
    pub fn anim_files(&self) -> &[TAE_Header_FileInfo] {
        if self.anim_files.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.anim_files, self.anim_file_count as usize) }
    }

    /// Mutable version.
    pub fn anim_files_mut(&mut self) -> &mut [TAE_Header_FileInfo] {
        if self.anim_files.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.anim_files, self.anim_file_count as usize) }
    }
}

impl TAE_Header_AnimFileGroupsInfo {
    pub fn anim_file_groups(&self) -> &[TAE_Header_AnimFileGroup] {
        if self.anim_file_groups.is_null() {
            return &[];
        }
        unsafe {
            std::slice::from_raw_parts(self.anim_file_groups, self.anim_file_group_count as usize)
        }
    }

    pub fn anim_file_groups_mut(&mut self) -> &mut [TAE_Header_AnimFileGroup] {
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

impl TAE_Header_FileInfo {
    /// All animations in this file.
    pub fn animations(&self) -> &[TAE_Animation] {
        if self.animations.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.animations, self.anim_count as usize) }
    }

    pub fn animations_mut(&mut self) -> &mut [TAE_Animation] {
        if self.animations.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.animations, self.anim_count as usize) }
    }
}

impl TAE_AnimGroups {
    pub fn groups(&self) -> &[TAE_AnimGroup] {
        if self.groups.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.groups, self.group_count as usize) }
    }

    pub fn groups_mut(&mut self) -> &mut [TAE_AnimGroup] {
        if self.groups.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.groups, self.group_count as usize) }
    }
}

impl TAE_AnimData {
    pub fn events(&self) -> &[TAE_Event] {
        if self.events.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.events, self.event_count as usize) }
    }

    pub fn events_mut(&mut self) -> &mut [TAE_Event] {
        if self.events.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.events, self.event_count as usize) }
    }

    pub fn event_groups(&self) -> &[TAE_EventGroup] {
        if self.event_groups.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.event_groups, self.event_group_count as usize) }
    }

    pub fn event_groups_mut(&mut self) -> &mut [TAE_EventGroup] {
        if self.event_groups.is_null() {
            return &mut [];
        }
        unsafe {
            std::slice::from_raw_parts_mut(self.event_groups, self.event_group_count as usize)
        }
    }

    /// Shared time pool; startTime/endTime in every [`TAE_Event`] were copied from here.
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

impl TAE_EventGroup {
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

    pub fn resolve_event(&self, index: usize) -> Option<&TAE_Event> {
        let offsets = self.event_offset();
        let offset = *offsets.get(index)? as usize;
        let base = self.main_header as usize;
        (base + offset)
            .ne(&0)
            .then(|| unsafe { &*((base + offset) as *const TAE_Event) })
    }

    pub fn resolve_event_mut(&mut self, index: usize) -> Option<&mut TAE_Event> {
        let offsets = self.event_offset();
        let offset = *offsets.get(index)? as usize;
        let base = self.main_header as usize;
        (base + offset)
            .ne(&0)
            .then(|| unsafe { &mut *((base + offset) as *mut TAE_Event) })
    }
}
