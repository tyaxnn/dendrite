pub mod view{
    use nannou::prelude::*;
    use std::collections::HashMap;
    use crate::dendrite_model::model::{Model,Nodes};

    pub fn dd_view(app: &App, model: &Model, frame: Frame) {
        frame.clear(SKYBLUE);

        let mut draw = app.draw();

        view_attractors(&model.attractors, &mut draw);

        view_nodes_dot(&model.nodes.nodesmap,&mut draw);

        view_nodes_branch(&model.nodes, &mut draw);

        draw.to_frame(app, &frame).unwrap();
    }

    fn view_attractors(attractors : &Vec<Vec2>, draw : &mut Draw) {

        for i in 0..attractors.len(){
            draw.ellipse()
                .xy(attractors[i])
                .radius(3.);
        }
        
    }

    fn view_nodes_dot(nodesmap : &HashMap<u32,Vec2>, draw : &mut Draw) {

        for (_key, pos) in nodesmap{
            draw.ellipse()
                .xy(*pos)
                .radius(3.)
                .color(GRAY);
        }
    }

    fn view_nodes_branch(nodes : &Nodes, draw : &mut Draw) {
        for i in 0..nodes.connection.len(){
            let (key1,key2) = nodes.connection[i];

            draw.line()
                .start(*nodes.nodesmap.get(&key1).unwrap())
                .end(*nodes.nodesmap.get(&key2).unwrap())
                .weight(3.)
                .color(GRAY);
        }
    }

    
}