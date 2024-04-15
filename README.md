**riirando** is a reimplementation of the [Ocarina of Time randomizer](https://github.com/OoTRandomizer/OoT-Randomizer).

# Goals

* Compatibility with the existing implementation via the `.zpf` file format
* Experiment with new algorithms at the core of the randomizer (especially the fill algorithm)
* Experiment with a replacement for the Python-based logic syntax
* Prototype an overhaul of the settings system to eliminate redundancy, make settings randomization more powerful, and allow separate settings per world
* Find bugs by reimplementing things in a programming language with a strict compiler
* Experiment with augmenting the ASM/C code with Rust for better ergonomics

# Non-goals (for now)

* Cosmetics and generating from patch files. Those are best handled by the offline patcher.
* Compatibility with the web patcher. It's closed source and breaks with patch files that work fine in the offline patcher, making it difficult to debug.
* Glitched logic. I don't know enough about glitches to support it, but contributions are welcome.
