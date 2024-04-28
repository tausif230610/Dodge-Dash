use std::collections::LinkedList;//this is important to store the targets at view


use godot::obj::WithBaseField;

use godot:: prelude::*;//init stuff
use godot::engine::{Area2D, IArea2D}; //mindless mover is a child type of area 2d

use crate::DFPS;// need the fps
#[derive(GodotClass)]
#[class(init,base=Area2D)]
pub struct MindlessMover{
    //classic R theta style calculations
    #[var]
    pub angle:u8,
    #[var]
    pub speed:u8,
    #[export]
    #[init(default=None)]
    pub death_particle_scene:Option<Gd<PackedScene>>,
    // mindless mover is a general class so actor is unknown
    #[init(default=crate::actortype::Types::Unknown)]
    type_of_actor:crate::actortype::Types,
    #[init(default=None)]
    // targets which the mindless mover care about
    pub actual_targets:Option<LinkedList<(Gd<Area2D>,i8)>>,
    #[export]
    // the behaviour type that controls the object as a whole aka the brain if it is the body
    behaviour:Option<Gd<Node2D>>,
    #[export]
    #[init(default=5)]
    // how fast this entity can react
    angle_change_rate:u8,
    pub base:Base<Area2D>
}
#[godot_api]
impl IArea2D for MindlessMover {
    fn process(&mut self,delta:f64){
        // current radian angle
        let radangle=crate::_semi_broken_hex_angle_to_godot_radian_angle(self.angle);
        // calculated velocity
        let velocity=Vector2::new(self.speed as f32, 0.0).rotated(radangle);
        // get the current position
        let mut position=self.base_mut().get_position();
        // add the velocity to the position
        position+=velocity*delta as f32 *DFPS;
        // set position and rotation
        self.base_mut().set_position(position);
        self.base_mut().set_rotation(radangle);
        // get the targets
        let  act_trgts=self.actual_targets.clone();
        // create a vector of angles
        let mut angls:Vec<u8>=Vec::new();
        match act_trgts {
            // get the targets from the options
            Some( act_tr)=>{

            for tr in act_tr{
                // get the areas

                let trgt_ara= &tr.0;
                // get the associated values
                let vl=tr.1;
                // get the hex angel
                let mut angle_of_target=crate::_godot_radian_angle_to_semi_broken_hex_angle(self.base_mut().get_position().angle_to_point(trgt_ara.get_position()));
                // match the associations
                match vl {
                    1=>{},
                    -1=>{angle_of_target=angle_of_target.wrapping_neg();},
                    _=>{angle_of_target=self.angle;}
                    
                }
                // push em into the angle array


                angls.push(angle_of_target);
            }
            // take the avarage
            let mut sum:u64=0;
            for i in &angls{
                sum+=*i as u64;
            }
            
            sum=sum/(angls.len() as u64);
            let truesum=sum as u8;
            let ngl:u8;
            // special move towards function. it works and thats it!
            unsafe{
            ngl=crate::circular_hex_angle_move_towards(self.angle, truesum, self.angle_change_rate);
            }

            self.angle=ngl;
        }
            ,
            None=>{

            }
        }
        
}
}
#[godot_api]
impl MindlessMover {
    // entity would die. when they die. let the main scene know that some one has died.
    #[signal]
    fn death();
    #[func]
    // tell the main scene that the mob is dead.
    fn kill_command(&mut self){
        if let Some(particle_info)=self.death_particle_scene.as_ref(){
            let mut part_scene=particle_info.instantiate_as::<crate::explosion_particle::OneTimeParticle>();
            part_scene.set_position(self.base_mut().get_position());
            part_scene.set_rotation(self.base_mut().get_rotation());
            
            self.base_mut().get_parent().unwrap().add_child(part_scene.clone().upcast());
        }
        self.base_mut().emit_signal("death".into(), &[]);
        self.base_mut().queue_free();
    }
    #[func]
    //also they got out of bounds
    fn on_viewport_exit(&mut self){
        self.base_mut().queue_free();
    }
    #[func]
    // get the type
    pub fn get_type(&mut self)->GString{
        let mut p=self.type_of_actor;
        return p.godot_gstring_serilize();
    }
    // set the type
    pub fn set_type(&mut self,typ:crate::actortype::Types){
        self.type_of_actor=typ;
    }
    #[func]
    // if anything leaves then delete there reference
    fn on_area_out_of_rng(&mut self, area:Gd<Area2D>){
        let mut track:usize=0;// create a track variable
    if let Some(trgtarr)=&self.actual_targets{
        // loop till found the leaver
        for i in trgtarr {
            if i.0==area && area != self.base().clone().upcast(){
                break;
            }
            track+=1;
        }
    }
    // remove the thing at that location
    if let Some(trg)= self.actual_targets.as_mut() {
    trg.remove(track);
    }
    // if the list is empty then regester it empty
    if self.actual_targets.as_mut().unwrap().is_empty(){
        self.actual_targets=None;
    }
    }
    #[func]
    // regester em to the list
    fn on_area_colide(&mut self,mut area: Gd<Area2D>){
        // check if it isnt the entity itself coliding with itself
        if area!=self.base_mut().clone().upcast(){      
        // a type variable
        let mut type_of_area:crate::actortype::Types;
        // get the type of area
        match area.try_call("get_type".into(), &[])  {
            Ok(res)=>{
                match res.try_to::<GString>(){
                    Ok(tyinfo)=>{
                        type_of_area=crate::actortype::Types::godot_gstring_deserilize(tyinfo);
                    },
                    Err(_)=>
                    {   
                        type_of_area=crate::actortype::Types::Unknown;
                    }
                }
            },
            Err(_)=>{
                type_of_area=crate::actortype::Types::Unknown;            }
        }
        // from the type set the appropreate behaviour through insider behaviour 
        let val:i8=self.behaviour.as_mut().unwrap().call("set_behaviour".into(),&[Variant::from(type_of_area.godot_gstring_serilize())]).to::<i8>();
        // check if the list is empty
        let  nonecase=match self.actual_targets {
            Some(_)=>false,
            None=>true,
        };
        // if it is empty then regester a new linked list
        if nonecase  {
            self.actual_targets=Some(LinkedList::new());
        }
        // push the variable
        self.actual_targets.as_mut().unwrap().push_back((area,val));


    }
    }
    // devide if false else mult
    #[func]
    pub fn slow_down_handler(&mut self,speed_factor:u8,dev_or_mul:bool){
        
        if dev_or_mul{
            self.speed*=speed_factor;
        }
        else {
            self.speed/=speed_factor;
        }
    }

}