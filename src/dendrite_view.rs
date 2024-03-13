pub mod view{
    use nannou::prelude::*;
    use std::collections::HashMap;
    use crate::dendrite_model::model::{Model,Nodes,Ninfo,Ainfo};
    use crate::dendrite_setting::*;

    use self::setting::TEST;

    pub fn dd_view(app: &App, model: &Model, frame: Frame) {
        frame.clear(WHITE);

        let mut draw = app.draw();


        if TEST{
            view_attractors(&model.attractors, &mut draw);

            //view_nodes_dot(&model.nodes.nodesmap,&mut draw);
        }
        

        view_nodes_branch(&model.nodes, &mut draw);

        draw.to_frame(app, &frame).unwrap();
    }

    #[allow(dead_code)]
    fn view_attractors(attractors : &Vec<Ainfo>, draw : &mut Draw) {

        for i in 0..attractors.len(){
            draw.ellipse()
                .xy(attractors[i].pos)
                .radius(1.)
                .color(ORANGE);
        }
        
    }

    #[allow(dead_code)]
    fn view_nodes_dot(nodesmap : &HashMap<u32,Ninfo>, draw : &mut Draw) {

        for (_key, ninfo) in nodesmap{
            draw.ellipse()
                .xy((*ninfo).pos)
                .radius(1.)
                .color(SKYBLUE);
        }
    }

    #[allow(dead_code)]
    fn view_nodes_branch(nodes : &Nodes, draw : &mut Draw) {
        for i in 0..nodes.connection.len(){
            let (key1,key2) = nodes.connection[i];

            let start_pos = (*nodes.nodesmap.get(&key1).unwrap()).pos;
            let end_pos = (*nodes.nodesmap.get(&key2).unwrap()).pos;

            draw.line()
                .start(start_pos)
                .end(end_pos)
                .weight(1.)
                .color(BLACK);
        }
    }

    
}