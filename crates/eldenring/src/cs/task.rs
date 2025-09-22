use std::ptr::NonNull;
use std::{ffi, marker::PhantomData};
use vtable_rs::VPtr;
use windows::core::PCWSTR;

use crate::dlkr::DLPlainConditionSignal;
use crate::dlrf::DLRuntimeClass;
use crate::fd4::{FD4TaskBase, FD4TaskBaseVmt, FD4TaskData};
use crate::{
    dlkr::DLPlainLightMutex,
    fd4::{FD4BasicHashString, FD4Time},
    Tree, Vector,
};
use shared::OwnedPtr;

#[vtable_rs::vtable]
pub trait CSEzTaskVmt: FD4TaskBaseVmt {
    /// Called by execute() in the case of CSEzTask.
    fn eztask_execute(&mut self, data: &FD4TaskData);
    /// Called to register the task to the appropriate runtime.
    fn register_task(&mut self);
    /// Called to clean up the task.
    fn free_task(&mut self);
}

#[vtable_rs::vtable]
pub trait CSEzTaskProxyVmt: FD4TaskBaseVmt {
    fn get_task_group(&self) -> CSTaskGroupIndex;
}

#[repr(C)]
pub struct CSEzTask {
    pub vftable: VPtr<dyn CSEzTaskVmt, Self>,
    unk8: u32,
    _padc: u32,
    pub task_proxy: NonNull<CSEzTaskProxy>,
}

#[repr(C)]
pub struct CSEzRabbitTaskBase {
    pub ez_task: CSEzTask,
    unk18: u32,
    _pad1c: u32,
}

#[repr(C)]
pub struct CSEzRabbitNoUpdateTask {
    pub ez_rabbit_task_base: CSEzRabbitTaskBase,
}

/// Often used by the game to periodically run some update on a structure.
#[repr(C)]
pub struct CSEzUpdateTask<TEzTask, TSubject> {
    pub base_task: TEzTask,

    /// Whatever this update task is operating on
    pub subject: NonNull<TSubject>,

    /// Takes in the subject and the delta time
    pub executor: fn(&TSubject, &FD4Time),
}

#[repr(C)]
pub struct CSEzRabbitTask {
    pub base: CSEzTask,
    unk18: u32,
    unk1c: u32,
}

#[repr(C)]
pub struct CSEzVoidTask<TEzTask, TSubject> {
    pub base_task: TEzTask,

    /// Whatever this update task is operating on
    pub subject: NonNull<TSubject>,

    /// Takes in the subject and the delta time
    pub executor: fn(&TSubject, f32),
}

#[repr(C)]
pub struct CSEzTaskProxy {
    vftable: VPtr<dyn CSEzTaskProxyVmt, Self>,
    unk8: u32,
    _padc: u32,
    pub task: Option<NonNull<CSEzTask>>,
}

#[repr(C)]
#[dlrf::singleton("CSTaskGroup")]
pub struct CSTaskGroup {
    vftable: usize,
    pub task_groups: [OwnedPtr<CSTimeLineTaskGroupIns>; 168],
}

#[repr(C)]
pub struct CSTaskGroupIns {
    vftable: usize,
    pub name: FD4BasicHashString,
    unk40: [u8; 0x10],
}

#[repr(C)]
pub struct CSTimeLineTaskGroupIns {
    pub base: CSTaskGroupIns,
    pub step_impl: usize,
    unk60: [u8; 0x20],
}

#[repr(C)]
#[dlrf::singleton("CSTask")]
pub struct CSTaskImp {
    vftable: usize,
    pub inner: OwnedPtr<CSTask>,
}

#[repr(C)]
pub struct CSTaskBase {
    vftable: usize,
    allocator: usize,
    pub task_groups: Vector<TaskGroupEntry>,
    pub task_group_index_max: u32,
    _pad34: u32,
}

#[repr(C)]
pub struct TaskGroupEntry {
    pub index: u32,
    pub name: [u16; 64],
    pub active: bool,
}

