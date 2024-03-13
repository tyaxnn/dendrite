mod dendrite_model;
mod dendrite_update;
mod dendrite_view;
mod dendrite_setting;

use dendrite_model::model::*;
use dendrite_update::update::*;
use dendrite_view::view::*;
//use setting::setting::*;

fn main(){
    nannou::app(dd_model)
        .update(dd_update)
        .view(dd_view)
        .run()
}
