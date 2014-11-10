sm-808
======

This is a rudimentary implementation for the Splice coding challenge found here: https://github.com/mattetti/sm-808/

This version is written in Rust.

To install Rust, follow the directions here: http://doc.rust-lang.org/guide.html#installing-rust

Once Rust is installed, cd into this repo and execute: "cargo run"

This command will build the project and run the code.

I make a couple of assumptions for this implementation, here are a few of the more relevant ones:

Assumption 1: patterns will exist as size 8, 16, or 32. I did this for simplicity; they can be expanded to 64, 128, etc, if so desired.

Assumption 2: host OS has a word size of at least 32-bits. Just in case someone who would wants to run this has a 32-bit VM or something.

Assumption 3: the song should only play as many steps as it takes to play the longest pattern once. I wasn't sure how long to play for, so I just chose this as a safe minimum.

Assumption 4: OS is little-endian (Intel convention). I do some bitshifting to try and be efficient/fancy. I didn't add a check for endianess.
