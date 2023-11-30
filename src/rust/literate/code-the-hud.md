# The Hud

We take a brief divergence from the [GDScript Tutorial]() here. It won't make it harder to complete.

The hud is much of what you've already encountered however it has a greater emphasis on the `impl`. Our hud will expose functions for our main scene to use. We do that by having public functions on our struct. That being said the shape of the file is very much the same. Give it a go.

###### file:../src/hud.rs
```
<<imports>>

<<struct>>

<<struct impl>>

<<engine impl>>
```

###### engine impl
```rust
#[godot_api]
impl ICanvasLayer for Hud {
    <<init>>
}
```

###### struct impl
```rust
#[godot_api]
impl Hud {
    <<signal>>
    
    <<show message text>>
    
    <<show game over>>
    
    <<update score>>
    
    <<on start button pressed>>
    
    <<on message timer timeout>>
}
```

Give it a shot. You can handle a lot of these already.

![cat]()

# Implementation
Lets go through the easy stuff real quick.

###### imports
```rust
use godot::engine::{Button, CanvasLayer, ICanvasLayer, Label, Timer};
use godot::prelude::*;
```

###### struct
```rust
#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    #[base]
    pub base: Base<CanvasLayer>,
}
```

###### signal
```rust
#[signal]
fn start_game();
```

###### init
```rust
fn init(base: Base<Self::Base>) -> Self {
    Self { base }
}
```

All of these are the exact same patterns you've already seen.

## handlers
We have a lot of async in this file. We have functions that will get called when signals are emitted and we have things that emit signals. Through that we can dynamically do things. There's one gotcha.

###### on message timer timeout
```
#[func]
fn on_message_timer_timeout(&mut self) {
    let mut message_label = self.base.get_node_as::<Label>("Message");
    message_label.hide();
}
```
Just a get node as. You did need to know that many nodes have a `.hide()` but this is largely the same as [GDScript]()

###### on start button pressed
```rust
#[func]
fn on_start_button_pressed(&mut self) {
    let mut start_button = self.base.get_node_as::<Button>("StartButton");
    start_button.hide();
    self.base.emit_signal("start_game".into(), &[]);
}
```
You've emitted a signal before in the player code. Did you remember the [Variant]()?

###### update score
```rust
#[func]
pub fn update_score(&mut self, score: f32) {
    let mut score_label = self.base.get_node_as::<Label>("ScoreLabel");
    score_label.set_text(score.to_string().into());
}
```
Type casting is the challenge here. A `to_string()` and an `into()`. Why's that?

Score is a number, so we convert that to a string. Lots of ways to do it. Then we call `into()` because when we are putting things into nodes or the godot engine we need to make them `GStrings`. That way they work within the godot's lifecycles and we can forget about managing them on our end.

###### show message text
```rust
#[func]
pub fn show_message_text(&mut self, text: GString) {
    let mut message_label = self.base.get_node_as::<Label>("Message");
    message_label.set_text(text);
    message_label.show();
    let mut message_timer = self.base.get_node_as::<Timer>("MessageTimer");
    message_timer.start()
}
```
A stress test of what you know so far. Get the two nodes. Set the text. Show text. Start a timer. Did you do yours as a `GString`? If you didn't that's fine, but know you may need to make a change elsewhere. 

###### show game over
```
pub fn show_game_over(&mut self) {
    self.show_message_text("Game Over".into());

    let mut message_label = self.base.get_node_as::<Label>("Message");
    // TODO: FIXME This isn't the same as the game
    message_label.set_text("Dodge The Creeps".into());
    message_label.show();

    let mut button = self.base.get_node_as::<Button>("StartButton");
    button.show()
}
```
And the gotcha. We have a function that sets the label twice. Which means that if we rely on `show_message_text` we'll have bad behavior. We'll try to show one text and then immediately overwrite it with another. In the [GDScript Example]() this is solved with two timers. Here we TODO:

With that we're done. Be sure to do the [GDScript Example's]() editor side of this. You should have all the tools you need to make it work.

[previous]() [next]()
