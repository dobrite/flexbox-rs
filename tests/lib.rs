
extern crate flexbox;

use flexbox::*;

#[test]
fn it_sets_root_width_to_width_with_default_height_no_children() {
    let width = 800u32;
    let height = 0u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    assert!(layout.len() == 1);
    assert_eq!(800, layout[0].rect.width);
}

#[test]
fn it_sets_root_height_to_0_with_default_height_no_children() {
    let width = 800u32;
    let height = 0u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    assert!(layout.len() == 1);
    assert_eq!(0, layout[0].rect.height);
}

#[test]
fn it_sets_root_width_to_800_with_no_children() {
    let width = 800u32;
    let height = 100u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    assert!(layout.len() == 1);
    assert_eq!(800, layout[0].rect.width);
}

#[test]
fn it_sets_root_height_to_100_with_no_children() {
    let width = 800u32;
    let height = 100u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    assert!(layout.len() == 1);
    assert_eq!(100, layout[0].rect.height);
}
