```
$ cargo run --example sdl2 --features backend
```

```
$ cargo test --features backend
```

### What is this?

Some sort of opinionated minimal implementation of flexbox in Rust.

### What is this not?

Spec compliant layout engine suitable, well, for basically anything.

### How does it work?

Given a tree:

```rust
    let root = Renderable::View(View::new(Style::new().with_bg(RGB::new(0, 0, 0)).with_flex_direction(FlexDirection::Column), vec![
        Renderable::View(View::new(Style::new().with_width(50).with_height(100).with_bg(RGB::new(255, 0, 0)), vec![])),
        Renderable::View(View::new(Style::new().with_width(50).with_height(100).with_bg(RGB::new(0, 255, 0)), vec![
            Renderable::View(View::new(Style::new().with_width(15).with_height(50).with_bg(RGB::new(0, 125, 125)), vec![])),
            Renderable::View(View::new(Style::new().with_width(15).with_height(50).with_bg(RGB::new(125, 125, 0)), vec![])),
        ])),
    ]));
```

It outputs a `Vec<Layout>`

```rust
    Layout { bg: RGB { r: 0, g: 0, b: 0 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 0, top: 0, width: 800, height: 600 } }
    Layout { bg: RGB { r: 255, g: 0, b: 0 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 0, top: 0, width: 50, height: 100 } }
    Layout { bg: RGB { r: 0, g: 255, b: 0 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 0, top: 100, width: 50, height: 100 } }
    Layout { bg: RGB { r: 0, g: 125, b: 125 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 0, top: 100, width: 15, height: 50 } }
    Layout { bg: RGB { r: 125, g: 125, b: 0 }, fg: RGB { r: 0, g: 0, b: 0 }, rect: Rect { left: 15, top: 100, width: 15, height: 50 } }
```

Suitable for input into a user-supplied `Renderer` implementing the `Render` trait. A simple SDL2 renderer
is in the backend folder and used in the sdl2 example.

![flexbox-rs sdl2 example](https://cloud.githubusercontent.com/assets/1541631/15799928/f30fad30-2a21-11e6-8da0-020f4ddfdebb.png)
