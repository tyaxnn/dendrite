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


        if TEST{
            draw_attractors(&model.attractors, &mut draw, mag);

            draw_nodes_dot(&model.nodes.nodesmap,&mut draw, mag);

            draw_text(app, &model, &mut draw);

            draw_attraction(&model, &mut draw, mag);

            draw.rect()
                .w_h(WID as f32, HEI as f32)
                .no_fill()
                .stroke(GRAY)
                .stroke_weight(1.);
        }
        

        draw_nodes_branch(&model.nodes, &mut draw, mag);

        draw.to_frame(app, &frame).unwrap();
    }

    fn draw_attractors(attractors : &Vec<Ainfo>, draw : &mut Draw, mag : f32) {

        for i in 0..attractors.len(){
            draw.ellipse()
                .xy(attractors[i].pos * mag)
                .radius(1.)
                .color(GRAY);
        }
        
    }

    fn draw_nodes_dot(nodesmap : &HashMap<u32,Ninfo>, draw : &mut Draw, mag : f32) {
        for (_key, ninfo) in nodesmap{
            draw.ellipse()
                .xy((*ninfo).pos * mag)
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

    fn draw_nodes_branch(nodes : &Nodes, draw : &mut Draw, mag : f32) {
        for i in 0..nodes.connection.len(){
            let (key1,key2) = nodes.connection[i];

            let start_pos = (*nodes.nodesmap.get(&key1).unwrap()).pos;
            let end_pos = (*nodes.nodesmap.get(&key2).unwrap()).pos;

            draw.line()
                .start(start_pos * mag)
                .end(end_pos * mag)
                .weight(1.)
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

    fn draw_attraction(model : &Model, draw : &mut Draw, mag : f32) {
        for attractor in &model.attractors{
            let a_pos = attractor.pos;
            match attractor.nest_k{
                Some(key) => {
                    let nest_p = model.nodes.nodesmap.get(&key).unwrap().pos;

                    draw.line()
                        .start(a_pos * mag)
                        .end(nest_p * mag)
                        .weight(0.2)
                        .rgba8(80, 80, 80,120);
                } 
                None => {}};
        }
    }
}