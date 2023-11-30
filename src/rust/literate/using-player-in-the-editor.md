# Using your player

We've now coded up the player. Now to use it.

First building. If you forgot to read the [hello world](https://godot-rust.github.io/book/intro/hello-world.html) tutorial or are unfamiliar with rust. We use [Cargo](https://doc.rust-lang.org/cargo/) in rust land. Execute the following in your rust directory.

```
cargo build
```

This will create your output directory in the standard rust directory of `target/`. Which your `.gdextension` in the `godot/` directory should point to. This is all covered in better detail in the [hello world](https://godot-rust.github.io/book/intro/hello-world.html) tutorial.

At this time you should open your godot editor if it isn't already. If it is already open you may need to restart it. There is a handy shortcut `Project` > `Reload Current Project`

When things are set up correctly you'll know because the `Player` node will now exist within the possible nodes you can create. Clicking `+` or `+ other node` will open the modal. Type player and if you find it you've got it working.

# Adding the children

Godot will let you know your player node is missing some children, but you can't expect this to be exhaustive. You should use your understanding of the rust code and the godot engine to implement this properly. The exact items are called out in the [Godot Tutorial](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/index.html#contents).

# Check you added the keybinds
 
Adding keybinds was called out in the previous section. How to do this is in the [Godot Tutorial](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/index.html#contents). If you have not done this already now would be a good time to do it.

# Animations

There is no difference with animations when you do it via rust. Follow the [Godot Tutorial](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/index.html#contents). Just be sure the names exactly match the strings in your player code.

# The Hit Signal

The only notable difference between godot rust and gscript rust is adding signals. You can do this programmatically in rust, manually code it into the .tscn files. But the most reasonable solution is using the editor. Double clicking the `body_entered(body: Node2D)` event you will open the signal attacking modal. 

Enable "Advanced"
Click the "Player" node
Press "Pick"

You will now have another modal. This one is empty. Had we been writing in gdscript you'd see functions here. But we are not using gdscript. 

Disable "Script Methods Only"

For every other signal this would work fine. But we are using a function that takes an argument and our function doesn't have an identical type. Therefore our desired function is not displayed. 

Disable "Compatible Methods Only"

You should not need to disable that for any other signals in this tutorial.

You can now select `player` ->  `on_player_body_entered(_body: PhysicsBody2D)`

Double Click. Then select connect. 

# Everything else
Everything else exactly mirrors the [gdscript tutorial](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/index.html#contents). Once you've completed the player section you can return here to continue.

[Previous Page](https://0awful.github.io/literate-dodge-the-creeps-rust/code-the-player) 
[Next Page](https://0awful.github.io/literate-dodge-the-creeps-rust/code-the-mob)

