use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::engine::{GpuParticles2D, IGpuParticles2D};
#[derive(GodotClass)]
#[class(init,base=GpuParticles2D)]
pub struct OneTimeParticle{
#[export]
color:Color,
base:Base<GpuParticles2D>
}
#[godot_api]
impl IGpuParticles2D for OneTimeParticle {
    fn ready(&mut self) {
        let callfunc=self.base_mut().callable("self_distruct");
        self.base_mut().set_lifetime(0.5);
        self.base_mut().set_one_shot(true);
        self.base_mut().set_emitting(true);
        self.base_mut().set_explosiveness_ratio(1.0);
        let clr=self.color;
        self.base_mut().set_modulate(clr);
        self.base_mut().connect("finished".into(), callfunc);
    }
}
#[godot_api]
impl OneTimeParticle {
    #[func]
    fn self_distruct(&mut self){
        self.base_mut().queue_free();
    }
}