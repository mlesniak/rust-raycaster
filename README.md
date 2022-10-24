# Overview

This is a simple raycasting engine in Rust for the sake of learning (and playing around) with Rust and SDL2. One of the key priorities was library independence, i.e. we only need a single function

```rust
    fn set_pixel(x: i32, y: i32, r: u8, g: u8, b: u8);
```
 which shall allow us to draw single pixels. Everything else is done via my own code, hence this might easily be ported to other libraries by providing the counterpart of this function and loading images into our own texture format. The texture format allows for inmemory textures as well as loaded images; it shall be easy to extend to dynamic, time-dependent textures, e.g. for blinking, etc.

![Animation](animation.gif)

The animation is a bit laggy since it's a gif. The actual output runs smoothly with 60FPS in 1280 x 960 (and probably way higher resolutions as well).

The map format is (hopefully) self-explanatory: every number corresponds to one texture in `Raycaster::load_textures()`, i.e. for the level shown in the animation it is

```
444444444444444
400000000000004
4000000000000044444
4004404000000000003
4004004000000004444
40040000000000004
40040440000000444
400000000544444
400000000000002
444444444444444
```

Please be aware that it's important that the whole area has to be surrounded by walls, i.e. some non-zero block.

To compile and run this, `libsdl2` and its image support (for loading textures) are necessary, i.e.

    brew install sdl2 sdl2_image
    cargo run --release
