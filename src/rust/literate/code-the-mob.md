# Coding the Mob

It's time for you to give this another go. From what you know now you can be pretty successful in completing the mob rust code. I'll give you a high level overview to lead you in the right direction.

###### file:../src/mob.rs
```
<<file imports>>

<<mob struct>>

<<mob impl>>

<<mob RigidBody2D impl>>
```


###### file imports
```
<<typical imports>>

<<a special import>>
```

###### mob impl
```rs
<<api macro>>
impl Mob {
    <<offscreen cleanup>>
}
```

###### mob RigidBody2D impl
```rs
<<api macro>>
impl IRigidBody2D for Mob {
    <<init>>

    <<ready>>
}
```

Please give it a go and come back. Here is a cat for moral support

![]()

If you'd like to work through pure code the code example is [here](), but you'll probably learn more by trying and failing once first. I will also walk you through this file.

# Implementation
Lets start with the simple. Did you remember the API macro?

###### api macro
```rust
#[godot_api]
```

Here's the mob. Nothing special.

###### mob struct
```rs
#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    pub min_speed: real,
    pub max_speed: real,

    #[base]
    base: Base<RigidBody2D>
}
```

We set min and max speed like in the [GDScript Tutorial]()

Next we move on to the init of the impl. Its not much different from the `player`.

###### init
```rust
fn init(base: Base<RigidBody2D>) -> Self {
    Mob {
        min_speed: 150.0,
        max_speed: 250.0,
        base,
    }
}
```

You've got some magic numbers you could pull out as consts if you'd like. 

###### ready
```rust
fn ready(&mut self) {
    let mut sprite = self.base.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
    
    let anim_names = sprite.get_sprite_frames().unwrap().get_animation_names();
    let anim_names = anim_names.to_vec();
    let mut rng = rand::thread_rng();
    
    let animation_name = anim_names.choose(&mut rng).unwrap();
    
    sprite.set_animation(animation_name.into());
    sprite.play();
}
```

Here you use the mysterious 'special import' great work if you got it. Rust has a `rand` crate everyone uses and the default language doesn't have any randoms. This means we have an update to the cargo.toml. I'll share that later if you're following along.

###### a special import
```rust
use rand::seq::SliceRandom;
```

and for good measure here's the normal imports

###### typical imports
```rust
use godot::engine::{AnimatedSprite2D, IRigidBody2D, RigidBody2D};
use godot::prelude::*;
```

And we have what I imagine would've been the hardest to guess, but the most intuitive. Here's how you free the memory.

###### offscreen cleanup
```
#[func]
fn on_visibility_screen_exited(&mut self) {
    self.base.queue_free();
}
```

You use base because you're acting on the godot node and not your struct.

# Add this to lib.rs
```
mod mob;
```

Then you go through the standard process of adding this item to godot. Follow the [GDScript Tutorial](). Nothing is different for this section.

# Cargo.toml
If you're looking for it. Here it is.

###### file:../Cargo.toml
```toml
[package]
name = "dodge-the-creeps"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }
rand = "0.8.5"
```

[Previous]()
[Next]()
