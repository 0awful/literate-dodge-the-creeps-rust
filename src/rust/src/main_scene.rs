use godot::engine::{AudioStreamPlayer2D, Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

use crate::hud::Hud;
use crate::mob::Mob;
use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainScene {
    pub score: real,
    pub mob_scene: Gd<PackedScene>,
    #[base]
    pub base: Base<Node>,
}

#[godot_api]
impl MainScene {
    #[func]
    pub fn game_over(&mut self) {
        let mut score_timer = self.base.get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base.get_node_as::<Timer>("MobTimer");
        score_timer.stop();
        mob_timer.stop();
    
        self.base
            .get_tree()
            .unwrap()
            .call_group("mobs".into(), "queue_free".into(), &[]);
    
        let mut hud = self.base.get_node_as::<Hud>("HUD");
        hud.bind_mut().show_game_over();
    
        let mut music_player = self.base.get_node_as::<AudioStreamPlayer2D>("Music");
        music_player.stop();
        let mut death_sound = self.base.get_node_as::<AudioStreamPlayer2D>("DeathSound");
        death_sound.play();
    }

    #[func]
    pub fn new_game(&mut self) {
        self.score = 0.0;
        let mut player = self.base.get_node_as::<Player>("Player");
        let marker = self.base.get_node_as::<Marker2D>("StartPosition");
        let mut player = player.bind_mut();
        player.start(marker.get_global_position());
    
        let mut start_timer = self.base.get_node_as::<Timer>("StartTimer");
        start_timer.start();
    
        let mut hud = self.base.get_node_as::<Hud>("HUD");
        let mut hud = hud.bind_mut();
        hud.update_score(self.score);
        hud.show_message_text("Get Ready".into());
    
        let mut music_player = self.base.get_node_as::<AudioStreamPlayer2D>("Music");
        music_player.play();
    }

    #[func]
    pub fn on_score_timer_timeout(&mut self) {
        self.score += 1.0;
    
        let mut hud = self.base.get_node_as::<Hud>("HUD");
        hud.bind_mut().update_score(self.score);
    }

    #[func]
    pub fn on_start_timer_timeout(&mut self) {
        let mut score_timer = self.base.get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base.get_node_as::<Timer>("MobTimer");
        score_timer.start();
        mob_timer.start();
    }

    #[func]
    pub fn on_mob_timer_timeout(&mut self) {
        let mut mob_scene = self.mob_scene.instantiate_as::<RigidBody2D>();
        
        let mut mob_spawn_location = self
            .base
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");
        
        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);
        
        mob_spawn_location.set_progress(progress as f32);
        mob_scene.set_position(mob_spawn_location.get_position());
        
        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);
        
        mob_scene.set_rotation(direction);
        
        self.base.add_child(mob_scene.clone().upcast());
        
        let mut mob = mob_scene.cast::<Mob>();
        let range = {
            // Local scope bind. Explain the rust
            let mob = mob.bind();
            rng.gen_range(mob.min_speed..mob.max_speed)
        };
        
        mob.set_linear_velocity(Vector2::new(range, 0.0));
        let lin_vel = mob.get_linear_velocity().rotated(real::from_f32(direction));
        mob.set_linear_velocity(lin_vel);
    }
}

#[godot_api]
impl INode for MainScene {
    fn init(base: Base<Node>) -> Self {
        MainScene {
            base,
            score: 0.0,
            mob_scene: PackedScene::new(),
        }
    }

    fn ready(&mut self) {
        self.mob_scene = load("res://mob.tscn");
    }
}