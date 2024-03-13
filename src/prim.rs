pub mod setting{
    pub const WID : u32 = 720;
    pub const HEI : u32 = 720;
}

pub mod math{
    use nannou::prelude::*;
    

    pub fn random_vec2(widf : f32, heif : f32) -> Vec2{

        let x = random_range(-widf * 0.5, widf * 0.5);
        let y = random_range(-heif * 0.5, heif * 0.5);

        vec2(x, y)
    }
}