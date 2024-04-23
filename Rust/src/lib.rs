#![feature(linked_list_remove)]
pub extern crate nanoserde;//using nanoserde from the og godot lib
use std::f32::consts::{PI, TAU};
use godot::prelude::*;
const DFPS:f32=60.0;// base frame rate
struct Myext{}//init everything
pub unsafe fn signer(a:u8)->i8{
    // for jobs sake
    union Convertor {
        ap:u8,
        bx:i8,
    }
    let  x=Convertor{
        ap:a,
    };
    x.bx
}
pub unsafe fn designer(a:i8)->u8{
    //same
union Convertor {
    ap:u8,
    bx:i8,
}
let x=Convertor{
    bx:a
};
x.ap
}
pub unsafe fn circular_hex_angle_move_towards(from:u8,to:u8,delta:u8)->u8{
//tur the values to signed values (bits are unchanged but representation and available tools are changed)
let s_from=signer(from);
let s_to=signer(to);
//get the distance
let signed_dist=s_to.wrapping_sub(s_from);
if signed_dist.abs()<signer(delta).abs(){
// if the distance is small enough then snap.
to
}
else if to.wrapping_sub(from) == 128 {
    from.wrapping_add(delta)
}
else {
    // get an angle on the direction of distance but with the value of delta
    let res =from.wrapping_add(designer(signer(delta)*signed_dist.signum()));

    res
}
}

pub fn _godot_radian_angle_to_semi_broken_hex_angle(angle:f32)->u8{
// first of all this angle system is in radian and clockwise
// angle range is from -PI to PI
// i have to convert -PI to PI into 0 to 256
//first devide angle by pi to get -1 to 1 range
// then multiply by 128 to get -128 to 128
// then add 128 to get 0 to 256 and then convert it into u8 but in this case 0 the base would be maped to -PI
// to avoid it and get a suitable angle 128 is added again but this time everything is wraped out. 
((((angle/PI)*128.0)+128.0) as u8).wrapping_add(128) 

}
pub fn _semi_broken_hex_angle_to_godot_radian_angle(angle:u8)->f32{
    // its zero to 256
    // mult by tau to get 0 to 256Tau
    // devide by 256 to get 0 to Tau aka jobs done
    ((angle as f32 )* TAU) /256.0
}

mod actortype;//actor types job is to allow cross communication between fellow types. like mob trying to chase player or fleeing 
// from bullet or bullet chasing mob etc.
mod timetracker;//stuff responsible for inputs having time data
mod specialbutton;//Dash, Shoot info enum
mod controler;//controls the player

mod player;//The player class

mod mindless_mover;// base class of bullet and mob <-out dated
                   // more like abstract logic class?
mod bullet;//
mod mob;// this is the enemy class
//mod main_scene;

#[gdextension]
unsafe impl ExtensionLibrary for Myext {}