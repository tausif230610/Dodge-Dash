use std::fmt::Debug;
//Without it Compiler gives warning
use godot::{builtin::meta::GodotConvert, prelude::*};//i literelly implemented To Godot, From Godot, Godot Convert by hand for this thing. well what could go wrong?
#[derive(PartialEq,Clone, Copy)]//pls dont break any more as you already give me a headace
//yeah you are now breaking once more nice
//A basic enum implementing all the Special buttons

pub enum SpecialButton{
    Dash,
    Shoot,
    None
}
// conversion from u8 to special button and vice versa is quite the common thing. so this fuctions help me with this stuffs.
impl SpecialButton {
    //special button to u8 aka 1 byte unsigned int (it would be far better if i just got 2 byte aka 0,1,2,3 insted of 0..255. waste of space but minimum till better method are discovered).
    //used for serilizing for godot and normal output
    fn specialbutton_to_u8(&self)->u8{
        // simple match statements
        match self {
            SpecialButton::None=>0,
            SpecialButton::Dash=>1,
            SpecialButton::Shoot=>2
        }
    }
    // for implementing the debug output
    fn specialbutton_to_string(&self)->String {
        match self {
            // also match statements
            SpecialButton::Dash=>"Dash".to_string(),
            SpecialButton::Shoot=>"Shoot".to_string(),
            SpecialButton::None=>"None".to_string(),
        }
    }
    // for deserilization
    fn u8_to_specialbutton(value:u8)->Self{
        //match statements. 1-dash, 2-shoot 3..255 U 0-None
        match value {
            1=>SpecialButton::Dash,
            2=>SpecialButton::Shoot,
            _=>SpecialButton::None,

        }
    }
}
impl ToGodot for SpecialButton {
    //serialized em and packed em to godot
    fn to_godot(&self) -> u8 {
        self.specialbutton_to_u8()
    }
}
impl FromGodot for SpecialButton {
    // get the number from godot and magically(or not) get the special number
    fn from_godot(via: u8) -> Self {
        SpecialButton::u8_to_specialbutton(via)
    }
    // same. it is just a 0..255 number begging to turn into a button via a function who can happily recive any u8 number and simply turn them into Special button 
    fn try_from_godot(via: u8) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(SpecialButton::u8_to_specialbutton(via))
    }
}
// idk what it is but i think i set the type Via = u8 and it is ok now? and Via is the type by whom godot communicate with the rust libs
impl GodotConvert for SpecialButton {
    type Via = u8;
}
// i told before that sb to u8 is nessesory enough to be its own func. guess what this is where the serializer is held responsible for printing too!
impl std::fmt::Display for SpecialButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.specialbutton_to_u8())
    }
}
// well debug needs special info. and sb to string gives the type of button being pressed. No extra info but not to little info good for both sides
impl Debug for SpecialButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.specialbutton_to_string())
    }
}
/* 
well its obsolete now
impl Default for SpecialButton {
    fn default() -> Self {
        Self::None
    }
}
*/