#[repr(C)]
pub struct CSTask {
    pub task_base: CSTaskBase,
    allocator: usize,
    unk40: usize,
    unk48: [usize; 3],
    unk60: [usize; 3],
    pub task_runner_manager: OwnedPtr<CSTaskRunnerManager>,
    pub task_runners: [OwnedPtr<CSTaskRunner>; 6],
    pub task_runners_ex: [OwnedPtr<CSTaskRunnerEx>; 6],
    unke0: usize,
}

#[repr(C)]
pub struct CSTaskRunner {
    vftable: usize,
    task_queue: usize,
    pub task_runner_manager: OwnedPtr<CSTaskRunnerManager>,
    unk18: u32,
    _pad1c: u32,
    unk_string: PCWSTR,
}

#[repr(C)]
pub struct CSTaskRunnerEx {
    // TODO
}

#[repr(C)]
pub struct CSTaskRunnerManager {
    allocator: usize,
    pub concurrent_task_group_count: usize,
    pub concurrent_task_group_policy: OwnedPtr<TaskGroupConcurrency>,
    pub current_concurrent_task_group: u32,
    unk1c: u32,
    unk20: u32,
    _pad24: u32,
    pub mutex: DLPlainLightMutex,
    pub signals: [DLPlainConditionSignal; 6],
    unkb8: u32,
    unkbc: u32,
    unkc0: u32,
    unkc4: u32,
    unkc8: u32,
    unkcc: u32,
    unkd0: u32,
    unkd4: u32,
}

#[repr(C)]
pub struct TaskGroupConcurrency {
    pub slots: [TaskGroupConcurrencySlot; 6],
}

