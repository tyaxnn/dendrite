pub mod update{
    use nannou::prelude::*;
    use crate::dendrite_model::model::Model;

    pub fn dd_update(_app: &App, model: &mut Model, _update: Update) {
        let mut next_key = model.nodes.nodesmap.len() as u32;

        let mut new_active_keys = Vec::new();
        let mut dead = Vec::new();

        for i in 0..model.nodes.active.len(){

            let active_key = model.nodes.active[i];
            let position = *model.nodes.nodesmap.get(&active_key).unwrap();

            match cal_ave_dir(&mut model.attractors, position){
                Some(dir) => {
                    //update nodes
                    model.nodes.nodesmap.insert(next_key, position+dir);
                    model.nodes.connection.push((active_key,next_key));
                    new_active_keys.push(next_key);
                    next_key +=1;
                }
                None => {
                    dead.push(i);
                }
            }
            
        }

        //update active lists
        dead.reverse();
        for index in dead{
            model.nodes.active.remove(index);
        }

        //kill attractors
        for key in &new_active_keys{
            let pos = model.nodes.nodesmap.get(key).unwrap();

            kill_attractors(&mut model.attractors, *pos)
        }

        model.nodes.active.append(&mut new_active_keys);

        

        println!("{},{}",model.nodes.active.len(),model.attractors.len());
    }

    fn cal_ave_dir(attractors: &Vec<Vec2>, node_position : Vec2) -> Option<Vec2>{

        let mut sum_dir = vec2(0., 0.);
        let mut near_count = 0;

        for i in 0..attractors.len(){

            let dir = attractors[i] - node_position;
            if dir.length() < 100.{
                sum_dir += dir;
                near_count += 1;
            }
        }

        if near_count == 0{None}
        else {

            let ave_dir = sum_dir / (near_count as f32);

            if ave_dir.length() > 10.{
                Some(ave_dir)
            }
            else{None}
            
        
        }
            
    }

    fn kill_attractors(attractors : &mut Vec<Vec2> , new_position : Vec2){

        let mut kill_list = Vec::new();

        for i in 0..attractors.len(){

            let dis = attractors[i] - new_position;

            if dis.length() < 10.{
                kill_list.push(i);
            }
        }

        //kill
        kill_list.reverse();

        for index in kill_list{
            attractors.remove(index);
        }
    }
}