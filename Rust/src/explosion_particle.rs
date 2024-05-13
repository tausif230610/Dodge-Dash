use godot::obj::WithBaseField;
use godot::prelude::*;// init stuff class is gpuparticle2d
use godot::engine::{GpuParticles2D, IGpuParticles2D};
#[derive(GodotClass)]
#[class(init,base=GpuParticles2D)]
// basically color is settable stuff is already set and self distruct due to being one time 
pub struct OneTimeParticle{
#[export]
color:Color,
#[export]
#[init(default=None)]
explosion_sound:Option<Gd<AudioStreamPlayer>>,
base:Base<GpuParticles2D>
}
#[godot_api]
impl IGpuParticles2D for OneTimeParticle {
    // allow it to be self distructable and set the default settings and play there explosion sound
    fn ready(&mut self) {
        let callfunc=self.base_mut().callable("self_distruct");
        self.base_mut().set_lifetime(0.5);
        self.base_mut().set_one_shot(true);
        self.base_mut().set_emitting(true);
        self.base_mut().set_explosiveness_ratio(1.0);
        let clr=self.color;
        self.base_mut().set_modulate(clr);
        self.base_mut().connect("finished".into(), callfunc);
        if let Some(se)=self.explosion_sound.as_mut(){
            se.play();
        }
    }
}
#[godot_api]
impl OneTimeParticle {
    #[func]
    // self distruction part
    fn self_distruct(&mut self){
        self.base_mut().queue_free();
    }
}