#[repr(C)]
pub struct TaskGroupConcurrencySlot {
    pub task_group_index: u32,
    pub task_group_concurrency_type: u32,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum CSTaskGroupIndex {
    FrameBegin,
    SteamThread0,
    SteamThread1,
    SteamThread2,
    SteamThread3,
    SteamThread4,
    SteamThread5,
    SystemStep,
    ResStep,
    PadStep,
    GameFlowStep,
    EndShiftWorldPosition,
    GameMan,
    TaskLineIdx_Sys,
    TaskLineIdx_Test,
    TaskLineIdx_NetworkFlowStep,
    TaskLineIdx_InGame_InGameStep,
    TaskLineIdx_InGame_InGameStayStep,
    MovieStep,
    RemoStep,
    TaskLineIdx_InGame_MoveMapStep,
    FieldArea_EndWorldAiManager,
    EmkSystem_Pre,
    EmkSystem_ConditionStatus,
    EmkSystem_Post,
    EventMan,
    FlverResDelayDelectiionBegin,
    TaskLineIdx_InGame_FieldAreaStep,
    TaskLineIdx_InGame_TestNetStep,
    TaskLineIdx_InGame_InGameMenuStep,
    TaskLineIdx_InGame_TitleMenuStep,
    TaskLineIdx_InGame_CommonMenuStep,
    TaskLineIdx_FrpgNet_Sys,
    TaskLineIdx_FrpgNet_Lobby,
    TaskLineIdx_FrpgNet_ConnectMan,
    TaskLineIdx_FrpgNet_Connect,
    TaskLineIdx_FrpgNet_Other,
    SfxMan,
    FaceGenMan,
    FrpgNetMan,
    NetworkUserManager,
    SessionManager,
    BlockList,
    LuaConsoleServer,
    RmiMan,
    ResMan,
    SfxDebugger,
    REMOTEMAN,
    Geom_WaitActivateFade,
    Geom_UpdateDraw,
    Grass_BatchUpdate,
    Grass_ResourceLoadKick,
    Grass_ResourceLoad,
    Grass_ResourceCleanup,
    WorldChrMan_Respawn,
    WorldChrMan_Prepare,
    ChrIns_CalcUpdateInfo_PerfBegin,
    ChrIns_CalcUpdateInfo,
    ChrIns_CalcUpdateInfo_PerfEnd,
    WorldChrMan_PrePhysics,
    WorldChrMan_CalcOmissionLevel_Begin,
    WorldChrMan_CalcOmissionLevel,
    WorldChrMan_CalcOmissionLevel_End,
    WorldChrMan_ConstructUpdateList,
    WorldChrMan_ChrNetwork,
    ChrIns_Prepare,
    ChrIns_NaviCache,
    ChrIns_AILogic_PerfBegin,
    ChrIns_AILogic,
    ChrIns_AILogic_PerfEnd,
    AI_SimulationStep,
    ChrIns_PreBehavior,
    ChrIns_PreBehaviorSafe,
    GeomModelInsCreatePartway_Begin,
    HavokBehavior,
    GeomModelInsCreatePartway_End,
    ChrIns_BehaviorSafe,
    ChrIns_PrePhysics_Begin,
    ChrIns_PrePhysics,
    ChrIns_PrePhysics_End,
    NetFlushSendData,
    ChrIns_PrePhysicsSafe,
    ChrIns_RagdollSafe,
    ChrIns_GarbageCollection,
    GeomModelInsCreate,
    AiBeginCollectGabage,
    WorldChrMan_Update_RideCheck,
    InGameDebugViewer,
    LocationStep,
    LocationUpdate_PrePhysics,
    LocationUpdate_PrePhysics_Parallel,
    LocationUpdate_PrePhysics_Post,
    LocationUpdate_PostCloth,
    LocationUpdate_PostCloth_Parallel,
    LocationUpdate_PostCloth_Post,
    LocationUpdate_DebugDraw,
    EventCondition_BonfireNearEnemyCheck,
    HavokWorldUpdate_Pre,
    RenderingSystemUpdate,
    HavokWorldUpdate_Post,
    ChrIns_PreCloth,
    ChrIns_PreClothSafe,
    HavokClothUpdate_Pre_AddRemoveRigidBody,
    HavokClothUpdate_Pre_ClothModelInsSafe,
    HavokClothUpdate_Pre_ClothModelIns,
    HavokClothUpdate_Pre_ClothManager,
    CameraStep,
    DrawParamUpdate,
    GetNPAuthCode,
    SoundStep,
    HavokClothUpdate_Post_ClothManager,
    HavokClothUpdate_Post_ClothModelIns,
    HavokClothVertexUpdateFinishWait,
    ChrIns_PostPhysics,
    ChrIns_PostPhysicsSafe,
    CSDistViewManager_Update,
    HavokAi_SilhouetteGeneratorHelper_Begin,
    WorldChrMan_PostPhysics,
    GameFlowInGame_MoveMap_PostPhysics_0,
    HavokAi_SilhouetteGeneratorHelper_End,
    DmgMan_Pre,
    DmgMan_ShapeCast,
    DmgMan_Post,
    GameFlowInGame_MoveMap_PostPhysics_1_Core0,
    GameFlowInGame_MoveMap_PostPhysics_1_Core1,
    GameFlowInGame_MoveMap_PostPhysics_1_Core2,
    MenuMan,
    WorldChrMan_Update_BackreadRequestPre,
    ChrIns_Update_BackreadRequest,
    WorldChrMan_Update_BackreadRequestPost,
    HavokAi_World,
    WorldAiManager_BeginUpdateFormation,
    WorldAiManager_EndUpdateFormation,
    GameFlowInGame_TestNet,
    GameFlowInGame_InGameMenu,
    GameFlowInGame_TitleMenu,
    GameFlowInGame_CommonMenu,
    GameFlowFrpgNet_Sys,
    GameFlowFrpgNet_Lobby,
    GameFlowFrpgNet_ConnectMan,
    GameFlowFrpgNet_Connect,
    GameFlowStep_Post,
    ScaleformStep,
    FlverResDelayDelectiionEnd,
    Draw_Pre,
    GraphicsStep,
    DebugDrawMemoryBar,
    DbgMenuStep,
    DbgRemoteStep,
    PlaylogSystemStep,
    ReviewMan,
    ReportSystemStep,
    DbgDispStep,
    DrawStep,
    DrawBegin,
    GameSceneDraw,
    AdhocDraw,
    DrawEnd,
    Draw_Post,
    SoundPlayLimitterUpdate,
    BeginShiftWorldPosition,
    FileStep,
    FileStepUpdate_Begin,
    FileStepUpdate_End,
    Flip,
    DelayDeleteStep,
    AiEndCollectGabage,
    RecordHeapStats,
    FrameEnd,
}
