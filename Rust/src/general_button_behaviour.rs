use godot::prelude::*;//init stuff
use godot::engine::{ InputEventAction, Node};
#[derive(GodotClass)]
#[class(init,base=Node)]
// button behaviour structure
struct BB{
    #[export]
    // set type
    button_type:crate::specialbutton::SpecialButton,
    base:Base<Node>
}
#[godot_api]
impl BB {
    // press and release func
    #[func]
    fn press(&mut self){
    let mut input=InputEventAction::new_gd();
    input.set_action(match self.button_type {
        crate::specialbutton::SpecialButton::Dash=>"dash".into(),
        crate::specialbutton::SpecialButton::Shoot=>"shoot".into(),
        crate::specialbutton::SpecialButton::None=>"dash".into(),
    });
    input.set_pressed(true);
    let mut isingtn=Input::singleton();
    isingtn.parse_input_event(input.upcast());
    }
    #[func]
    fn release(&mut self){

        let mut input=InputEventAction::new_gd();
        input.set_action(match self.button_type {
            crate::specialbutton::SpecialButton::Dash=>"dash".into(),
            crate::specialbutton::SpecialButton::Shoot=>"shoot".into(),
            crate::specialbutton::SpecialButton::None=>"dash".into(),
        });
        input.set_pressed(false);
        let mut isingtn=Input::singleton();
        isingtn.parse_input_event(input.upcast());
    }
}