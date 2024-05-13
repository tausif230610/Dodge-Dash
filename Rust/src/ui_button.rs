use godot::prelude::*;
use godot::engine::{TouchScreenButton,ITouchScreenButton,TextureProgressBar};// it is a touch button system
// all the dash and shoot button logics
#[derive(GodotClass)]
#[class(base=TouchScreenButton)]
pub struct SUIButton{
// take the 3 progress bars and fill them
#[export]
joint_progress_bar1:Option<Gd<TextureProgressBar>>,
#[export]
joint_progress_bar2:Option<Gd<TextureProgressBar>>,
#[export]
joint_progress_bar3:Option<Gd<TextureProgressBar>>,
base:Base<TouchScreenButton>
}
#[godot_api]
impl ITouchScreenButton for SUIButton {// init stuff
    fn init(base: Base<TouchScreenButton>) -> Self {
        SUIButton {
            joint_progress_bar1: None,
            joint_progress_bar2:None,
            joint_progress_bar3:None,
            base }
    }

    
    
}
#[godot_api]
impl SUIButton {
    #[func]// fill them accordingly
    pub fn on_level_change(&mut self,level:u8){
        if let (Some(mut tar1),Some(mut tar2),Some(mut tar3))=(self.joint_progress_bar1.clone(),self.joint_progress_bar2.clone(),self.joint_progress_bar3.clone()){
            tar1.set_value(level as f64);
            tar2.set_value(level as f64);
            tar3.set_value(level as f64);
        }
    }

}