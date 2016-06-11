
extern crate flexbox;

use flexbox::*;

fn debug_layout(layout: &[Command]) {
    for l in layout {
        println!("{:?}", l)
    }
}

struct MockMeasure;

impl Measure for MockMeasure {
    fn get_dim(&self, s: &str) -> Dim {
        Dim::new(s.as_bytes().len() as u32, 0)
    }
}

#[test]
fn it_sets_root_width_to_width_with_default_height_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(0), vec![
        Renderable::View(View::new(Style::new(), vec![]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(2, layout.len());
    assert_eq!(800, root.width());
}

#[test]
fn it_sets_root_height_to_0_with_default_height_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(0), vec![
        Renderable::View(View::new(Style::new(), vec![]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(2, layout.len());
    assert_eq!(0, root.height());
}

#[test]
fn it_sets_root_bg_to_white_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(600), vec![
        Renderable::View(View::new(Style::new(), vec![]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(2, layout.len());
    assert_eq!(RGB::new(255, 255, 255), root.bg());
}

#[test]
fn it_sets_root_fg_to_black_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(600), vec![
        Renderable::View(View::new(Style::new(), vec![]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(2, layout.len());
    assert_eq!(RGB::new(0, 0, 0), root.fg());
}

#[test]
fn it_sets_root_width_to_800_with_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(100), vec![
        Renderable::View(View::new(Style::new(), vec![]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(2, layout.len());
    assert_eq!(800, root.width());
}

#[test]
fn it_sets_root_height_to_100_with_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(100), vec![
        Renderable::View(View::new(Style::new(), vec![]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(2, layout.len());
    assert_eq!(100, root.height());
}

#[test]
fn it_sets_child_dim_with_no_child_height() {
    let child_width = 50u32;
    let root = Root::new(Style::new().with_width(800).with_height(600), vec![
        Renderable::View(View::new(Style::new(), vec![
            Renderable::View(View::new(Style::new().with_width(child_width), vec![])),
        ]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let child = &layout[2];
    assert_eq!(3, layout.len());
    assert_eq!(child_width, child.width());
    assert_eq!(600, child.height());
}

#[test]
fn it_sets_child_dim_with_no_child_width() {
    let child_height = 100u32;
    let root = Root::new(Style::new().with_width(800).with_height(600), vec![
        Renderable::View(View::new(Style::new(), vec![
            Renderable::View(View::new(Style::new().with_height(child_height), vec![])),
        ]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let child = &layout[2];
    assert_eq!(3, layout.len());
    assert_eq!(child_height, child.height());
    assert_eq!(0, child.width());
}

#[test]
fn it_sets_single_child_dim() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new().with_width(800).with_height(600), vec![
        Renderable::View(View::new(Style::new(), vec![
            Renderable::View(View::new(Style::new().with_height(child_height).with_width(child_width), vec![])),
        ]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let child = &layout[2];
    assert_eq!(3, layout.len());
    assert_eq!(child_height, child.height());
    assert_eq!(child_width, child.width());
}

#[test]
fn it_sets_two_child_rect_row() {
    let children_width = 50u32;
    let children_height = 100u32;

    let root = Root::new(Style::new().with_width(800).with_height(600), vec![
        Renderable::View(View::new(Style::new(), vec![
            Renderable::View(View::new(Style::new().with_height(children_height).with_width(children_width), vec![])),
            Renderable::View(View::new(Style::new().with_height(children_height).with_width(children_width), vec![])),
        ]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(4, layout.len());

    let mut child = &layout[2];
    assert_eq!(children_height, child.height());
    assert_eq!(children_width, child.width());
    assert_eq!(0, child.top());
    assert_eq!(0, child.left());

    child = &layout[3];
    assert_eq!(children_height, child.height());
    assert_eq!(children_width, child.width());
    assert_eq!(0, child.top());
    assert_eq!(50, child.left());
}

#[test]
fn it_sets_two_child_rect_column() {
    let children_width = 50u32;
    let children_height = 100u32;

    let root = Root::new(Style::new().with_width(800).with_height(600), vec![
        Renderable::View(View::new(Style::new().with_flex_direction(FlexDirection::Column), vec![
            Renderable::View(View::new(Style::new().with_height(children_height).with_width(children_width), vec![])),
            Renderable::View(View::new(Style::new().with_height(children_height).with_width(children_width), vec![])),
        ]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(4, layout.len());

    let mut child = &layout[2];
    assert_eq!(children_height, child.height());
    assert_eq!(children_width, child.width());
    assert_eq!(0, child.top());
    assert_eq!(0, child.left());

    child = &layout[3];
    assert_eq!(children_height, child.height());
    assert_eq!(children_width, child.width());
    assert_eq!(100, child.top());
    assert_eq!(0, child.left());
}

#[test]
fn it_sets_two_child_two_child_rect_column() {
    let root = Root::new(Style::new().with_width(800).with_height(600), vec![
        Renderable::View(View::new(Style::new().with_flex_direction(FlexDirection::Column), vec![
            Renderable::View(View::new(Style::new().with_height(100).with_width(50), vec![])),
            Renderable::View(View::new(Style::new().with_height(100).with_width(50), vec![
                Renderable::View(View::new(Style::new().with_height(25).with_width(15), vec![])),
                Renderable::View(View::new(Style::new().with_height(25).with_width(15), vec![])),
            ])),
        ]))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(6, layout.len());

    let mut child = &layout[2];
    assert_eq!(100, child.height());
    assert_eq!(50, child.width());
    assert_eq!(0, child.top());
    assert_eq!(0, child.left());

    child = &layout[3];
    assert_eq!(100, child.height());
    assert_eq!(50, child.width());
    assert_eq!(100, child.top());
    assert_eq!(0, child.left());

    let mut child = &layout[4];
    assert_eq!(25, child.height());
    assert_eq!(15, child.width());
    assert_eq!(100, child.top());
    assert_eq!(0, child.left());

    child = &layout[5];
    assert_eq!(25, child.height());
    assert_eq!(15, child.width());
    assert_eq!(100, child.top());
    assert_eq!(15, child.left());
}

#[test]
fn it_sets_root_width_to_width_text_with_default_height_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(0), vec![
        Renderable::Text(Text::new(Style::new(), "blah"))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(2, layout.len());
    let root = &layout[1];
    assert_eq!(4, root.width());
}

#[test]
fn it_sets_root_height_to_0_text_with_0_height_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(0), vec![
        Renderable::Text(Text::new(Style::new(), "blah"))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(2, layout.len());
    let root = &layout[1];
    assert_eq!(0, root.height());
}

#[test]
fn it_sets_root_width_to_double_width_text_with_two_texts() {
    let root = Root::new(Style::new().with_width(800).with_height(0), vec![
        Renderable::Text(Text::new(Style::new(), "blah")),
        Renderable::Text(Text::new(Style::new(), "blah"))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(3, layout.len());
    let left = &layout[1];
    assert_eq!(4, left.width());
    let right = &layout[2];
    assert_eq!(4, left.width());
}
