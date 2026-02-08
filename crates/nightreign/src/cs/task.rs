use pelite::pe64::Pe;
use std::sync::LazyLock;

use shared::{OwnedPtr, RecurringTask, SharedTaskImp, program::Program};

use crate::fd4::{FD4BasicHashString, FD4TaskData};
use crate::rva;

#[repr(C)]
#[shared::singleton("CSTask")]
pub struct CSTaskImp {
    vftable: usize,
    pub inner: OwnedPtr<CSTask>,
}

static REGISTER_TASK_VA: LazyLock<u64> = LazyLock::new(|| {
    Program::current()
        .rva_to_va(rva::get().register_task)
        .expect("Call target for REGISTER_TASK_VA was not in exe")
});

// TODO: Track down exactly what DS3's FD4TaskData struct looks like.
impl SharedTaskImp<CSTaskGroupIndex, FD4TaskData> for CSTaskImp {
    fn register_task_internal(&self, index: CSTaskGroupIndex, task: &RecurringTask<FD4TaskData>) {
        let register_task: extern "C" fn(
            &CSTaskImp,
            CSTaskGroupIndex,
            &RecurringTask<FD4TaskData>,
        ) = unsafe { std::mem::transmute(*REGISTER_TASK_VA) };

        register_task(self, index, task);
    }
}

#[repr(C)]
#[shared::singleton("CSTaskGroup")]
pub struct CSTaskGroup {
    vftable: usize,
    pub task_groups: [OwnedPtr<CSTimeLineTaskGroupIns>; 0x61],
}

#[repr(C)]
pub struct CSTimeLineTaskGroupIns {
    vftable: usize,
    pub name: FD4BasicHashString,
}

#[repr(C)]
pub struct CSTask {}

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
	FaceGenMan,
	FrpgNetMan,
	NetworkUserManager,
	SessionManager,
	BlockList,
	LuaConsoleServer,
	ResMan,
	REMOTEMAN,
	Geom_WaitActivateFade,
	Geom_UpdateDraw,
	Geom_UpdateGridDistance,
	Grass_BatchUpdate,
	Grass_ResourceLoadKick,
	Grass_ResourceLoad,
	Grass_ResourceCleanup,
	WorldChrMan_Respawn,
	WorldChrMan_Prepare,
	ChrIns_CalcUpdateInfo,
	WorldChrMan_PrePhysics,
	WorldChrMan_CalcOmissionLevel_Begin,
	WorldChrMan_CalcOmissionLevel,
	WorldChrMan_CalcOmissionLevel_End,
	WorldChrMan_ConstructUpdateList_Begin,
	WorldChrMan_ConstructUpdateList,
	WorldChrMan_ConstructUpdateList_End,
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
	GeomModelInsCreate,
	AiBeginCollectGabage,
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
	HavokAi_SilhouetteGeneratorHelper_Begin,
	WorldChrMan_PairAnim,
	WorldChrMan_StatusData_Sync,
	WorldChrMan_PostPhysics,
	GameFlowInGame_MoveMap_PostPhysics_0,
	HavokAi_SilhouetteGeneratorHelper_End,
	DmgMan_Pre,
	DmgMan_ShapeCast,
	DmgMan_Post,
	JamaisVu_JamaisVuExecute,
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
	AimCamAssistMan_BeginAimCamAssistCalc,
	AimCamAssistMan_EndAimCamAssistCalc,
	WorldChrMan_BeginUpdateLandingPosition,
	WorldChrMan_EndUpdateLandingPosition,
	GameFlowInGame_TestNet,
	ScaleformStep,
	FlverResDelayDelectiionEnd,
	Draw_Pre,
	GraphicsStep,
	DebugDrawMemoryBar,
	DbgMenuStep,
	DbgRemoteStep,
	PlaylogSystemStep,
	ReportSystemStep,
	DbgDispStep,
	DrawStep,
	SfxInsUpdate,
	SfxUpdate,
	LodUpdate,
	DrawBegin,
	GameSceneDraw,
	AdhocDraw,
	DrawEnd,
	Draw_Post,
	SoundPlayLimitterUpdate,
	BeginShiftWorldPosition,
	FileStep,
	Flip,
	DelayDeleteStep,
	AiEndCollectGabage,
	RecordHeapStats,
	FrameEnd,
	Dummy,
}
