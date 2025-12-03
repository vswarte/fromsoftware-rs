use vtable_rs::VPtr;

use shared::{Subclass, Superclass};

pub static TEAM_TYPE_RIVAL: CSTeamTypeRival = CSTeamTypeRival {
    base: CSTeamTypeBase {
        vftable: VPtr::new(),
    },
};

pub static TEAM_TYPE_ENEMY: CSTeamTypeEnemy = CSTeamTypeEnemy {
    base: CSTeamTypeBase {
        vftable: VPtr::new(),
    },
};

pub static TEAM_TYPE_FRIEND: CSTeamTypeFriend = CSTeamTypeFriend {
    base: CSTeamTypeBase {
        vftable: VPtr::new(),
    },
};

#[repr(C)]
#[derive(Superclass)]
#[superclass(children(CSTeamTypeNeutral, CSTeamTypeFriend, CSTeamTypeEnemy, CSTeamTypeRival))]
pub struct CSTeamTypeBase {
    vftable: VPtr<dyn CSTeamTypeVmt, Self>,
}

#[vtable_rs::vtable]
pub trait CSTeamTypeVmt {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool;
}

impl CSTeamTypeVmt for CSTeamTypeBase {
    extern "C" fn validate(
        &self,
        _team_relation: &TeamRelationTargetInfo,
        _self_target: bool,
    ) -> bool {
        unimplemented!("CSTeamTypeBase should not be used directly");
    }
}

#[repr(C)]
#[derive(Subclass)]
pub struct CSTeamTypeNeutral {
    pub base: CSTeamTypeBase,
}

impl CSTeamTypeVmt for CSTeamTypeNeutral {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool {
        if self_target {
            return team_relation.self_target;
        }
        false
    }
}

#[repr(C)]
#[derive(Subclass)]
pub struct CSTeamTypeFriend {
    pub base: CSTeamTypeBase,
}

impl CSTeamTypeVmt for CSTeamTypeFriend {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool {
        if self_target {
            return team_relation.self_target;
        }
        team_relation.friendly_target
    }
}

#[repr(C)]
#[derive(Subclass)]
pub struct CSTeamTypeEnemy {
    pub base: CSTeamTypeBase,
}

impl CSTeamTypeVmt for CSTeamTypeEnemy {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool {
        if self_target {
            return team_relation.self_target;
        }
        team_relation.oppose_target
    }
}

#[repr(C)]
#[derive(Subclass)]
pub struct CSTeamTypeRival {
    pub base: CSTeamTypeBase,
}

impl CSTeamTypeVmt for CSTeamTypeRival {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool {
        if self_target {
            return team_relation.self_target;
        }
        if !team_relation.oppose_target && !team_relation.friendly_target {
            return false;
        }
        true
    }
}

#[repr(C)]
pub struct TeamRelationTargetInfo {
    pub oppose_target: bool,
    pub friendly_target: bool,
    pub self_target: bool,
}
