use godot::engine::{Button, CanvasLayer, ICanvasLayer, Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    #[base]
    pub base: Base<CanvasLayer>,
}

#[godot_api]
impl Hud {
    #[signal]
    fn start_game();

    #[func]
    pub fn show_message_text(&mut self, text: GString) {
        let mut message_label = self.base.get_node_as::<Label>("Message");
        message_label.set_text(text);
        message_label.show();
        let mut message_timer = self.base.get_node_as::<Timer>("MessageTimer");
        message_timer.start()
    }

    pub fn show_game_over(&mut self) {
        self.show_message_text("Game Over".into());

        let mut timer = self.base.get_tree().unwrap().create_timer(2.0).unwrap();
        timer.connect("timeout".into(), self.base.callable("_show_start_button"));
    }

    #[func]
    fn _show_start_button(&mut self) {
        let mut message_label = self.base.get_node_as::<Label>("Message");
        message_label.set_text("Dodge The Creeps".into());
        message_label.show();
        let mut button = self.base.get_node_as::<Button>("StartButton");
        button.show();
    }

    #[func]
    pub fn update_score(&mut self, score: f32) {
        let mut score_label = self.base.get_node_as::<Label>("ScoreLabel");
        score_label.set_text(score.to_string().into());
    }

    #[func]
    fn on_start_button_pressed(&mut self) {
        let mut start_button = self.base.get_node_as::<Button>("StartButton");
        start_button.hide();
        self.base.emit_signal("start_game".into(), &[]);
    }

    #[func]
    fn on_message_timer_timeout(&mut self) {
        let mut message_label = self.base.get_node_as::<Label>("Message");
        message_label.hide();
    }
}

#[godot_api]
impl ICanvasLayer for Hud {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }
}
