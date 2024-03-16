pub mod setting{

    //device setting
    const LIGHT : bool = true;
    const _48000 : bool = false;

    //algorithm_setting
    pub const WID : u32 = if LIGHT{480} else{960};
    pub const HEI : u32 = if LIGHT{480} else{960};
    pub const VEROCITY : f32 = 0.1;
    pub const DENSITY : f32 = if LIGHT{0.0001} else{0.001};
    pub const KILL_RANGE : f32 = 10.;

    pub const EXPANSION : f32 = 1.05;
    pub const SWITCH_FRAME : f32 = 58.87;

    //display_setting
    pub const TEST : bool = false;

    pub const WINDOW_WID : u32 = if LIGHT{960} else{1920};
    pub const WINDOW_HEI : u32 = if LIGHT{540} else{1080};
    pub const FONT_UI_PATH : &str = "./assets/font/Ubuntu/Ubuntu-LightItalic.ttf";

    pub const MUSIC_PATH : &str = if _48000{"./assets/music/demo_48000.wav"} else {"./assets/music/demo_44100.wav"};

    pub const GLAPH_DATA_NUM : u32 = 240;

    const FPS : u32 = 20;
    pub const FPS_60 : u32 = 60 / FPS;

    //save_setting
    pub const SAVE :bool = false;
    pub const SAVE_NAME : &str = "test_on_mac";
}