# View The Tutorial [Here](https://0awful.github.io/literate-dodge-the-creeps-rust/)

# Dodge the creeps Rust
This project is going to look atypical if you look in some places and very typical if you look in others. This is the dodge the creeps example built using gdext, referencing and building upon gdext's example, and with direct reference to godot's dodge the creeps tutorial. This aims to be the second tutorial that you see in your process of learning gdext. You should know how to set up a project and the absolute basics of gdext. Additionally it will not redescribe the details of how godot works. You can find many other examples that would do that better than this document ever could. 

This aims to highlight differences between gdscript and gdext and call out the specific editor level differences you will encounter on this path. After completing this tutorial you should be at the same skill level with gdext as you would be finishing the gdscript dodge-the-creeps. 

# This Repository

If you are unfamiliar with [literate programming](https://en.wikipedia.org/wiki/Literate_programming) this repository should look very strange to you. Fundamentally it is the exact same as writing a blog post on programming dodge the creeps with the notable difference that the code snippets are then extracted out to directly form source code and an executable. You will therefore need to learn a small [DSL](https://en.wikipedia.org/wiki/Domain-specific_language) to maintain this document. This project uses the literate programming conventions defined by [knot](https://github.com/mqsoh/knot) (Thanks [mqsoh](https://github.com/mqsoh])). 

It is mostly complete. The Godot side has not been implemented and placed in version control. It will be eventually.

# Building Locally
```
./tangle.sh
```
Requires docker.

Then `cargo build` to build the rust code

# CI
We use CI processes to do the following:
1. We tangle the .md files into rust files
2. We format those files with rustfmt
3. We build the rust code
4. We deploy the code from the main branch to github pages.

This has a wrinkle

## Rustfmt will always be unhappy with the code

This happens because our rust code will have whitespaces from the indentation of the code blocks. Rather than changing the formatting of some blocks into something that would look stranger we accept these whitespace errors. You should still read the formatting log to see what its mad about. If its not a whitespace error you should fix it. Eventually a port of the rustfmt action will be made which has better compatibility in that regard.

# Contributing

You are welcome to open PRs. The CI tools will do a lot of the stuff behind the scenes. You won't need to build locally. If you update the `.md` files it will just work. The build check will let you know if you've failed a build. Check rustfmt and make it as happy as you can, don't worry about whitespace errors. Then tag @0awful and I'll review it for you.
