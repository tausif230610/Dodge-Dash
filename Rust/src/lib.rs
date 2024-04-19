use godot::prelude::*;
const DFPS:f32=60.0;// base frame rate
struct Myext{}//init everything
mod timetracker;//stuff responsible for inputs having time data
mod specialbutton;//Dash, Shoot info enum
mod controler;//controls the player

mod player;//The player class
mod mindless_mover;// base class of bullet and mob <-out dated
                   // more like abstract logic class?
                    
//mod bullet;//
mod mob;// this is the enemy class
//mod main_scene;

#[gdextension]
unsafe impl ExtensionLibrary for Myext {}