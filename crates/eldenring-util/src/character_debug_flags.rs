use std::mem::transmute;
use std::sync::{LazyLock, RwLock};

use crate::program::Program;
use eldenring::cs::WorldChrManDbgFlags;
use pelite::pattern;
use pelite::pattern::Atom;
use pelite::pe64::Pe;

const CHARACTER_DEBUG_FLAGS_PATTERN: &[Atom] = pattern!(
    "
    ba b4 00 00 00
    48 8d 0d ? ? ? ?
    e8 ? ? ? ?
    80 3d $ { ' } 00
    0f 85 ? ? ? ?
    32 c0
    48 83 c4 20
    "
);

pub static CHARACTER_DEBUG_FLAGS: LazyLock<RwLock<&mut WorldChrManDbgFlags>> =
    LazyLock::new(|| {
        let program = Program::current();

        let mut matches = [0; 2];

        if !program
            .scanner()
            .finds_code(CHARACTER_DEBUG_FLAGS_PATTERN, &mut matches)
        {
            panic!("Failed to find character debug flags pattern");
        }

        tracing::debug!("Found character properties pattern");
        RwLock::new(unsafe {
            transmute::<u64, &mut WorldChrManDbgFlags>(program.rva_to_va(matches[1] + 1).unwrap())
        })
    });
