pub mod update{
    use nannou::prelude::*;
    use std::collections::HashMap;
    use crate::dendrite_model::model::*;
    use crate::dendrite_setting::setting::*;

    pub fn dd_update(_app: &App, model: &mut Model, _update: Update) {
        let mut next_key = model.nodes.nodesmap.len() as u32;

        let mut new_positions = Vec::new();
        let mut new_keys = Vec::new();
        
        //calculate sum direction each nodes
        let mut which_node = HashMap::new();

        for ainfo in &model.attractors{
            let old_sum_dir = which_node.entry(ainfo.nest_k).or_insert(vec2(0.,0.));
            *old_sum_dir += {
                let dir = ainfo.pos - model.nodes.nodesmap.get(&ainfo.nest_k).unwrap().pos;
                dir * VEROCITY
            };
        }

        //create new node
        for (key , sum_dir) in which_node{

            let parent_ninfo = model.nodes.nodesmap.get(&key).unwrap();

            //update Nodesmap

            let new_pos = {
                let ave_dir = sum_dir / parent_ninfo.nest_c as f32;
                parent_ninfo.pos + ave_dir
            };
            
            model.nodes.nodesmap.insert(next_key, 
                Ninfo { 
                    pos: new_pos,
                    nest_c: 0 
                }
            );

            //update connection
            model.nodes.connection.push((key,next_key));

            //save new node info 
            new_positions.push(new_pos);
            new_keys.push(next_key);

            next_key += 1;
        }

        //kill attractors
        for new_pos in new_positions{
            kill_attractors(&mut model.attractors, new_pos);
        }

        //update info
        for new_key in new_keys{
            update_info(model, new_key);
        }
    }

    

    fn kill_attractors(attractors : &mut Vec<Ainfo> , new_pos : Vec2){

        let mut kill_list = Vec::new();

        for i in 0..attractors.len(){

            let dis = attractors[i].pos - new_pos;

            if dis.length() < KILL_RANGE{
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