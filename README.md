# Langton's Ant
An implementation of [Langton's Ant](https://en.wikipedia.org/wiki/Langton%27s_ant)
in Rust that runs from the terminal. Simply execute:
```cargo run```. Watch at a rate your eyes can follow with 
```cargo run -- --maxsteps 500 --sleep 20```.

<p align="center">

![Demo Animation](/assets/langtonsant.gif?raw=true)

</p>


## Usage, Flags, & Options
```
$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/langtonsant --help`
Langton's Ant 0.1.0
Alex Yankov
Simulates Langton's Ant in the terminal
USAGE:
    langtonsant [FLAGS] [OPTIONS]
FLAGS:
    -f, --fillterminal    Fills entire terminal. Does not skip last line.
    -h, --help            Prints help information
    -i, --invisibleant    Do not draw the ant.
    -V, --version         Prints version information
OPTIONS:
    -r, --rotations <SEQUENCE>    A sequence of states of squares that the ant advances by
                                  one with every visit. The state of the square rotates the
                                  ant as encoded in this sequence. The first element of the
                                  sequence initially covers the entire board. Valid elements:
                                  R - Turn 90 degrees to the right
                                  L - Turn 90 degrees to the left
                                  U - Turn 180 degrees
                                  N - No change [default: RL]
    -s, --sleep <MILLISECONDS>    Sets a custom sleep time between steps. [default: 0]
    -m, --maxsteps <STEPS>        Maximum number of steps ant takes before stopping. [default: 15000]
```

## A Note on Terminal Graphics
This tool makes use of the `▀` block element character to draw in the terminal,
and setting ANSi codes for foreground and background colors as required. For 
best results your terminal should use a font as close to 2:1 height:width ratio 
as possible. The demo animation was recorded in Alacritty terminal with Iosevka.

## Interesting Ants
Through some experimentations, I have found the following ants interesting.
Because of symmetry, Rs and Ls can be switched resulting in a drawing flipped
on the horizontal axis (because ant initially faces west). You can run these
ants with `--rotations` option like so:
`cargo run -- --rotations LLRLL`.

* LLRLL - quickly expanding square
* LLRNUR - vertical highway
* LLRRLL - Symmetric shape
* LLRRRR - very slowly expanding square
* LLULU - slowly growing cross
* LLURNU - narrow vertical highway
* LLUUR - 1:5 off vertical highway
* LNULRU - order gets consumed by chaos
* LNUUUL - half order, half chaos, all slowly expanding
* LRLUUU - fancy highway
* LRNLUL - fills 1/8th of the screen in a triangle
* LRRLLL - very slowly growing symmetric shape
* LRRRL - very slowly growing squarish shape
* LRRRRL - very slowly growing symmetric shape inside a square
* LRUURR - blob with 4 wide arms growing out of it
* LULLLL - Spiral
* LUUURN - quickly expanding square turned on a corner
* LUUUUR - medium sized oscillator
* ULLL - Three colored rectangle
* ULLLL -  Cutoff parallelogram
* ULLLU - very quickly expanding square
* ULRRUR - forms a triangle
* UNLUL - forms a an expanding rhombus frame
* URLRUL - Tiny oscillator
* URRLNL - forms a single wide highway after growing chaotically
* LLLLRRRRRLLLLLLRLRLRRRLRLRL - Forms a highway after a long period of chaotic growth (400000 steps)
