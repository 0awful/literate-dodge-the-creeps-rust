# The Hud

We take a brief divergence from the GDScript tutorial to [Set up the hud](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/index.html#contents) before the Main Scene. We do this because we want to use this in the Main Scene. Its easier to have it made before you need to use it.

The hud is much of what you've already encountered however it has a greater emphasis on the `Hud` `impl`. Our hud will expose functions for our main scene to use. We do that by having public functions on our struct. That being said the shape of the file is very much the same. Give it a go. Here's the [GDScript Ref](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/index.html#contents)

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

![cat](https://images.pexels.com/photos/1056251/pexels-photo-1056251.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1)

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
Just a get node as. You did need to know that many nodes have a `.hide()` but this is largely the same as [GDScript](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/06.heads_up_display.html)

###### on start button pressed
```rust
    #[func]
    fn on_start_button_pressed(&mut self) {
        let mut start_button = self.base.get_node_as::<Button>("StartButton");
        start_button.hide();
        self.base.emit_signal("start_game".into(), &[]);
    }
```
You've emitted a signal before in the player code. Did you remember the [Variant](https://docs.godotengine.org/en/stable/classes/class_variant.html)? ([Rust Docs Ref](https://godot-rust.github.io/docs/gdext/master/godot/builtin/struct.Variant.html))

This one has a gotcha later on. Here's what the official example has to say:

> Note: this works only because `start_game` is a deferred signal.
> This method keeps a &mut Hud, and start_game calls Main::new_game(), which itself accesses this Hud
> instance through Gd<Hud>::bind_mut(). It will try creating a 2nd &mut reference, and thus panic.
> Deferring the signal is one option to work around it.

This means in the editor when you attach the signal you must also check the 'deferred' checkbox.

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

        let mut timer = self.base.get_tree().unwrap().create_timer(1.0).unwrap();
        timer.connect("timeout".into(), self.base.callable("_show_start_button"));
    }

    #[func]
    fn _show_start_button(&mut self) {
        let mut message_label = self.base.get_node_as::<Label>("Message");
        message_label.set_text("Dodge The Creeps".into());
        message_label.show();
        let mut button = self.base.get_node_as::<Button>("StartButton");
        button.show()
    }
```
And the gotcha. We have a function that sets the label twice. Which means that if we rely on `show_message_text` we'll have bad behavior. We'll try to show one text and then immediately overwrite it with another. In the [GDScript Example](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/06.heads_up_display.html#startbutton) this is solved with two timers. They `await` one of them. Well here we can't do that. You just can't `await` a `Timer`

Its an [known issue](https://github.com/godot-rust/gdext/issues/432) that has been marked "Not possible at the moment" and "Quite a pain to get working." So we're boned. Abandon all hope. We've failed.

Of course not. In that same issue they call out a fix. Its the one you see above.

Here's a great time to mention you can connect signals in rust. This is how you do it. You have a node that emits a signal and a callable that you will connect to the signal. Here that node is the timer node. If the node already existed we could use `get_node_as` to get it, but because we don't we'll create one. We get the node tree and then add a node.

With a reference to our brand new timer node we call `.connect`. The first param is the signal on the timer node we want to connect to. This is a string so be careful to match it correctly. Then you call `self.base.callable()` to make a callable function from one of the functions you have defined. It needs to have the `#[func]` macro to make it work this way. Then you enter the name of the function you want to call. We have prefixed the function name with an `_` to make `cargo` happy. It can't tell that the function is being called so prefixing it with underscore lets it know not to worry about it.

The function body is what the [GDScript Example](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/06.heads_up_display.html#) does.

With that we're done. Be sure to do the [GDScript Example's](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/06.heads_up_display.html#) editor side of this. You should have all the tools you need to make it work.

[Previous Page](https://0awful.github.io/literate-dodge-the-creeps-rust/code-the-mob) [Full Hud Code](https://github.com/0awful/literate-dodge-the-creeps-rust/blob/main/src/rust/src/hud.rs) [Next Page](https://0awful.github.io/literate-dodge-the-creeps-rust/code-the-main-scene)
