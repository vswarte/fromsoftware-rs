use windows::core::PCWSTR;

#[repr(C)]
/// Source of name: RTTI
#[fromsoftware_shared::singleton("CSWindow")]
pub struct CSWindowImp {
    vftable: usize,
    pub window_handle: isize,
    pub lp_window_name: PCWSTR,
    pub lp_class_name: PCWSTR,
    pub screen_pos_x: i32,
    pub screen_pos_y: i32,
    pub screen_width: i32,
    pub screen_height: i32,
    /// hInstance from WinMain, used to interact with windows api
    /// E.g. CreateWindowExW
    pub window_module_handle: isize,
    screen_mode_ctrl: usize,
    unk40: [u8; 0x2c],
    pub runtime_window_config: CSWindowScreenConfig,
    /// represents config that will be saved to disk
    /// values from here will take priority over runtime_window_config
    pub persistent_window_config: CSWindowScreenConfig,
    unk134: [u8; 0x25F4],
}

#[repr(C)]
pub struct CSWindowScreenConfig {
    pub windowed_screen_width: i32,
    pub windowed_screen_height: i32,
    pub fullscreen_width: i32,
    pub fullscreen_height: i32,
    pub borderless_screen_width: i32,
    pub borderless_screen_height: i32,
    pub window_type: CSWindowType,
    pub auto_detect_best_setting: OnOffSetting,
    pub fps_target: FpsTarget,
    pub quality_setting: QualitySetting,
    pub texture_quality: QualityLevelSetting,
    pub antialiasing_quality: ToggleableGraphicsQuality,
    pub ssao: ToggleableGraphicsQuality,
    pub dof: ToggleableGraphicsQuality,
    pub motion_blur: ToggleableGraphicsQuality,
    pub shadow_quality: QualityLevelSetting,
    pub lighting_quality: QualityLevelSetting,
    pub effects_quality: QualityLevelSetting,
    pub reflection_quality: QualityLevelSetting,
    pub water_surface_quality: QualityLevelSetting,
    pub shader_quality: QualityLevelSetting,
    pub volumetric_quality: QualityLevelSetting,
    pub ray_tracing_quality: ToggleableGraphicsQuality,
    pub gi_quality: QualityLevelSetting,
    pub grass_quality: GrassQuality,
}

#[repr(u32)]
pub enum CSWindowType {
    Windowed = 0,
    Fullscreen = 1,
    Borderless = 2,
}

#[repr(u32)]
pub enum QualityLevelSetting {
    Low = 0,
    Medium = 1,
    High = 2,
    Maximum = 3,
}

#[repr(u32)]
pub enum ToggleableGraphicsQuality {
    Off = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Maximum = 4,
}

#[repr(u32)]
pub enum OnOffSetting {
    Off = 0,
    On = 1,
}

#[repr(u32)]
pub enum FpsTarget {
    FPS30 = 0,
    FPS60 = 1,
    FPS85 = 2,
    FPS120 = 3,
}

#[repr(u32)]
pub enum QualitySetting {
    Low = 1,
    Medium = 2,
    High = 3,
    Maximum = 4,
    Custom = 5,
}

#[repr(u32)]
pub enum GrassQuality {
    Medium = 2,
    High = 3,
    Maximum = 4,
}
