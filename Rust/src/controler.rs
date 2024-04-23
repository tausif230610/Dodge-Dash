use godot::engine::global::MouseButtonMask;
use godot:: prelude::*;//basic init stuff
use godot:: engine::{INode, InputEvent, Node};//it would be a child of Node like time tracker
use crate::specialbutton:: SpecialButton;//Need special button to handle stuff like dash and shood
use crate::timetracker::TimeTracker;//Need time tracker so that controler could connect to its child node time tracker
#[derive(GodotClass)]//init stuff
#[class(init,base=Node)]
pub struct Controler{
    input_direction:Vector2,//controler handles the input direction. although it could be an angle the case where no directional input is given and all input cancel eachother should be considered
    #[init(default=SpecialButton::None)]//it only tracks the button currently being held which would be always on the start None right?
    btnh:SpecialButton,
    #[init(default=false)]//Is dashing and is shooting  is there after i found out that the controler stil queue dash and shoot while holding. basically in this release based system this things allow button to be held
    is_dashing:bool,
    #[init(default=false)]
    is_shooting:bool,
    keyboard_position:Vector2,
    mouse_position:Vector2,
    base:Base<Node>
}
#[godot_api]
impl INode for Controler{
    // a mouse based control is also important
    fn input(&mut self,event:Gd<InputEvent>){
        //check if it is a mouse event
        if event.is_class("InputEventMouse".into()){
            //check if it is the left button being held
            if event.get("button_mask".into()).to::<MouseButtonMask>()==MouseButtonMask::LEFT{
            // get the mouse position
            let pos=event.get("position".into()).to::<Vector2>();
            // get the position of player as player is always controlers parnet
            let parent_pos=self.base_mut().get_parent().unwrap().get("position".into()).to::<Vector2>();
            // a bit of math
            // parentpos+pos=mousepos
            //pos=mousepos-parentpos
            let mut truepos=pos-parent_pos;
            godot_print!("{}",truepos);
            //check if the mouse and the player is too close.
            if truepos.length()<1.0{
                truepos=Vector2::ZERO;
            }
            
            //set the stuff
            self.mouse_position=truepos.normalized();
    
            }
            // set zero for everything
            else{
                self.mouse_position=Vector2::ZERO;
            }
        }
        else {
            self.mouse_position=Vector2::ZERO;
        }
    }

    fn process(&mut self,_delta:f64){
        let keyboard_input_handler=Input::singleton();//first i get the keyboard input handler. Note: it also works with controlers 
        
        let direcion=Vector2::new(keyboard_input_handler.get_action_strength("ui_right".into())-keyboard_input_handler.get_action_strength("ui_left".into()),keyboard_input_handler.get_action_strength("ui_down".into())-keyboard_input_handler.get_action_strength("ui_up".into())).normalized();//this is classic. no explanation needed :-] how ever i did normalize the vector at the end.
        self.keyboard_position=direcion;//set the input direction for the player to use.
        //Now this is where the fun begins.
        //this if and elif statements basically just ensures that Dash and Shoot buttons are truly held able. 
        //this is a bit hacky way but well it has to end this way i guess.
        //there should be a better way though :<
        self.input_direction=(self.keyboard_position+self.mouse_position).normalized();
        if keyboard_input_handler.is_action_pressed("Dash".into()) && !self.is_dashing{
            self.special_input_handler(SpecialButton::Dash);
            self.is_dashing=true;
        }
        else if keyboard_input_handler.is_action_pressed("Shoot".into()) && !self.is_shooting {
            self.special_input_handler(SpecialButton::Shoot);
            self.is_shooting=true;
        }
        if keyboard_input_handler.is_action_just_released("Dash".into()){
            self.special_input_handler(SpecialButton::None);
            self.is_dashing=false;
        }
        else if keyboard_input_handler.is_action_just_released("Shoot".into()) {
            self.special_input_handler(SpecialButton::None);
            self.is_shooting=false;
        }
    }
}
#[godot_api]
impl Controler {
    // this is the thing that pass the command(special) to the player. then the player does stuff with it.
    fn special_input_handler(&mut self,btn:SpecialButton){
        //get a mutable reference to the time tracker child(so that i can do what ever i want. and this is an indie project so I am the boss).
        let timetrack= &mut self.base_mut().get_node_as::<TimeTracker>("TimeTracker");
        // if the button on held is None aka nothing is being held then set the button given to the hold position and start the time tracker. 
        // since the only place this function is called is the process and the previous bracings make sure that this code wont get none from process so 
        // extra checks are unnessosary.
        if self.btnh==SpecialButton::None{
            self.btnh=btn;
            timetrack.bind_mut().start_count();
        }
        // this is a rare case i guess but can happen. if for some reason godot thought that the button held isnt released aka release code didnt run then
        // the input would constantly get charged. so in this case if one press the same input just release the charge.
        else if self.btnh==btn {
            self.btnh=SpecialButton::None;
            let time:real=timetrack.bind_mut().finalize_elapsed_time_as_real_sec();
            self.base_mut().emit_signal("queue_input".into(), &[Variant::from(btn),Variant::from(time)]);
        }
        // we can always assume that if both of them are failed then ether it is a different command or No command is passed. both 
        // of the case is handled here
        else {
            let btnr=self.btnh;
            self.btnh=btn;
            let time:real=timetrack.bind_mut().finalize_elapsed_time_as_real_sec();
            self.base_mut().emit_signal("queue_input".into(), &[Variant::from(btnr),Variant::from(time)]);
            timetrack.bind_mut().start_count();
        }
    }
    #[signal]
    fn queue_input(btn:SpecialButton,held_time:real);// the signal by which all command is passed to the player 
    #[func]
    // well the player need to get the direction from the controler. so this convenient get method. No Varient BS( its sad that i who dont use bad language use one. but this is how it is)
    pub fn get_direction(&self)->Vector2{
        self.input_direction
    }
}
