# Overview

Raycaster in Rust, i.e. "your own 3D engine" just using primite `SDL::Canvas::draw_line` functions and math (and currently easing my life for the blue background texture, see TODO below).

![Animation](animation.gif)

Animation is a bit laggy since it's a gif. The actual output runs smoothly with 60FPS.

## TODO

[ ] Change from SDL image functions for background to pure pixel-based drawing functions.
[ ] Clean up code :)
[ ] Write detailed README