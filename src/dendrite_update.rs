pub mod update{
    use nannou::prelude::*;
    use std::collections::HashMap;
    use crate::dendrite_model::model::*;
    use crate::dendrite_primitive::prim::{expansion_2d, vec2_in_rect_or_not};
    use crate::dendrite_setting::setting::*;

    pub fn dd_update(app: &App, model: &mut Model, _update: Update) {
        let frame = app.elapsed_frames();

        if (frame as f32/SWITCH_FRAME) as u32%2 == 0 {
            grow(model);
        }
        else if ((frame - 1) as f32/SWITCH_FRAME) as u32%2 == 0 {
            g_2_e(model);
        }
        else if ((frame + 1) as f32/SWITCH_FRAME) as u32%2 == 0 {
            e_2_g(model);
        }
        else {
            expansion(model);
        }

        update_glaph(model);
        
    }

    fn grow(model : &mut Model) {
        model.time_scale.generation += 1;
        
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

            if key == 0 {
                model.fixed_points.push(FixedLine{
                    pos : new_pos,
                    generation : 0,
                });
            }
            
            model.nodes.nodesmap.insert(model.next_key, 
                Ninfo { 
                    pos: new_pos,
                    nest_c: 0,
                    generation: model.time_scale.generation,
                }
            );

            //update connection
            model.nodes.connection.push((key,model.next_key));

            //save new node info 
            new_positions.push(new_pos);
            new_keys.push(model.next_key);

            model.next_key += 1;
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

    fn kill_attractors(model : &mut Model , new_pos : Vec2) {

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
                            nest_c : nodemap.get(&key).unwrap().nest_c - num,
                            generation : nodemap.get(&key).unwrap().generation, 
                        }
                    });
                }
                None => {}
            }
        }
    }

    fn expansion(model : &mut Model) {
        model.mag *= EXPANSION;
        model.anchor = model.nodes.nodesmap.get(&0).unwrap().pos;
    }

    fn g_2_e(model : &mut Model) {
        model.attractors = Vec::new();
    }

    fn e_2_g(model : &mut Model) {
        model.time_scale.generation = 0;
        model.time_scale.block += 1;

        model.attractors = create_attractors(DENSITY);

        //renew pos and nest_c
        expansion_node(model, model.mag, model.anchor);

        //reset mag,anchor
        model.mag = 1.;
        model.anchor = vec2(0.,0.);

        //renew connection
        update_connection(model);

        for i in 0..model.fixed_points.len() {
            model.fixed_points[i].generation += 1;
        }
    }

    fn expansion_node(model : &mut Model , mag : f32 , anchor : Vec2) {
        let mut new_nodesmap = HashMap::new();

        let mut keys = Vec::new();

        for (key ,ninfo) in &model.nodes.nodesmap {
            let pos = expansion_2d(ninfo.pos, mag, anchor);

            if vec2_in_rect_or_not(pos, WID as f32 * 1.5, HEI as f32 * 1.2) {

                new_nodesmap.insert(*key, 
                    Ninfo{
                        pos,
                        nest_c : 0,
                        generation : 0,
                    }
                );

                keys.push(*key);
            }

            
        }

        model.nodes.nodesmap = new_nodesmap;

        for key in keys{
            update_info(model, key)
        }
    }

    fn update_connection(model : &mut Model) {
        let mut new_connection = Vec::new();


        for (key1,key2) in &model.nodes.connection{
            match &model.nodes.nodesmap.get(key1) {
                Some(_) => {
                    match &model.nodes.nodesmap.get(key2) {
                        Some(_) => {
                            new_connection.push((*key1,*key2));
                        }
                        None => {}
                    }
                }
                None => {    
                }
            }
        }

        model.nodes.connection = new_connection;
    }

    fn update_glaph(model : &mut Model) {
        let attractor_num = model.attractors.len() as u32;
        let node_num = model.nodes.nodesmap.len() as u32;

        model.glaph_data.push_back({
            GlaphData {attractor_num,node_num}
        });

        if model.glaph_data.len() > GLAPH_DATA_NUM as usize {
            model.glaph_data.pop_front();
        }

        //update max
        if attractor_num > model.glaph_max.attractor_max {
            model.glaph_max.attractor_max = attractor_num;
        }

        if node_num > model.glaph_max.node_max {
            model.glaph_max.node_max = node_num;
        }
    }
}