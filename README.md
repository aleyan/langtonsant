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
    langtonsant [OPTIONS]
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
OPTIONS:
    -s, --sleep <MILLISECONDS>    Sets a custom sleep time between steps. [default: 0]
    -m, --maxsteps <STEPS>        Maximum number of steps ant takes before stopping. [default: 15000]
```

## A Note on Terminal Graphics
This tool makes use of the `▀` block element character to draw in the terminal,
and setting ANSi codes for foreground and background colors as required. For 
best results your terminal should use a font as close to 2:1 height:width ratio 
as possible. The demo animation was recorded in Alacritty terminal with Iosevka.
