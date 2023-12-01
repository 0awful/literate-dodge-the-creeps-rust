# Setting Up The Project

Follow the examples given in [The Godot Rust Book](https://godot-rust.github.io/book/intro/index.html) to setup the initial conditions. We will be using the following directory format.

```
/godot
-- Godot things here
/rust
-- Rust things here
```

This is indentical to the described format. Additionally set up the `.gdextension` as described by The Godot Rust Book. We have named ours `rust.gdextension` but the name (`{name}.gdextension`) can be anything. It is purely for your ability to understand what the file is for.

Next you'll need the assets. We use the same assets as [The Godot Docs](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/01.project_setup.html). Download them and put them in the same directories the godot docs describe.

Additionally follow their advice on setting "Viewport Height," "Viewport Width" and "Aspect Ratio" then return here for the first differences.

# The Player Scene

In the godot example you create the nodes first and then create the scripts. I find it works better with gdext to work from the script first and then go to the node levels. Why I prefer this will become very clear shortly.

Lets start with making a `player.rs` file at `/rust/src/player.rs` in the future you can assume all rust files will be in the `/src` directory but any directory format that is legal rust will work. 

###### set up the player struct
```rust 
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    speed: real,
    screen_size: Vector2,
   
    #[base]
    base: Base<Area2D>,
}
``` 

Here we have defined the player struct. This is the internal information that will be used on the [node](https://docs.godotengine.org/en/stable/classes/class_node.html). We use [rust macros](https://doc.rust-lang.org/book/ch19-06-macros.html) to make the struct compatible with godot. 

Additionally we set the `base` and use the `#[base]` macro. This will give us a lot of power later and you'll come to really enjoy this pattern. But it is opt in. You don't need it, but you probably want it most of the time. 

Because the base node's type is Area2D we set it here in the `#[class(base=<T>)]` macro. Which can be any node type. We also use `#[derive(GodotClass)]`, which was covered in the [Hello World Tutorial](https://godot-rust.github.io/book/intro/hello-world.html). This means you must also import it `use godot::engine::Area2D` will get it for you. You also need `use godot::prelude::*`. The full imports will be shown off later.

## Programming the scene

We need to do a couple things for the player to do what we'd like. We need to set up animations, player movement, collisions, and spawning the character. Our character will be removed from the game when it hits a mob and respawned when we start the game. Lets start with the most easy to understand. The player movement.

###### player process
```rust
fn process(&mut self, delta: f64) {
    let mut velocity = Vector2::new(0.0, 0.0);

    let input = Input::singleton();
    if input.is_action_pressed("move_right".into()) {
        velocity += Vector2::RIGHT;
    }
    if input.is_action_pressed("move_left".into()) {
        velocity += Vector2::LEFT;
    }
    if input.is_action_pressed("move_down".into()) {
        velocity += Vector2::DOWN;
    }
    if input.is_action_pressed("move_up".into()) {
        velocity += Vector2::UP;
    }

    <<get sprite node>>
    if velocity.length() > 0.0 {
        velocity = velocity.normalized() * self.speed;

    <<animate sprite node>>
    } else {
        <<handle stopping the animation>>
    }

    <<move the player>>
}
```

This means we also need to set up these actions within our godot editor. This is done identically to the godot example so reference the official dodge the creeps example for setting that up. The same will be true of animations and setting up the collision shape. But we need to do a little more before we can set that up.

And lets return to moving the player
###### move the player
```rust
let change = velocity * real::from_f64(delta);
let position = self.base.get_global_position() + change;
let position = Vector2::new(
    position.x.clamp(0.0, self.screen_size.x),
    position.y.clamp(0.0, self.screen_size.y),
);
self.base.set_global_position(position)
```
Here we have a couple things we can talk about.

1. `self.base.{property}` vs `self.{property}`
2. The `real` type.

## `self.base.{property}` vs `self.{property}`
As you may remember we opted to set the base value on the player struct. We didn't have to do it, but we chose to. Here is where we first get the benefits from choosing to do this. `self.base` is a reference to the properties of the godot node. We use it to modify the position of that godot node. If you need to change something in the engine, you'll often do that through `self.base`. Additionally we use `self.screen_size` in this case. This is a reference to the property on the `player` struct. Anything you add to the struct you can get from `self.{property}`. You'll see how more shortly. 

## The `real` type
The `real` type is a godot floating point number. Is is f32 or f64? Yes. Here's the [official real docs](https://godot-rust.github.io/docs/gdext/master/godot/prelude/type.real.html). You can choose to make it f64. But in general you ignore the underlying floating point type and convert your `float`s into `real`s. Here we do it by using `real::from_f64(delta)`. Where `delta` is the `process(delta)` value or the time between two frames. `delta` more complicated than that in reality, but you don't need to know how or why at this time. If you're curious check out [Understanding Delta at KidsCanCode](https://kidscancode.org/godot_recipes/4.x/basics/understanding_delta/index.html) and [this video from Jonas Tyroller](https://www.youtube.com/watch?v=yGhfUcPjXuE) for even more depth.

# Initializing the player node
Okay so we can move the player with that code, but where does it go? If you did the required reading of the [Rust Hello World Tutorial](https://godot-rust.github.io/book/intro/hello-world.html) you probably already know. But if you don't that's fine too.

Our player.rs file has the following loose structure.
###### file:../src/player.rs
```
<<player imports>>

<<set up the player struct>>

<<define the player impl>>

<<define the IArea2D impl>>
```

The `move the player` code goes in `IArea2D` `impl` which handles the `init` `ready` and `process` functions. 

###### define the IArea2D impl
```rust
#[godot_api]
impl IArea2D for Player {
    <<player init>>

    <<player ready>>

    <<player process>>
}
```

And here we have another macro `#[godot_api]`. This does magic behind the scenes. If you leave it off things won't work correctly, but `cargo` will nicely let you know it is missing. 

We just wrote the `player process code` leaving a few placeholder's we'll fill out shortly. Lets wrap up the `IArea2D` `impl` first.

###### player init
```rust
fn init(base: Base<Area2D>) -> Self {
    Player {
        speed: 400.0,
        screen_size: Vector2::new(0.0, 0.0),
        base,
    }
}
```

Because we use base in the `Player` `struct` we need to have base as a parameter in our `init`. This is then handled automagically by `gdext`. 

But also what the heck. `screen_size` isn't `(0.0, 0.0)`. We handle this in `ready`. We do this because the godot gscript tutorial does this. The reason it does that is because of lifecycle methods. Here's the docs on [ready](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html#ready-vs-enter-tree-vs-notification-parented) and [init](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html#init-vs-initialization-vs-export). Given godot recommends you use ready. You should use ready for these sorts of things. You don't need to understand the depths of this at this time. We will talk about it more later if you're curious.

###### player ready
```rust
fn ready(&mut self) {
    let viewport = self.base.get_viewport_rect();
    self.screen_size = viewport.size;
    self.base.hide();
}
```

Here we set the screen size and hide the player. We do this because we don't want them visible when we are on the main menu. 

At this point we could place this node in the scene. It wouldn't do what we want yet. But we could add the node. In order to add a node you need an `init` function. Otherwise the godot editor won't expose it as a node you can add to your scene. We have now used `ready` `process` and `init`. These are the main godot lifecycle methods. You'll be seeing a lot of them.

If you did choose to add the node at this time you may discover some interesting behavior. If you `cargo build` and open the godot editor, then add the node as described by [The hello world tutorial](https://godot-rust.github.io/book/intro/hello-world.html) you may find that later `cargo build`s automatically update your nodes within godot. This is because it is reading from your library when you start the game. This means if you change the library it may pick up the changes. But it doesn't do this always and doesn't do it for all kinds of changes. You can force it to pick up your changes by restarting the editor. We'll talk more about this in the next section.

# Preparing for collisions and the player impl
We have some functions that we put in the `I{NodeName}` `impl` and others we put in the `Player` `impl`. This distinction is because we have some code that is attaching to existing engine functions and others that are purely our creations. If its ours it goes in the `Player` `impl`. This distinction will become easier to understand with time.

###### define the player impl
```rust
#[godot_api]
impl Player {
    <<hit signal>>

    <<player collision logic>>

    <<player start logic>>
}
```
We will have this rough structure.

###### player collision logic
```rust
#[func]
fn on_player_body_entered(&mut self) {
    self.base.hide();
    self.base.emit_signal("hit".into(), &[]);
 
    let mut collision_shape = self
        .base
        .get_node_as::<CollisionShape2D>("CollisionShape2D");

    collision_shape.set_deferred("disabled".into(), true.to_variant());
}
```

So when we are hit we hide our player, hide the collision body, and emit a signal. Nothing special right? Well if you do some digging into the [engine documentation](https://docs.godotengine.org/en/stable/classes/class_area2d.html#methods) you'll find that `body entered` (the signal we'll attach to this) has an argument it provides.

![body entered docs snippet](https://github.com/0awful/literate-dodge-the-creeps-rust/tree/main/assets/body_entered.png)

This means we could take a value but are opting not to. This will come back in the near future.

`emit_signal` has two values. The first is the signal cast into an engine string. The second is the properties to put on that signal. We don't want to set any so we place an empty value of `&[]`. This is an empty [Variant Type](https://docs.godotengine.org/en/stable/classes/class_variant.html).

We also use `true.to_variant()`. Its another Variant Type. This is a dynamic godot engine type. It can be many things. The engine uses it in several places. Here we cast to it like we typecast many other things. Nothing too special about it. Something to be aware of. You can learn more [here](https://docs.godotengine.org/en/stable/classes/class_variant.html).

Now it is time to set up our signal.
###### hit signal
```rust
#[signal]
fn hit();
```

That's it. Signals are very easy.

Lets set up the start function. We need this because we will be respawning the player and therefore can't rely on the `init` and `ready` functions to handle all of this logic.

###### player start logic
```rust
#[func]
pub fn start(&mut self, pos: Vector2) {
    self.base.set_global_position(pos);
    self.base.show();

    let mut collision_shape = self
        .base
        .get_node_as::<CollisionShape2D>("CollisionShape2D");

    collision_shape.set_disabled(false);
}
```

We set the player's position, show the player and enable collision. Standard. 

Now seems like a great time to talk about `get_node_as::<T>("STRING")`

This is how you access the children of your node. Our player node will have a few children. You've seen us get a `CollisionShape2D`, we will also have an `AnimatedSprite2D`. When we add our player to the scene we will have to manually add these children in the editor. This is why it is easier to do the rust code first. Because we cannot add the player node until we write this code, so we would have to make a temporary node. Give it children. Then move those children to our player and then make that player the root of the scene. We could absolutely do it that way. But I'm of the belief this is easier. 

The string is the name of the node. We can rename the node in the scene to anything. If you have multiple of the same type you will be forced to rename at least one of them. No two nodes can have the same name. You may find errors here. If you wrote `"CollisionShape2d"` instead of `"CollisionShape2D"` you'd have an error that may be difficult to diagnose. Be aware of this and check this when you debug.

# Animating the player

We're inches away from having the rust code complete. We have to write the code for `get sprite node` `animate sprite node` and `stop the animation`

I invite you right now to try to implementing `get the sprite node`. You know all you need to do it. Once you've tried and even if you've failed, come back and move on.

Here I'll give you a little picture

![cat](https://images.pexels.com/photos/45201/kitty-cat-kitten-pet-45201.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1)


###### get sprite node
```rust
let mut animated_sprite = self
    .base
    .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
```

This pattern is going to be central to `gdext` development. You will write a line like this one in every project. It'll probably be the first line you can write by memory. If you have autocomplete, code snippets, or the like, this would be a great candidate for something to set that up for.

Now its time to animate. This is a little more difficult. Remember this is happening in the context of an `if` block in our `process` function.

###### animate sprite node
```rust
let animation;

if velocity.x != 0.0 {
    animation = "right";
    animated_sprite.set_flip_v(false);
    animated_sprite.set_flip_h(velocity.x < 0.0)
} else {
    animation = "up";
    animated_sprite.set_flip_v(velocity.y > 0.0)
}

animated_sprite.play_ex().name(animation.into()).done();
```

Here we are stringly matching the name of animations we will set up in the editor. If they don't match exactly you'll have errors. We also handle flipping x/y

What does play_ex mean? Well in this case we need to give it a little more information to get our desired behavior. We need to tell it which animation to play. We do that by calling the `_ex()` variant. That returns a [`ExPlay`](https://godot-rust.github.io/docs/gdext/master/godot/engine/animated_sprite_2d/struct.ExPlay.html). On that `ExPlay` we call `.name()` with the name of the animation to play and then `.done()` to play it. 

If you didn't need to specify extra information you could call `.play()`.

This pattern is common, where a simple method is exposed like `.play()` or `.connect()` which runs via the godot default values. If you need to use different values you call the `_ex()` version of the function instead and chain the values you need to set. This is [`Builder` semantics](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html).

Once again you now know all you'd need to know to stop the animation. Take a stab at implementing it. It occurs in the else block of the `velocity.length() > 0.0` `if` statement. 

Here's another photo.

![cat](https://images.pexels.com/photos/416160/pexels-photo-416160.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1)

###### handle stopping the animation
```rust
animated_sprite.stop();
```

Its that easy.

We don't need to say anything special so we don't need to use `.stop_ex()`

If you editor didn't automatically perform the imports for you. Here are the imports.
###### player imports
```rust
use godot::engine::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D};
use godot::prelude::*;
```


With this you have now implemented all the rust code for the player sprite. You can find the combined code at the [player node example code](https://github.com/0awful/literate-dodge-the-creeps-rust/blob/main/src/rust/src/player.rs). Lets now tackle setting it up in the editor.

# The Final Step
The last step is an addition to your `lib.rs`. Using standard rust semantics you expose the module.

###### Example Lib.rs
```rust
use godot::prelude::*;

struct MyExtension;

pub mod player;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
```

You will do this for all code you wish to use in godot.

[Full Player Code](https://github.com/0awful/literate-dodge-the-creeps-rust/blob/main/src/rust/src/player.rs) [Next Page](https://0awful.github.io/literate-dodge-the-creeps-rust/using-player-in-the-editor)
