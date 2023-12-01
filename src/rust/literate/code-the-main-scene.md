# Main

This is where it all comes together. Up until now your experience has largely been that of using the editor and writing code. Now we get to play the game. You'll likely catch something you've forgotten here.

Take a moment and try to do it without any guidance. But you will hit things you have not done. Here is the [GDScript Ref](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/05.the_main_game_scene.html)

![cat](https://images.pexels.com/photos/2835623/pexels-photo-2835623.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1)

# Implementing
## High level structure
###### file:../src/main_scene.rs
```rust
<<imports>>

<<struct>>

<<struct impl>>

<<class impl>>

```
## Imports
###### imports
```rust
use godot::engine::{AudioStreamPlayer2D, Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

use crate::hud::Hud;
use crate::mob::Mob;
use crate::player::Player;
```
We have a couple new things. We pull in the other file's structs so that we can use the types within. 

## The Struct
Here's where things are going to get different.

In the official tutorial they call out a packed scene. We handle that here and in the ready. Lets dive in.

###### struct
```rust
#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainScene {
    pub score: real,
    pub mob_scene: Gd<PackedScene>,
    #[base]
    pub base: Base<Node>,
}
```

Everything outside of the `PackedScene` is typical. However we do have a `Gd<T>`. This is a very important type in gdext. So important in fact that it is on [the first page](https://godot-rust.github.io/docs/gdext/master/godot/#type-categories) of the gdext docs. Please refer to the documentation [here](https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html) to learn more 

## INode
This is the next easiest part of this code. It has one new function and it's just like the `player` code in how we use it.
###### class impl
```rust
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
```
As you can see we instantiate the PackedScene as a new empty, like we did the with `Player.screen_size`. Then we assign a value to it with `load` in the `ready` call. This is once again because lifecycles. But you could also argue its because [`load()` might fail](https://godot-rust.github.io/docs/gdext/master/godot/engine/fn.load.html). In rust when you use the struct syntax that we chose in `init` it by convention should not be failable. If it's failable you should use `builder` semantics and provide a `Result<T, E>` value instead. We would be violating that if `load` failed. 

# The Main Event
This is where things start to go in direction's you'll find challenging. Lets start with the easy parts.

###### struct impl
```rust
#[godot_api]
impl MainScene {
<<game over>>

<<new game>>

<<score timer timeout>>

<<start timer timeout>>

<<mob timer timeout>>
}
```

The [GDScript Tutorial](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/05.the_main_game_scene.html#main-script) had you code timeout functions. They are very similar in rust land.
## Easy Timeouts
###### start timer timeout
```rust
    #[func]
    pub fn on_start_timer_timeout(&mut self) {
        let mut score_timer = self.base.get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base.get_node_as::<Timer>("MobTimer");
        score_timer.start();
        mob_timer.start();
    }
```
Look at that. This isn't hard.

###### score timer timeout
```rust
    #[func]
    pub fn on_score_timer_timeout(&mut self) {
        self.score += 1.0;

        let mut hud = self.base.get_node_as::<Hud>("HUD");
        hud.bind_mut().update_score(self.score);
    }
```
The gotcha here is `bind_mut`. Its because the hud is another rust component. To gain access we use bind. Because we are performing a modifying call we have to `bind_mut()`

# New Game
This is a lot of things you chould be okay with doing. There's the same `bind_mut()` gotcha. But once you're past that its the same game
###### new game
```rust
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
```

We've got the music player calls, the hud calls, a show message call. But at the end of the day this is just `get_node_as` with a couple instances of `bind_mut()`

Good job if you got this one. No worries if you didn't. `bind_mut` is a curve ball.

# The Hard Timeout
This is the hardest part. Of the entire application. Every line of code. This right here is the hardest. If you didn't get it no worries. If you did congrats. I know I didn't.

###### mob timer timeout
```
    #[func]
    pub fn on_mob_timer_timeout(&mut self) {
<<instantiate as>>

<<pick location>>

<<set rotation>>

<<add child>>

<<make the mob go>>
    }
```

Lets talk about why its hard before we get into it.

We have a somewhat solvable problem of picking the location on the path. That's something you could pretty easily get. At the same time. Its using ranges and RNG as well as setting progress. 
###### pick location
```rust
        let mut mob_spawn_location = self
            .base
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);

        mob_spawn_location.set_progress(progress as f32);
        mob_scene.set_position(mob_spawn_location.get_position());
```

But you may not have even hit this problem because

###### instantiate as
```rust
        let mut mob_scene = self.mob_scene.instantiate_as::<RigidBody2D>();
```

To even see this part of the problem you need to crack `instantiate_as`. You've never used it before. Its the only time we use it in this project. We use this because we are turning a scene into a node. `instantiate_as::<T>()` takes a `PackedScene` and returns a `Gd<T>` (I told you `Gd<>` was important). But this function is failable. This will panic and crash if it can't become `T`. See the documentation [here](https://godot-rust.github.io/docs/gdext/master/godot/prelude/trait.PackedSceneExt.html#method.instantiate_as).

If you were productionizing this you might want to use `try_instantiate_as` which would then allow for you to either attempt to recover or gracefully exit with a helpful error message. 

###### set rotation
And then you have the hardest math problem in the project. If you aren't up on your geometry, well, you aren't going to solve it. I view myself as a mathy kind of person. I couldn't do this. If you know you know. If you don't that's what tutorials are for. To be fair. You could've stolen this from the [GDScript Tutorial](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/05.the_main_game_scene.html#main-script) but you would've needed to crack a few other tough parts to even get here.
```
        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);

        mob_scene.set_rotation(direction);
```

Making it go is a little easier, but has a weird wrinkle
###### make the mob go
```rust
        mob.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));
```

So you `set_linear_velocity()` with your brand new `Vector2` that you give a little rotation to. It's not hard. I totally didn't spend an hour trying to crack this. Nope. That didn't happen.

Also you use `real::from_f32` to support double precision if you want it.

To cap it all off you have to add a child programatically. Which is the first and only time you do it in this project.

###### add child
```rust
        self.base.add_child(mob_scene.clone().upcast());

        let mut mob = mob_scene.cast::<Mob>();
        let range = {
            let mob = mob.bind();
            rng.gen_range(mob.min_speed..mob.max_speed)
        };
```

Okay so the easy parts first. We call `self.base.add_child()`. It does what it says on the tin. There is a `.clone()` and `.upcast()` this is to play nice with rust's borrowing and type semantics. If its going to leave your scope, you usually make a clone. You can do more things. `move` or the like. But `clone()` is fine. If you need even more performance then evaluate that. But for our case `move` isn't possible because we still need that scene. We `.cast` that into `Mob` and get the `mut mob` variable.

This is the easy stuff. Here's the headscratcher. You open a closure to make a local scope. We call `let mob = mob.bind()` so we've shadowed mob into its own thing. Then we generate a range with `.gen_range(...)`. Notice the absent semi colon. That means its return it assined to the `range` variable.

So we've created a range with extra steps? You might ask. Yes, and we're better for it. These extra steps are allowing a borrow to occur. When you `bind()` you are getting something like a `RefCell`. This means you've performed a `borrow`. Which will expire at the end of the scope. But we don't need it past this moment so we do it in a local scope so we can return the `borrow` as fast as possible. If you are still fuzzy, that's okay. This is a lot of knitty gritty rust. Here's some links. [bind](https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html#method.bind), [borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html), 
[Gd](https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html#)
[refcell in easy rust](https://fongyoong.github.io/easy_rust/Chapter_42.html) and [refcell in the rust book](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

If you struggled to get this. Of course you did. I don't believe anyone doing this as their first time could have arrived at this themselves. But now you know why and how to do it for the future. Lets wrap this up.

# Game Over
This is using something new, but if you referenced the docs, you might have been able to get it. Its very intuitive.
###### game over
```rust
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
```
You need to get the tree and then tell it to "queue_free". This requires you to get the `base` `get_tree` on the base. That returns an optional so `unwrap()` then `call_group` to free it. The only hard part is the `variant`. If you remember from the `emit` part of this project you can make an empty variant with `&[]`. We don't need to say anything other than `queue_free`. So `&[]` will get the job done. 

You have a `bind_mut()` which you should be starting to get comfortable with. Everything else is just about what you expect.

# Make it go

At this point you've now crafted everything you need to make the game happen. Follow along with [GDScript](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/05.the_main_game_scene.html#) for setting up the nodes. Don't forget the [Finishing Up Sections](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/07.finishing-up.html) But outside of that you've completed it. 

# Next steps:

Go check out the [Gdext Github](https://github.com/godot-rust/gdext). If you have any questions with Godot Rust (not this tutorial though :sweat_smile:) join us in the [discord](https://discord.com/invite/aKUCJ8rJsc).

If you notice any errors in this tutorial or anything is confusing, go to the [tutorial github](https://github.com/0awful/literate-dodge-the-creeps-rust) and make a change to this code to correct it. I love the help. And welcome to the the wonderful world of godot rust. 

###### file:../src/lib.rs
```rust
use godot::prelude::*;

pub mod hud;
pub mod main_scene;
pub mod mob;
pub mod player;

struct DodgeTheCreeps;

#[gdextension]
unsafe impl ExtensionLibrary for DodgeTheCreeps {}

```
Ps. This is what [lib.rs](https://github.com/0awful/literate-dodge-the-creeps-rust/blob/main/src/rust/src/lib.rs) should look like


[Previous Page](https://0awful.github.io/literate-dodge-the-creeps-rust/code-the-hud) [Full Main Scene Code](https://github.com/0awful/literate-dodge-the-creeps-rust/blob/main/src/rust/src/main_scene.rs) 
