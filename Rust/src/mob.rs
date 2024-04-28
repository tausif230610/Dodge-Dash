//actually i wanted it to be its own class but sicne rust class cant have child so this is a specialized
//work around.



use godot::engine::utilities::{randi_range, randomize};// rng stuff
use godot::obj::NewAlloc;// for alocation
use godot:: prelude::*;//init stuff
use godot::engine::{AnimatedSprite2D,Node2D};// animated sprite 2d

use crate::mindless_mover::MindlessMover; // i dont need any extra interface
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MobBehaviour{
    // min speed and max speed
    #[export]
    max_speed:u8,
    #[export]
    min_speed:u8,
    #[export]
    // the actual base that would be manupulated
    actual_base:Gd<MindlessMover>,
    base:Base<Node2D>
}

#[godot_api]
impl INode2D for MobBehaviour {
// init them
fn init(base:Base<Node2D>)->Self{
Self {
    max_speed: 10,
    min_speed: 5,
    actual_base:NewAlloc::new_alloc(),
    
    base }
}
fn ready(&mut self){
// get the animator aka animated sprite 2d
let animator=&mut self.base_mut().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
// get the actual base
let base_mover=&mut self.actual_base;
// set the base as mob
base_mover.bind_mut().set_type(crate::actortype::Types::Mob);
// call randomize as a lot of RNG follows
randomize();
// get a random intiger between min speed and max soeed
let speed_of_mob=randi_range(self.min_speed as i64, self.max_speed as i64) as u8;
// set the speed to mob
base_mover.bind_mut().set_speed(speed_of_mob);
// get the mob type
let mob_types=animator.get_sprite_frames().unwrap().get_animation_names();
// set a random animation 
animator.set_animation(mob_types.get(randi_range(0, (mob_types.len() as i64) -1) as usize).into());
// play em
animator.play();
}
}
#[godot_api]
impl MobBehaviour {
    #[func]
    pub fn set_behaviour(type_of_area:GString)->i8{
        // set the mob behaviour chase player but flee dashing player and bullet and ignore everything
        let typ=crate::actortype::Types::godot_gstring_deserilize(type_of_area);
        match typ {
            crate::actortype::Types::Player(dsh)=>{
                match dsh {
                    true=>{
                        -1
                    },
                    false=>{
                        1
                    }
                }
            },
            crate::actortype::Types::Bullet=>{
                -1
            },
            _=>0,
        }
    }
}