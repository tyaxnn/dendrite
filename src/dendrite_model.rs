pub mod model{
    use nannou::prelude::*;
    use std::collections::HashMap;
    use crate::dendrite_setting::setting::*;

    pub struct Model{
        pub nodes : Nodes,
        pub attractors : Vec<Ainfo>,
    }

    pub struct Nodes{
        pub nodesmap : HashMap<u32,Ninfo>,
        //for draw branch
        pub connection : Vec<(u32,u32)>,
    }

    //Node information
    pub struct Ninfo{
        //the positon of node.
        pub pos : Vec2,
        //how many attractor see this key as nearest one.
        pub nest_c : u32,
    }

    //Attractor information
    pub struct Ainfo{
        //the positon of attractor.
        pub pos : Vec2,
        //the nearest node's key.
        pub nest_k : u32,
        //the distance between nearest node and this attractor
        pub nest_l : f32,
    }

    pub fn dd_model(app: &App) -> Model{
        //create window
        app.new_window()
            .size(WID,HEI)
            .build()
            .unwrap();

        let attractors = create_attractors(DENSITY);
        
        let mut model = Model{
            nodes : create_one_nodes(attractors.len() as u32),
            attractors,
        };

        //initialize information
        update_info(&mut model, 0);

        model
    }

    fn create_attractors(density : f32) -> Vec<Ainfo> {
        let mut attractors = Vec::new();
        let (widf,heif) = (WID as f32,HEI as f32);

        let num_attractors = (density * widf * heif) as u32;

        for _ in 0..num_attractors{
            attractors.push( 
                Ainfo{
                    pos : random_vec2(widf, heif),
                    nest_k : 0,
                    nest_l : f32::MAX,
                }
                    
            )
        }

        attractors
    }

    #[allow(dead_code)]
    //create first node randamly
    fn create_one_nodes(num_attractor : u32) -> Nodes{

        let mut nodesmap = HashMap::new();
        
        nodesmap.insert(0,
            Ninfo{
                pos : random_vec2(WID as f32, HEI as f32),
                nest_c : num_attractor,
            }
        );

        Nodes{
            nodesmap,
            connection : Vec::new(),
        }
    }

    //when add new_key in hashmap, this function updates Ainfo and Ninfo
    pub fn update_info(model : &mut Model , new_key : u32){

        let new_pos = (*model.nodes.nodesmap.get(&new_key).unwrap()).pos;

        for i in 0..model.attractors.len(){

            let new_dis = model.attractors[i].pos.distance(new_pos);
            let old_nest_k = model.attractors[i].nest_k;

            if model.attractors[i].nest_l > new_dis{
                //update Ainfo
                model.attractors[i].nest_k = new_key;
                model.attractors[i].nest_l = new_dis;
                
                //nest_c(new_key ++)
                model.nodes.nodesmap.insert(new_key,
                    Ninfo{
                        pos : model.nodes.nodesmap.get(&new_key).unwrap().pos,
                        nest_c : model.nodes.nodesmap.get(&new_key).unwrap().nest_c + 1
                    }
                );

                //nest_c(old_key --)
                model.nodes.nodesmap.insert(old_nest_k,
                    Ninfo{
                        pos : model.nodes.nodesmap.get(&old_nest_k).unwrap().pos,
                        nest_c : model.nodes.nodesmap.get(&old_nest_k).unwrap().nest_c - 1
                    }
                );
            }
        }
    }

    fn random_vec2(widf : f32, heif : f32) -> Vec2{

        let x = random_range(-widf * 0.5, widf * 0.5);
        let y = random_range(-heif * 0.5, heif * 0.5);

        vec2(x, y)
    }
}


