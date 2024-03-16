mod dendrite_model;
mod dendrite_update;
mod dendrite_view;
mod dendrite_setting;
mod dendrite_primitive;
mod dendrite_audio;

use dendrite_model::model::*;
use dendrite_update::update::*;
use dendrite_view::view::*;
//use dendrite_primitive::prim::*;
//use setting::setting::*;

use std::env;

fn main(){
    env::set_var("RUST_BACKTRACE", "0");

    nannou::app(dd_model)
        .update(dd_update)
        .view(dd_view)
        .run()
}
