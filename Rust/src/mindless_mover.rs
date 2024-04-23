use std::collections::LinkedList;//this is important to store the targets at view


use godot::obj::WithBaseField;

use godot:: prelude::*;//init stuff
use godot::engine::{Area2D, IArea2D}; //mindless mover is a child type of area 2d

use crate::DFPS;// need the fps
#[derive(GodotClass)]
#[class(init,base=Area2D)]
pub struct MindlessMover{
    #[var]
    pub angle:u8,
    #[var]
    pub speed:u8,

    #[init(default=crate::actortype::Types::Unknown)]
    type_of_actor:crate::actortype::Types,
    #[init(default=None)]
    pub actual_targets:Option<LinkedList<(Gd<Area2D>,i8)>>,
    #[export]
    behaviour:Option<Gd<Node2D>>,
    #[export]
    #[init(default=5)]
    angle_change_rate:u8,
    pub base:Base<Area2D>
}
#[godot_api]
impl IArea2D for MindlessMover {
    fn process(&mut self,delta:f64){

        let radangle=crate::_semi_broken_hex_angle_to_godot_radian_angle(self.angle);
        let velocity=Vector2::new(self.speed as f32, 0.0).rotated(radangle);
        let mut position=self.base_mut().get_position();
        position+=velocity*delta as f32 *DFPS;
        self.base_mut().set_position(position);
        self.base_mut().set_rotation(radangle);
        let  act_trgts=self.actual_targets.clone();
        let mut angls:Vec<u8>=Vec::new();
        match act_trgts {
            Some( act_tr)=>{

            for tr in act_tr{

                let trgt_ara= &tr.0;
                
                let vl=tr.1;
                let mut angle_of_target=crate::_godot_radian_angle_to_semi_broken_hex_angle(self.base_mut().get_position().angle_to_point(trgt_ara.get_position()));
                
                match vl {
                    1=>{},
                    -1=>{angle_of_target=angle_of_target.wrapping_neg();},
                    _=>{angle_of_target=self.angle;}
                    
                }
                


                angls.push(angle_of_target);
            }
            let mut sum:u64=0;
            for i in &angls{
                sum+=*i as u64;
            }
            
            sum=sum/(angls.len() as u64);
            let truesum=sum as u8;
            let ngl:u8;
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
    #[signal]
    fn death();
    #[func]
    fn kill_command(&mut self){
        self.base_mut().queue_free();
    }
    #[func]
    fn on_viewport_exit(&mut self){
        self.base_mut().queue_free();
    }
    #[func]
    pub fn get_type(&mut self)->GString{
        let mut p=self.type_of_actor;
        return p.godot_gstring_serilize();
    }
    pub fn set_type(&mut self,typ:crate::actortype::Types){
        self.type_of_actor=typ;
    }
    #[func]
    fn on_area_out_of_rng(&mut self, area:Gd<Area2D>){
        let mut track:usize=0;
    if let Some(trgtarr)=&self.actual_targets{
        for i in trgtarr {
            if i.0==area && area != self.base().clone().upcast(){
                break;
            }
            track+=1;
        }
    }
    if let Some(trg)= self.actual_targets.as_mut() {
    trg.remove(track);
    }
    if self.actual_targets.as_mut().unwrap().is_empty(){
        self.actual_targets=None;
    }
    }
    #[func]
    fn on_area_colide(&mut self,mut area: Gd<Area2D>){
        if area!=self.base_mut().clone().upcast(){        
        let mut type_of_area:crate::actortype::Types;

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

        let val:i8=self.behaviour.as_mut().unwrap().call("set_behaviour".into(),&[Variant::from(type_of_area.godot_gstring_serilize())]).to::<i8>();
        let  nonecase=match self.actual_targets {
            Some(_)=>false,
            None=>true,
        };
        if nonecase  {
            self.actual_targets=Some(LinkedList::new());
        }
        self.actual_targets.as_mut().unwrap().push_back((area,val));


    }
    }

    

}