pub mod save{
	use crate::dendrite_model::model::*;
	use crate::dendrite_setting::setting::*;
	use crate::dendrite_primitive::prim::*;
	use nannou::prelude::*;

	pub fn save(app : &App, model : &Model) {

		let time_string :String  = model.runtime.format("%Y_%m_%d_%H_%M_%S").to_string(); 

		let view_frame = app.elapsed_frames() as u32/ FPS_60;
		
		let file_name = format!("./assets/outputs/{}_{}/{}_{}.png",time_string,SAVE_NAME,SAVE_NAME,natural_number_formatter(view_frame,5));
		app.main_window().capture_frame(file_name);
	}
}