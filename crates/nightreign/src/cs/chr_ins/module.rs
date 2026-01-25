use shared::OwnedPtr;

mod data;
mod fall;
mod time_act;
mod event;
mod physics;

pub use data::*;
pub use fall::*;
pub use time_act::*;
pub use event::*;
pub use physics::*;

#[repr(C)]
pub struct ChrInsModuleContainer {
    pub data: OwnedPtr<CSChrDataModule>,
    unk8: i64,
    unk10: i64,
    pub time_act: OwnedPtr<CSChrTimeActModule>,
    unk18: [u8; 0x38],
    pub event: OwnedPtr<CSChrEventModule>,
    unk60: usize,
    pub physics: OwnedPtr<CSChrPhysicsModule>,
    pub fall: OwnedPtr<CSChrFallModule>,
    // TODO: rest
}
