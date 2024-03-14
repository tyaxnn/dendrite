pub mod update{
    use nannou::prelude::*;
    use std::collections::HashMap;
    use crate::dendrite_model::model::*;
    use crate::dendrite_setting::setting::*;

    pub fn dd_update(app: &App, model: &mut Model, _update: Update) {
        let frame = app.elapsed_frames();
        grow(model);
    }

    fn grow(model : &mut Model) {
        let mut next_key = model.nodes.nodesmap.len() as u32;

        let mut new_positions = Vec::new();
        let mut new_keys = Vec::new();
        
        //calculate sum direction each nodes
        let mut which_node = HashMap::new();

        for ainfo in &model.attractors{

            match ainfo.nest_k{
                Some(key) => {
                    let old_sum_dir = which_node.entry(key).or_insert(vec2(0.,0.));
                    *old_sum_dir += {
                        let dir = ainfo.pos - model.nodes.nodesmap.get(&key).unwrap().pos;
                        dir * VEROCITY
                    };
                }
                None => {}
            }
            
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
            kill_attractors(model, new_pos);
        }

        //update info
        for new_key in new_keys{
            update_info(model, new_key);
        }        
    }

    fn kill_attractors(model : &mut Model , new_pos : Vec2){

        let attractors = &mut model.attractors;
        let nodemap = &mut model.nodes.nodesmap;

        //kill attractor in this list
        let mut kill_list = Vec::new();

        //if key is in this hashmap , we reduce associated nest_c 
        let mut decline_nest_c = HashMap::new();

        for i in 0..attractors.len(){

            let dis = attractors[i].pos - new_pos;

            if dis.length() < KILL_RANGE{
                kill_list.push(i);

                let count = decline_nest_c.entry(attractors[i].nest_k).or_insert(0);

                *count += 1u32;
            }
        }

        //kill
        kill_list.reverse();

        for index in kill_list{
            attractors.remove(index);
        }

        //reduce
        for (o_key, num) in decline_nest_c{
            match o_key {
                Some(key) => {
                    nodemap.insert(key, {
                        Ninfo{
                            pos : nodemap.get(&key).unwrap().pos,
                            nest_c : nodemap.get(&key).unwrap().nest_c - num
                        }
                    });
                }
                None => {}
            }
        }
    }

    fn expansion(model : &mut Model) {

    }
}