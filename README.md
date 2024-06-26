**riirando** is a reimplementation of the [Ocarina of Time randomizer](https://github.com/OoTRandomizer/OoT-Randomizer).

# Goals

* Compatibility with the existing implementation via the `.zpf` file format
* Experiment with new algorithms at the core of the randomizer (especially the fill algorithm)
* Experiment with a replacement for the Python-based logic syntax
* Prototype an overhaul of the settings system to eliminate redundancy, make settings randomization more powerful, and allow separate settings per world
* Find bugs by reimplementing things in a programming language with a strict compiler
* Experiment with augmenting the ASM/C code with Rust for better ergonomics
* Write a Rust-based compressor and decompressor that run on NixOS, based on the new MIT-licensed [compressor](https://github.com/CMuncey/Zelda64_Compressor) and [decompressor](https://github.com/CMuncey/OoT_Decompressor) written in C
* Prototype [a new GUI](https://gist.github.com/fenhl/394e09e8ea5ac5e552c8c61d016992a6) that's more tightly integrated into the rest of the codebase and addresses some of the [open GUI issues](https://github.com/OoTRandomizer/OoT-Randomizer/labels/Component%3A%20GUI%2FWebsite)

# Non-goals (for now)

* Cosmetics and generating from patch files. Those are best handled by the offline patcher.
* Compatibility with the web patcher. It's closed source and breaks with patch files that work fine in the offline patcher, making it difficult to debug.
* Glitched logic. I don't know enough about glitches to support it, but contributions are welcome.

# Design principles

* All randomization is turned into a spoiler log, which is then used as a plando to generate the rom. This ensures that the spoiler/plando compatibility remains fully featured.
