# `mediastat` - an MPRIS client tool to display a "now playing" marquee

`mediastat` is a straightforward, no-nonsense "now playing" marquee generator
for status bars that can display the text output of a Linux command line
application. Written in Rust, the goal is to be light enough that it can run a
couple of times per second to provide a moderately smooth animated marquee with
a simple shell command. It can, theoretically, interface with any
MPRIS-compliant media player (though I've only tested it with
[`spotifyd`](https://github.com/Spotifyd/spotifyd)).
