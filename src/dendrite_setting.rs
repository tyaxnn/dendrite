pub mod setting{
    //algorithm_setting
    pub const WID : u32 = 960;
    pub const HEI : u32 = 960;
    pub const VEROCITY : f32 = 0.1;
    pub const DENSITY : f32 = 0.01;
    pub const KILL_RANGE : f32 = 30.;

    pub const EXPANSION : f32 = 1.06;
    pub const SWITCH_FRAME : u64 = 30;

    //display_setting
    pub const TEST : bool = true;

    pub const WINDOW_WID : u32 = 1440;
    pub const WINDOW_HEI : u32 = 1000;
    pub const FONT_UI_PATH : &str = "./font/Ubuntu/Ubuntu-LightItalic.ttf";
}