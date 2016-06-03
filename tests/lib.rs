
extern crate flexbox;

use flexbox::*;

#[test]
fn it_sets_root_width_to_width_with_default_height_no_children() {
    let width = 800u32;
    let height = 0u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    let root = &layout[0];
    assert!(layout.len() == 1);
    assert_eq!(800, root.width());
}

#[test]
fn it_sets_root_height_to_0_with_default_height_no_children() {
    let width = 800u32;
    let height = 0u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    let root = &layout[0];
    assert!(layout.len() == 1);
    assert_eq!(0, root.height());
}

#[test]
fn it_sets_root_bg_to_white_no_children() {
    let width = 800u32;
    let height = 600u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    let root = &layout[0];
    assert!(layout.len() == 1);
    assert_eq!(RGB::new(255, 255, 255), root.bg());
}

#[test]
fn it_sets_root_fg_to_black_no_children() {
    let width = 800u32;
    let height = 600u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    let root = &layout[0];
    assert!(layout.len() == 1);
    assert_eq!(RGB::new(0, 0, 0), root.fg());
}

#[test]
fn it_sets_root_width_to_800_with_no_children() {
    let width = 800u32;
    let height = 100u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    let root = &layout[0];
    assert!(layout.len() == 1);
    assert_eq!(800, root.width());
}

#[test]
fn it_sets_root_height_to_100_with_no_children() {
    let width = 800u32;
    let height = 100u32;
    let root = Renderable::View(View::new(Style::new(), vec![]));
    let layout = layout(width, height, &root);
    let root = &layout[0];
    assert!(layout.len() == 1);
    assert_eq!(100, root.height());
}

#[test]
fn it_sets_child_dim_with_no_child_height() {
    let width = 800u32;
    let height = 600u32;
    let child_width = 50u32;
    let root = Renderable::View(View::new(Style::new(),
                                          vec![
        Renderable::View(View::new(Style::new().with_width(child_width), vec![])),
    ]));
    let layout = layout(width, height, &root);
    let child = &layout[1];
    assert!(layout.len() == 2);
    assert_eq!(child_width, child.width());
    assert_eq!(height, child.height());
}

#[test]
fn it_sets_child_dim_with_no_child_width() {
    let width = 800u32;
    let height = 600u32;
    let child_height = 100u32;
    let root = Renderable::View(View::new(Style::new(),
                                          vec![
        Renderable::View(View::new(Style::new().with_height(child_height), vec![])),
    ]));
    let layout = layout(width, height, &root);
    let child = &layout[1];
    assert!(layout.len() == 2);
    assert_eq!(child_height, child.height());
    assert_eq!(0, child.width());
}
