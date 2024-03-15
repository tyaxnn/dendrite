pub mod prim{
    use nannou::prelude::*;

    pub fn natural_number_formatter(num : u32, length : u32) -> String {

        let mod10 = num % (10u32.pow(length));
        let num_len = mod10.to_string().chars().count();

        let zeros = {
            let mut ini = "".to_string();

            for _ in (0..{
                let dif = length as i32 - num_len as i32;
                if dif > 0 {dif as u32}
                else {0u32}
            }){
                ini = format!("0{}",ini);
            }

            ini.to_string()
        };

        format!("{}{}",zeros,mod10)


    }

    pub fn vec2_in_rect_or_not(pos : Vec2, widf : f32, heif : f32) -> bool {
        if pos.x.abs() > widf * 0.5 || pos.y.abs() > heif * 0.5 {false}
        else {true}
    }

    pub fn expansion_2d(ori : Vec2, mag : f32, anchor : Vec2) -> Vec2 {
        (ori - anchor) * mag + anchor
    }
}