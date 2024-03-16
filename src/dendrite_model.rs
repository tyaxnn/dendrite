pub mod model{
    use nannou::prelude::*;
    use std::collections::{HashMap, VecDeque};
    use crate::dendrite_setting::setting::*;
    use crate::dendrite_audio::audio::*;

    pub struct Model{
        pub nodes : Nodes,
        pub attractors : Vec<Ainfo>,
        pub mag : f32,
        pub anchor : Vec2,
        pub next_key : u32,
        pub fixed_points : Vec<FixedLine>,
        pub time_scale : TimeScale,
        pub glaph_data : VecDeque<GlaphData>,
        pub glaph_max : GlaphMax,
        pub stream : StreamAudio,
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
        //node generation, this factor determine branch stroke width
        pub generation : u64,
    }

    //Attractor information
    pub struct Ainfo{
        //the positon of attractor.
        pub pos : Vec2,
        //the nearest node's key.
        pub nest_k : Option<u32>,
        //the distance between nearest node and this attractor
        pub nest_l : f32,
    }

    pub struct FixedLine {
        pub pos : Vec2,
        pub generation : u32,
    }

    pub struct TimeScale {
        pub block : u64,
        pub generation : u64,
    }

    pub struct GlaphData {
        pub attractor_num : u32,
        pub node_num : u32,
    }

    pub struct GlaphMax {
        pub attractor_max : u32,
        pub node_max : u32,
    }

    pub fn dd_model(app: &App) -> Model{
        //create window
        app.new_window()
            .size(if TEST{WINDOW_WID}else{WID},if TEST{WINDOW_HEI}else{HEI})
            .build()
            .unwrap();
        
        let mut model = Model{
            nodes : create_one_nodes(),
            attractors : create_attractors(DENSITY),
            mag : 1.,
            anchor : vec2(0.,0.),
            next_key : 1,
            fixed_points : Vec::new(),
            time_scale : TimeScale { block : 0, generation : 0},
            glaph_data :  VecDeque::new(),
            glaph_max : GlaphMax { attractor_max : 0, node_max : 0},
            stream : audio_set(),
        };

        //initialize information
        update_info(&mut model, 0);

        model
    }

    pub fn create_attractors(density : f32) -> Vec<Ainfo> {
        let mut attractors = Vec::new();
        let (widf,heif) = (WID as f32,HEI as f32);

        let num_attractors = (density * widf * heif) as u32;

        for _ in 0..num_attractors{
            attractors.push( 
                Ainfo{
                    pos : random_vec2(widf, heif),
                    nest_k : None,
                    nest_l : f32::MAX,
                }
                    
            )
        }

        attractors
    }

    #[allow(dead_code)]
    //create first node randamly
    fn create_one_nodes() -> Nodes{

        let mut nodesmap = HashMap::new();
        
        nodesmap.insert(0,
            Ninfo{
                pos : vec2(0.,0.),
                nest_c : 0,
                generation : 0,
            }
        );

        Nodes{
            nodesmap,
            connection : Vec::new(),
        }
    }

    //when add new_key to hashmap, this function updates Ainfo and Ninfo
    pub fn update_info(model : &mut Model , new_key : u32){

        let new_pos = (*model.nodes.nodesmap.get(&new_key).unwrap()).pos;

        for i in 0..model.attractors.len(){

            let new_dis = model.attractors[i].pos.distance(new_pos);
            let old_nest_k = model.attractors[i].nest_k;

            if model.attractors[i].nest_l > new_dis{
                //update Ainfo
                model.attractors[i].nest_k = Some(new_key);
                model.attractors[i].nest_l = new_dis;
                
                //nest_c(new_key ++)
                model.nodes.nodesmap.insert(new_key,
                    Ninfo{
                        pos : model.nodes.nodesmap.get(&new_key).unwrap().pos,
                        nest_c : model.nodes.nodesmap.get(&new_key).unwrap().nest_c + 1,
                        generation : model.nodes.nodesmap.get(&new_key).unwrap().generation
                    }
                );

                match old_nest_k{
                    Some(key) => {
                        model.nodes.nodesmap.insert(key,
                            Ninfo{
                                pos : model.nodes.nodesmap.get(&key).unwrap().pos,
                                nest_c : model.nodes.nodesmap.get(&key).unwrap().nest_c - 1,
                                generation : model.nodes.nodesmap.get(&new_key).unwrap().generation
                            }
                        );
                    }
                    None => {}
                }
                //nest_c(old_key --)
                
            }
        }
    }

    fn random_vec2(widf : f32, heif : f32) -> Vec2{

        let x = random_range(-widf * 0.5, widf * 0.5);
        let y = random_range(-heif * 0.5, heif * 0.5);

        vec2(x, y)
    }
}


