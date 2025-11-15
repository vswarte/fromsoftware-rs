use pelite::{pattern, pattern::Atom, pe64::Pe};
use std::{ffi, marker::PhantomData, sync::LazyLock};
use vtable_rs::VPtr;

use shared::{program::Program, OwnedPtr, RecurringTask, SharedTaskImp};

use crate::fd4::FD4BasicHashString;
use crate::rva;

#[repr(C)]
#[shared::singleton("SprjTask")]
pub struct SprjTaskImp {
    vftable: usize,
    pub inner: OwnedPtr<SprjTask>,
}

static REGISTER_TASK_VA: LazyLock<u64> = LazyLock::new(|| {
    Program::current()
        .rva_to_va(rva::get().register_task)
        .expect("Call target for REGISTER_TASK_VA was not in exe")
});

// TODO: Track down exactly what DS3's FD4TaskData struct looks like.
impl SharedTaskImp<SprjTaskGroupIndex, usize> for SprjTaskImp {
    fn register_task_internal(&self, index: SprjTaskGroupIndex, task: &RecurringTask<usize>) {
        let register_task: extern "C" fn(
            &SprjTaskImp,
            SprjTaskGroupIndex,
            u64,
            &RecurringTask<usize>,
        ) = unsafe { std::mem::transmute(*REGISTER_TASK_VA) };
        register_task(self, index, 0, task);
    }
}

#[repr(C)]
#[shared::singleton("SprjTaskGroup")]
pub struct SprjTaskGroup {
    vftable: usize,
    pub task_groups: [OwnedPtr<SprjTimeLineTaskGroupIns>; 0x61],
}

#[repr(C)]
pub struct SprjTimeLineTaskGroupIns {
    vftable: usize,
    pub name: FD4BasicHashString,
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
    MenuMan,
    GameMan,
    TaskMan,
    FaceGenMan,
    FrpgNetMan,
    ResMan,
    WorldChrMan_PrePhysics,
    ChrIns_Prepare,
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
    REMOTEMAN,
    InGameDebugViewer,
    HavokClothUpdate,
    AdhocHavokClothPostUpdate,
    LocationStep,
    LocationUpdate_PrePhysics,
    LocationUpdate_PostCloth,
    HavokWorldUpdate_Pre,
    HavokWorldUpdate_Post,
    ChrIns_PreCloth,
    ChrIns_PreClothSafe,
    HavokClothUpdate_Pre_AddRemoveRigidBody,
    HavokClothUpdate_Pre_ClothModelInsSafe,
    HavokClothUpdate_Pre_ClothModelIns,
    HavokClothUpdate_Pre_ClothManager,
    ScaleformStep,
    ScaleformCapture,
    ScaleformCaptureFE,
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
