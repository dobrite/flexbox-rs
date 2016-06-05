
extern crate flexbox;

use flexbox::*;

fn debug_layout(layout: &[Command]) {
    for l in layout {
        println!("{:?}", l)
    }
}

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

#[test]
fn it_sets_single_child_dim() {
    let width = 800u32;
    let height = 600u32;
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Renderable::View(View::new(Style::new(),
                                          vec![
        Renderable::View(View::new(Style::new().with_height(child_height).with_width(child_width), vec![])),
    ]));
    let layout = layout(width, height, &root);
    let child = &layout[1];
    assert!(layout.len() == 2);
    assert_eq!(child_height, child.height());
    assert_eq!(child_width, child.width());
}

#[test]
fn it_sets_two_child_rect_row() {
    let width = 800u32;
    let height = 600u32;
    let children_width = 50u32;
    let children_height = 100u32;

    let root = Renderable::View(View::new(Style::new(),
                                          vec![
        Renderable::View(View::new(Style::new().with_height(children_height).with_width(children_width), vec![])),
        Renderable::View(View::new(Style::new().with_height(children_height).with_width(children_width), vec![])),
    ]));

    let layout = layout(width, height, &root);
    assert!(layout.len() == 3);

    let mut child = &layout[1];
    assert_eq!(children_height, child.height());
    assert_eq!(children_width, child.width());
    assert_eq!(0, child.top());
    assert_eq!(0, child.left());

    child = &layout[2];
    assert_eq!(children_height, child.height());
    assert_eq!(children_width, child.width());
    assert_eq!(0, child.top());
    assert_eq!(50, child.left());
}

#[test]
fn it_sets_two_child_rect_column() {
    let width = 800u32;
    let height = 600u32;
    let children_width = 50u32;
    let children_height = 100u32;

    let root = Renderable::View(View::new(Style::new().with_flex_direction(FlexDirection::Column),
                                          vec![
        Renderable::View(View::new(Style::new().with_height(children_height).with_width(children_width), vec![])),
        Renderable::View(View::new(Style::new().with_height(children_height).with_width(children_width), vec![])),
    ]));

    let layout = layout(width, height, &root);
    assert!(layout.len() == 3);

    let mut child = &layout[1];
    assert_eq!(children_height, child.height());
    assert_eq!(children_width, child.width());
    assert_eq!(0, child.top());
    assert_eq!(0, child.left());

    child = &layout[2];
    assert_eq!(children_height, child.height());
    assert_eq!(children_width, child.width());
    assert_eq!(100, child.top());
    assert_eq!(0, child.left());
}

#[test]
fn it_sets_two_child_two_child_rect_column() {
    let width = 800u32;
    let height = 600u32;

    let root = Renderable::View(View::new(Style::new().with_flex_direction(FlexDirection::Column),
                                          vec![
        Renderable::View(View::new(Style::new().with_height(100).with_width(50), vec![])),
        Renderable::View(View::new(Style::new().with_height(100).with_width(50), vec![
            Renderable::View(View::new(Style::new().with_height(25).with_width(15), vec![])),
            Renderable::View(View::new(Style::new().with_height(25).with_width(15), vec![])),
        ])),
    ]));


    let layout = layout(width, height, &root);
    debug_layout(&layout);
    assert!(layout.len() == 5);

    let mut child = &layout[1];
    assert_eq!(100, child.height());
    assert_eq!(50, child.width());
    assert_eq!(0, child.top());
    assert_eq!(0, child.left());

    child = &layout[2];
    assert_eq!(100, child.height());
    assert_eq!(50, child.width());
    assert_eq!(100, child.top());
    assert_eq!(0, child.left());

    let mut child = &layout[3];
    assert_eq!(25, child.height());
    assert_eq!(15, child.width());
    assert_eq!(100, child.top());
    assert_eq!(0, child.left());

    child = &layout[4];
    assert_eq!(25, child.height());
    assert_eq!(15, child.width());
    assert_eq!(100, child.top());
    assert_eq!(15, child.left());
}
