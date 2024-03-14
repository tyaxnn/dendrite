pub mod prim{

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
}