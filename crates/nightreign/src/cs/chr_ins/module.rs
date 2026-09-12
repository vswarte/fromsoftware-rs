use shared::OwnedPtr;

mod data;
mod event;
mod fall;
mod physics;
mod time_act;

pub use data::*;
pub use event::*;
pub use fall::*;
pub use physics::*;
pub use time_act::*;

#[repr(C)]
pub struct ChrInsModuleContainer {
    pub data: OwnedPtr<CSChrDataModule>,
    unk8: i64,
    unk10: i64,
    pub time_act: OwnedPtr<CSChrTimeActModule>,
    unk18: [u8; 0x38],
    pub event: OwnedPtr<CSChrEventModule>,
    unk60: i64,
    pub physics: OwnedPtr<CSChrPhysicsModule>,
    pub fall: OwnedPtr<CSChrFallModule>,
    // TODO: rest
}
