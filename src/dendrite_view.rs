pub mod view{
    use nannou::prelude::*;
    use nannou::text::font::from_file;
    use std::collections::HashMap;
    use crate::dendrite_model::model::{Model,Nodes,Ninfo,Ainfo};
    use crate::dendrite_setting::setting::*;
    use crate::dendrite_primitive::prim::*;

    pub fn dd_view(app: &App, model: &Model, frame: Frame) {
        frame.clear(WHITE);

        let mut draw = app.draw();

        let mag = model.mag;
        let anchor = model.anchor;


        if TEST{
            draw_attractors(&model.attractors, &mut draw, mag , anchor);

            draw_nodes_dot(&model.nodes.nodesmap,&mut draw, mag, anchor);

            draw_text(app, &model, &mut draw);

            draw_attraction(&model, &mut draw, mag, anchor);

            draw.rect()
                .w_h(WID as f32, HEI as f32)
                .no_fill()
                .stroke(GRAY)
                .stroke_weight(1.);
        }
        

        draw_nodes_branch(&model.nodes, &mut draw, mag, anchor);

        draw_fixed(model, &mut draw);

        draw.to_frame(app, &frame).unwrap();
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

    fn draw_text(app : &App, model : &Model, draw : &mut Draw) {
        let font_size = 12;
        let text_x = (WID/2 + (WINDOW_WID - WID)/4) as f32;

        //frame num
        let now_frame = natural_number_formatter(app.elapsed_frames() as u32, 5);

        draw.text(&format!("F : {}",now_frame))
            .color(BLACK)
            .font_size(font_size)
            .x_y( text_x, 0.)
            .font(from_file(FONT_UI_PATH).unwrap());

        //attractor num
        let attractors_num = natural_number_formatter(model.attractors.len() as u32, 5);

        draw.text(&format!("A : {}",attractors_num))
            .color(BLACK)
            .font_size(font_size)
            .x_y(text_x , -20.)
            .font(from_file(FONT_UI_PATH).unwrap());

        //node_num
        let nodes_num = natural_number_formatter(model.nodes.nodesmap.len() as u32, 5);

        draw.text(&format!("N : {}",nodes_num))
            .color(BLACK)
            .font_size(font_size)
            .x_y(text_x , -40.)
            .font(from_file(FONT_UI_PATH).unwrap());
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
}