use pelite::pe64::Pe;

use shared::{OwnedPtr, RecurringTask, SharedTaskImp, program::Program};

use crate::fd4::FD4BasicHashString;
use crate::rva;

#[repr(C)]
#[shared::singleton("SprjTask")]
pub struct SprjTaskImp {
    vftable: usize,
    pub inner: OwnedPtr<SprjTask>,
}

// TODO: Track down exactly what Sekiro's FD4TaskData struct looks like.
impl SharedTaskImp<SprjTaskGroupIndex, usize> for SprjTaskImp {
    fn register_task_internal(&self, index: SprjTaskGroupIndex, task: &RecurringTask<usize>) {
        let va = Program::current()
            .rva_to_va(rva::get().register_task)
            .expect("Expected register_task to be in the exe");

        let register_task: extern "C" fn(
            &SprjTaskImp,
            SprjTaskGroupIndex,
            u64,
            &RecurringTask<usize>,
        ) = unsafe { std::mem::transmute(va) };
        register_task(self, index, 0, task);
    }
}

#[repr(C)]
#[shared::singleton("SprjTaskGroup")]
pub struct SprjTaskGroup {
    vftable: usize,
    pub task_groups: [OwnedPtr<SprjTimeLineTaskGroupIns>; 0x73],
}

#[repr(C)]
pub struct SprjTimeLineTaskGroupIns {
    vftable: usize,
    pub name: FD4BasicHashString,
    _unk48: u32,
    _unk4c: u32,
    _unk50: u32,
    _unk54: u32,
    _unk58: u32,
    _unk5c: u8,
}

#[repr(C)]
pub struct SprjTask {}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum SprjTaskGroupIndex {
    FrameBegin,
    FD4TaskMng,
    SystemStep,
    FileStep,
    ResStep,
    PadStep,
    ObjResUpdate,
    GameFlowStep,
    GameMan,
    TaskMan,
    FaceGenMan,
    FrpgNetMan,
    ResMan,
    Grass_BatchUpdate,
    Grass_ResourceLoad,
    Grass_ResourceCleanup,
    Grass_HitUpdate,
    WorldChrMan_PrePhysics,
    WireActionTargetMan,
    ChrIns_Prepare,
    AI_SimulationStep,
    ChrIns_PreBehavior,
    ChrIns_PreBehaviorSafe,
    HavokBehavior,
    HavokBehavior_SceneModifier,
    ChrIns_BehaviorSafe,
    ChrIns_PrePhysics,
    ChrIns_PrePhysicsSafe,
    TaskLineIdx_Sys,
    TaskLineIdx_Test,
    TaskLineIdx_InGame_InGameStep,
    RemoStep,
    TaskLineIdx_InGame_MoveMapStep,
    TaskLineIdx_InGame_TestNetStep,
    TaskLineIdx_InGame_InGameMenuStep,
    TaskLineIdx_InGame_TitleMenuStep,
    TaskLineIdx_InGame_CommonMenuStep,
    TaskLineIdx_NetworkFlowStep,
    ItemGetEffect,
    REMOTEMAN,
    InGameDebugViewer,
    HavokClothUpdate,
    HavokClothPostUpdate,
    LocationStep,
    LocationUpdate_PrePhysics,
    LocationUpdate_PrePhysics_Parallel,
    LocationUpdate_PrePhysics_Post,
    LocationUpdate_PostCloth,
    LocationUpdate_PostCloth_Parallel,
    LocationUpdate_PostCloth_Post,
    HavokWorldUpdate_ExecWorldSyncRequest,
    HavokWorldUpdate_Pre,
    HavokWorldUpdate_Post,
    ChrIns_PreCloth,
    ChrIns_PreClothSafe,
    HavokClothUpdate_Pre_AddRemoveRigidBody,
    HavokClothUpdate_Pre_ClothModelInsSafe,
    HavokClothUpdate_Pre_ClothModelIns,
    HavokClothUpdate_Pre_ClothManager,
    WireActionTargetMan_CalcHasThingOnWireMoveOrbit,
    GetNPAuthCode,
    HavokAi_SilhouetteGeneratorHelper_Begin,
    HavokAi_SilhouetteGeneratorHelper_End,
    SoundStep,
    HavokClothUpdate_Post_ClothManager,
    HavokClothUpdate_Post_ClothModelIns,
    HavokClothUpdate_Post_UpdateVertex,
    HavokClothVertexUpdateFinishWait,
    ChrIns_PostPhysics,
    ChrIns_PostPhysicsSafe,
    WorldChrMan_PostPhysics,
    GameFlowInGame_MoveMap_PostPhysics_0,
    ChrIns_UpdateDraw_Begin,
    ChrIns_UpdateDraw,
    ChrIns_UpdateDraw_End,
    GameFlowInGame_MoveMap_PostPhysics_1,
    BloodMessageCreateStepIns,
    BloodMessageCreater,
    BloodMessageCreateGhostWatcher,
    MenuMan,
    HavokAi_World,
    BackreadRequestUpdate,
    CheckStabilizeBackread,
    CameraStep,
    DrawParamUpdate,
    AiCollectGabage,
    GameFlowInGame_TestNet,
    GameFlowInGame_InGameMenu,
    GameFlowInGame_TitleMenu,
    GameFlowInGame_CommonMenu,
    GameFlowFrpgNet_Sys,
    GameFlowFrpgNet_Lobby,
    GameFlowFrpgNet_ConnectMan,
    GameFlowFrpgNet_Connect,
    DarkSight,
    ScaleformStep,
    Draw_Pre,
    UpdateLodWorld,
    UpdateLodIns,
    GraphicsStep,
    DebugDrawMemoryBar,
    DbgMenuStep,
    DbgRemoteStep,
    PlaylogSystemStep,
    ReportSystemStep,
    DbgDispStep,
    DrawStep,
    DrawBegin,
    GameSceneDraw,
    AdhocDraw,
    DrawEnd,
    Flip,
    DelayDeleteStep,
    RecordHeapStats,
    FrameEnd,
}
