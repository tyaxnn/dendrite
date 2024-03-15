pub mod setting{
    //algorithm_setting
    pub const WID : u32 = 960;
    pub const HEI : u32 = 960;
    pub const VEROCITY : f32 = 0.1;
    pub const DENSITY : f32 = 0.01;
    pub const KILL_RANGE : f32 = 10.;

    pub const EXPANSION : f32 = 1.05;
    pub const SWITCH_FRAME : u64 = 60;

    //display_setting
    pub const TEST : bool = true;

    pub const WINDOW_WID : u32 = 1920;
    pub const WINDOW_HEI : u32 = 1080;
    pub const FONT_UI_PATH : &str = "./font/Ubuntu/Ubuntu-LightItalic.ttf";
}