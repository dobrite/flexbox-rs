### What is this?

Some sort of minimal implementation of flexbox in Rust.

### What is this not?

Spec compliant flexbox implementation suitable for anything.

### How does it work?

Given a tree:

```rust
    let root = Root::new(800, 600, Renderable::View(View::new(Style::new().with_bg(RGB::new(0, 0, 0)).with_flex_direction(FlexDirection::Column), vec![
        Renderable::View(View::new(Style::new().with_width(50).with_height(100).with_bg(RGB::new(255, 0, 0)), vec![])),
        Renderable::View(View::new(Style::new().with_width(50).with_height(100).with_bg(RGB::new(0, 255, 0)), vec![
            Renderable::View(View::new(Style::new().with_width(15).with_height(50).with_bg(RGB::new(0, 125, 125)), vec![])),
            Renderable::View(View::new(Style::new().with_width(15).with_height(50).with_bg(RGB::new(125, 125, 0)), vec![])),
        ])),
    ])));
```

It outputs a `Vec<Command>`

```rust
    Command { bg: RGB { r: 0, g: 0, b: 0 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 0, top: 0, width: 800, height: 600 } }
    Command { bg: RGB { r: 255, g: 0, b: 0 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 0, top: 0, width: 50, height: 100 } }
    Command { bg: RGB { r: 0, g: 255, b: 0 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 0, top: 100, width: 50, height: 100 } }
    Command { bg: RGB { r: 0, g: 125, b: 125 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 0, top: 100, width: 15, height: 50 } }
    Command { bg: RGB { r: 125, g: 125, b: 0 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 15, top: 100, width: 15, height: 50 } }
```

Suitable for input into a user-supplied `Renderer` implementing the `Render` trait. A simple SDL2 renderer
is in the backend folder and used in the sdl2 example.

![flexbox-rs sdl2 example](https://cloud.githubusercontent.com/assets/1541631/15799928/f30fad30-2a21-11e6-8da0-020f4ddfdebb.png)

### Running the example

> NOTE: you'll need SDL2 and SDL2_ttf installed. Installation is platform specific. Please consult the docs.

```bash
$ cargo run --example sdl2 --features backend
```

### Running the tests

> NOTE: you'll need SDL2 and SDL2_ttf installed. Installation is platform specific. Please consult the docs.

```bash
$ cargo test --features backend
```

### Disclaimer

The game loop and input boilerplate was taken from [ArcadeRS](http://jadpole.github.io/arcaders/arcaders-1-0) by jadpole. Much <3 for such a great series.
