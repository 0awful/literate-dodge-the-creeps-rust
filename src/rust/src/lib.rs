use godot::prelude::*;

pub mod hud;
pub mod main_scene;
pub mod mob;
pub mod player;

struct DodgeTheCreeps;

#[gdextension]
unsafe impl ExtensionLibrary for DodgeTheCreeps {}
