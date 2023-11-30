# View The Tutorial [Here](https://0awful.github.io/literate-dodge-the-creeps-rust/)

# Dodge the creeps Rust
This project is going to look atypical if you look in some places and very typical if you look in others. This is the dodge the creeps example built using gdext, referencing and building upon gdext's example, and with direct reference to godot's dodge the creeps tutorial. This aims to be the second tutorial that you see in your process of learning gdext. You should know how to set up a project and the absolute basics of gdext. Additionally it will not redescribe the details of how godot works. You can find many other examples that would do that better than this document ever could. 

This aims to highlight differences between gdscript and gdext and call out the specific editor level differences you will encounter on this path. After completing this tutorial you should be at the same skill level with gdext as you would be finishing the gdscript dodge-the-creeps. 

# This Repository

If you are unfamiliar with [literate programming](https://en.wikipedia.org/wiki/Literate_programming) this repository should look very strange to you. Fundamentally it is the exact same as writing a blog post on programming dodge the creeps with the notable difference that the code snippets are then extracted out to directly form the example. You will therefore need to learn a small [DSL](https://en.wikipedia.org/wiki/Domain-specific_language) to maintain this document. This project uses the literate programming conventions defined by [knot](https://github.com/mqsoh/knot) (Thanks [mqsoh](https://github.com/mqsoh])). 

The rust solves all but one function of the example. Its technically wrong in the examples at gdext for the reason called out in the code. The links are broken because I needed to set the repo up before I could set the links up, and because I got tired. The explanations are rough rough draft. There's suposed to be cat photos. There's probably a dozen todos. Mostly because I don't understand why and didn't want to bother the discord until I had this happening.

I didn't do the godot side of it. Mainly because I have this code doing the godot side of it in another directory, I just did the godot side of it, and I'm tired now. It'll be there soon. 

# Building Locally
```
./tangle.sh
```
Requires docker.

Then `cargo build` to build the rust code

# CI
I'll set up CI soon. Mostly to show it can be done. Then set up automated deployments to somewhere. Pages probably. Its easy to build. Github actions are free for public repos. It'll be an easy setup.

# Contributing
The future of this project is largely in flux. It could end up as a part of Gdext. Could end up as a solo blog. Regardless this is the absolute infancy and contributions are appreciated, but comments are desired. I want to know what works what doesn't and thoughts overall more than getting code commits. Its in a DSL after all. 
