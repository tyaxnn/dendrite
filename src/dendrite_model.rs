pub mod model{
    use nannou::prelude::*;
    use std::collections::HashMap;
    use crate::prim::{math::random_vec2, setting::*};

    pub struct Model{
        pub nodes : Nodes,
        pub attractors : Vec<Vec2>,
    }

    pub struct Nodes{
        pub nodesmap : HashMap<u32,Vec2>,
        pub connection : Vec<(u32,u32)>,
        pub active : Vec<u32>,
    }

    pub fn dd_model(app: &App) -> Model{
        app.new_window()
            .size(WID,HEI)
            .build()
            .unwrap();
        
        Model{
            nodes : create_one_nodes(),
            attractors : create_attractors(0.02),
        }
    }

    fn create_attractors(density : f32) -> Vec<Vec2> {
        let mut attractors = Vec::new();
        let (widf,heif) = (WID as f32,HEI as f32);

        let num_attractors = (density * widf * heif) as u32;

        for _ in 0..num_attractors{
            attractors.push(   
                    random_vec2(widf, heif)
            )
        }

        attractors
    }

    #[allow(dead_code)]
    fn create_one_nodes() -> Nodes{

        let mut nodesmap = HashMap::new();

        nodesmap.insert(0, random_vec2(WID as f32, HEI as f32));

        Nodes{
            nodesmap,
            connection : Vec::new(),
            active : vec![0],
        }
    }

    #[allow(dead_code)]
    fn create_two_nodes_connected() -> Nodes{
        let mut nodesmap = HashMap::new();

        nodesmap.insert(0, random_vec2(WID as f32, HEI as f32));
        nodesmap.insert(1, random_vec2(WID as f32, HEI as f32));

        Nodes{
            nodesmap,
            connection : vec![(0,1)],
            active : vec![0,1],
        }
    }
}


