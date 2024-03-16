pub mod view{
    use nannou::prelude::*;
    use nannou::text::font::from_file;
    use std::collections::HashMap;
    use crate::dendrite_model::model::{Model,Nodes,Ninfo,Ainfo};
    use crate::dendrite_setting::setting::*;
    use crate::dendrite_primitive::prim::*;
    use crate::dendrite_save::save::save;

    pub fn dd_view(app: &App, model: &Model, frame: Frame) {
        if app.elapsed_frames() % FPS_60 as u64 != 0 {
            return;
        }
        
        frame.clear(WHITE);

        let mut draw = app.draw();

        let mag = model.mag;
        let anchor = model.anchor;


        if TEST{
            draw_attractors(&model.attractors, &mut draw, mag , anchor);

            draw_nodes_dot(&model.nodes.nodesmap,&mut draw, mag, anchor);

            draw_texts(app, &model, &mut draw);

            draw_attraction(&model, &mut draw, mag, anchor);

            draw_glaph(model, &mut draw);

            draw.rect()
                .w_h(WID as f32, HEI as f32)
                .no_fill()
                .stroke(GRAY)
                .stroke_weight(1.);
        }
        

        draw_nodes_branch(&model.nodes, &mut draw, mag, anchor);

        draw_fixed(model, &mut draw);

        draw.to_frame(app, &frame).unwrap();

        if SAVE{
            save(app , model);
        }

        
    }

    fn draw_attractors(attractors : &Vec<Ainfo>, draw : &mut Draw, mag : f32, anchor : Vec2) {

        for i in 0..attractors.len(){
            draw.ellipse()
                .xy(expansion_2d(attractors[i].pos, mag, anchor))
                .radius(1.)
                .color(GRAY);
        }
        
    }

    fn draw_nodes_dot(nodesmap : &HashMap<u32,Ninfo>, draw : &mut Draw, mag : f32, anchor : Vec2) {
        for (_key, ninfo) in nodesmap{
            draw.ellipse()
                .xy(expansion_2d(ninfo.pos , mag, anchor))
                .radius(3.)
                .no_fill()
                .stroke_weight(
                    match ninfo.nest_c{
                        0 =>{0.}
                        _ =>{1.}
                    }
                )
                .stroke_color(GRAY);
        }
    }

    fn draw_nodes_branch(nodes : &Nodes, draw : &mut Draw, mag : f32, anchor : Vec2) {
        for i in 0..nodes.connection.len(){
            let (key1,key2) = nodes.connection[i];

            let start_pos = (*nodes.nodesmap.get(&key1).unwrap()).pos;
            let end_pos = (*nodes.nodesmap.get(&key2).unwrap()).pos;

            let ave_gen_f32 = {
                let gen1 = (*nodes.nodesmap.get(&key1).unwrap()).generation;
                let gen2 = (*nodes.nodesmap.get(&key2).unwrap()).generation;

                (gen1 + gen2) as f32 * 0.5
            };

            draw.line()
                .start(expansion_2d(start_pos, mag, anchor))
                .end(expansion_2d(end_pos, mag, anchor))
                .weight(2. / (1.+ave_gen_f32 * 0.1))
                .color(DIMGRAY);
        }
    }

    fn draw_texts(app : &App, model : &Model, draw : &mut Draw) {

        let text_x = WID as f32 * 0.5  + (WINDOW_WID - WID)as f32 * 0.35;
        let text_y = (WINDOW_HEI) as f32 * -0.46;

        //frame num
        let now_frame = natural_number_formatter(app.elapsed_frames() as u32, 5);
        //attractor num
        let attractors_num = natural_number_formatter(model.attractors.len() as u32, 5);
        //node_num
        let nodes_num = natural_number_formatter(model.nodes.nodesmap.len() as u32, 5);
        //block
        let block = natural_number_formatter(model.time_scale.block as u32, 5);
        //generation
        let generation = natural_number_formatter(model.time_scale.generation as u32, 5);

        

        draw_each_text(draw, text_x, text_y+80., &format!("Frames : {}",now_frame));
        draw_each_text(draw, text_x, text_y+60., &format!("Attractors : {}",attractors_num));
        draw_each_text(draw, text_x, text_y+40., &format!("Nodes  : {}",nodes_num));
        draw_each_text(draw, text_x, text_y+20., &format!("Blocks : {}",block));
        draw_each_text(draw, text_x, text_y+00., &format!("Generation : {}",generation));
    } 

    fn draw_each_text(draw : &mut Draw , x : f32, y :f32, sentence : &str) {
        draw.text(sentence)
            .color(BLACK)
            .font_size(12)
            .x_y(x , y)
            .font(from_file(FONT_UI_PATH).unwrap())
            .right_justify();
    }

    fn draw_attraction(model : &Model, draw : &mut Draw, mag : f32, anchor : Vec2) {
        for attractor in &model.attractors{
            let a_pos = attractor.pos;
            match attractor.nest_k{
                Some(key) => {
                    let nest_p = model.nodes.nodesmap.get(&key).unwrap().pos;

                    draw.line()
                        .start(expansion_2d(a_pos, mag, anchor))
                        .end(expansion_2d(nest_p, mag, anchor))
                        .weight(0.2)
                        .rgba8(80, 80, 80,120);
                } 
                None => {}};
        }
    }

    fn draw_fixed(model : &Model ,draw : &mut Draw) {
        for fixed_point in &model.fixed_points{

            let start_pos = model.nodes.nodesmap.get(&0).unwrap().pos;

            if fixed_point.generation > 0 {
                let end_pos = (fixed_point.pos - start_pos).normalize() * WINDOW_WID as f32 + start_pos;

                let alpha = 255 - (fixed_point.generation * 10) as u8;

                draw.line()
                        .start(start_pos)
                        .end(end_pos)
                        .weight(1. * (0.1 + 0.9/fixed_point.generation as f32))
                        .rgba8(69, 69, 69, alpha);
            }
            
        }
    }

    fn draw_glaph(model : &Model ,draw : &mut Draw) {
        let glaph_start = (WID/2 + (WINDOW_WID - WID)/16) as f32;
        let glaph_wid = (WINDOW_WID - WID) as f32 * (0.5 - 2./16.);
        let glaph_hei = WINDOW_HEI as f32 * 0.5;
        let plot_x_distance = glaph_wid/GLAPH_DATA_NUM as f32;

        let plox_a_distance = glaph_hei/model.glaph_max.attractor_max as f32;
        let plox_n_distance = glaph_hei/model.glaph_max.node_max as f32;

        for i in 0..model.glaph_data.len()-1 {
            let _a1 = model.glaph_data[i].attractor_num as f32 * plox_a_distance;
            let _a2 = model.glaph_data[i+1].attractor_num as f32 * plox_a_distance;
            let n1 = model.glaph_data[i].node_num as f32 * plox_n_distance;
            let n2 = model.glaph_data[i+1].node_num as f32 * plox_n_distance;

            let x1 = glaph_start + i as f32 * plot_x_distance;
            let x2 = glaph_start + (i+1) as f32 * plot_x_distance;

            draw.line()
                .start(vec2(x1, n1 - glaph_hei * 0.5))
                .end(vec2(x2, n2 - glaph_hei * 0.5))
                .weight(0.7)
                .color(DIMGRAY);
            
            /* 
            draw.line()
                .start(vec2(x1, n1 - glaph_hei - 5.))
                .end(vec2(x2, n2 - glaph_hei - 5.))
                .weight(0.5)
                .color(DIMGRAY);
            */
        }
    }
}