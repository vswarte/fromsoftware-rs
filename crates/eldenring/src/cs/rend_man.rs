use shared::OwnedPtr;

#[repr(C)]
#[shared::singleton("RendMan")]
pub struct RendMan {
    vftable: usize,
    allocator: usize,
    stage_rend: usize,
    gx_sg_layer_flat: usize,
    unk20: usize,
    pub debug_ez_draw: OwnedPtr<CSEzDraw>,
    // TODO: rest
}

#[repr(C)]
pub struct CSEzDraw {
    vftable: usize,
    // TODO: rest
}
