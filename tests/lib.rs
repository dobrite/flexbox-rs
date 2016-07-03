
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
        Dim::new(s.as_bytes().len() as u32, 4)
    }
}

#[test]
fn it_sets_root_width_to_width_with_default_height_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(0), vec![]);
    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(1, layout.len());
    assert_eq!(800, root.width());
}

#[test]
fn it_sets_root_height_to_0_with_default_height_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(0), vec![]);
    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(1, layout.len());
    assert_eq!(0, root.height());
}

#[test]
fn it_sets_root_fg_to_black_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(600), vec![]);
    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(1, layout.len());
    assert_eq!(RGB::new(0, 0, 0), root.fg());
}

#[test]
fn it_sets_root_width_to_800_with_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(100), vec![]);
    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(1, layout.len());
    assert_eq!(800, root.width());
}

#[test]
fn it_sets_root_height_to_100_with_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(100), vec![]);
    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let root = &layout[0];
    assert_eq!(1, layout.len());
    assert_eq!(100, root.height());
}

#[test]
fn it_sets_child_dim_with_no_child_height() {
    let child_width = 50u32;
    let root = Root::new(Style::new().with_width(800).with_height(600),
                         vec![
        Renderable::View(View::new(Style::new().with_width(child_width), vec![])),
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let child = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(child_width, child.width());
    assert_eq!(600, child.height());
}

#[test]
fn it_sets_child_dim_with_no_child_width() {
    let child_height = 100u32;
    let root = Root::new(Style::new().with_width(800).with_height(600),
                         vec![
        Renderable::View(View::new(Style::new().with_height(child_height), vec![])),
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let child = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(child_height, child.height());
    assert_eq!(0, child.width());
}

#[test]
fn it_sets_single_child_dim() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new().with_width(800).with_height(600),
                         vec![
        Renderable::View(View::new(Style::new().with_height(child_height)
                                   .with_width(child_width), vec![])),
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let child = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(child_height, child.height());
    assert_eq!(child_width, child.width());
}

#[test]
fn it_sets_two_child_rect_row() {
    let children_width = 50u32;
    let children_height = 100u32;

    let root = Root::new(Style::new().with_width(800).with_height(600),
                         vec![
        Renderable::View(View::new(Style::new().with_height(children_height)
                                   .with_width(children_width), vec![])),
        Renderable::View(View::new(Style::new().with_height(children_height)
                                   .with_width(children_width), vec![])),
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(3, layout.len());

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
    let children_width = 50u32;
    let children_height = 100u32;

    let root = Root::new(Style::new()
                             .with_width(800)
                             .with_height(600)
                             .with_flex_direction(FlexDirection::Column),
                         vec![
        Renderable::View(View::new(Style::new().with_height(children_height)
                                   .with_width(children_width), vec![])),
        Renderable::View(View::new(Style::new().with_height(children_height)
                                   .with_width(children_width), vec![])),
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(3, layout.len());

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
    let root = Root::new(Style::new()
                             .with_width(800)
                             .with_height(600)
                             .with_flex_direction(FlexDirection::Column),
                         vec![
        Renderable::View(View::new(Style::new().with_height(100).with_width(50), vec![])),
        Renderable::View(View::new(Style::new().with_height(100).with_width(50), vec![
            Renderable::View(View::new(Style::new().with_height(25).with_width(15), vec![])),
            Renderable::View(View::new(Style::new().with_height(25).with_width(15), vec![])),
        ])),
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(5, layout.len());

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

#[test]
fn it_sets_root_width_to_width_text_with_default_height_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(0),
                         vec![Renderable::Text(Text::new(Style::new(), "blah"))]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(2, layout.len());
    let root = &layout[1];
    assert_eq!(4, root.width());
}

#[test]
fn it_sets_root_height_to_0_text_with_0_height_no_children() {
    let root = Root::new(Style::new().with_width(800).with_height(0),
                         vec![Renderable::Text(Text::new(Style::new(), "blah"))]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(2, layout.len());
    let root = &layout[0];
    assert_eq!(0, root.height());
}

#[test]
fn it_sets_root_width_to_double_width_text_with_two_texts_flex_direction_row() {
    let root = Root::new(Style::new().with_width(800).with_height(0),
                         vec![Renderable::Text(Text::new(Style::new(), "blah")),
                              Renderable::Text(Text::new(Style::new(), "blah"))]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(3, layout.len());
    let left = &layout[1];
    assert_eq!(4, left.width());
    let right = &layout[2];
    assert_eq!(4, right.rect.left);
    assert_eq!(4, right.width());
}

#[test]
fn it_sets_root_width_to_double_width_text_with_two_texts_flex_direction_column() {
    let root = Root::new(Style::new()
                             .with_width(800)
                             .with_height(0)
                             .with_flex_direction(FlexDirection::Column),
                         vec![Renderable::Text(Text::new(Style::new(), "blah")),
                              Renderable::Text(Text::new(Style::new(), "blah"))]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(3, layout.len());
    let top = &layout[1];
    assert_eq!(4, top.width());
    let bottom = &layout[2];
    assert_eq!(4, bottom.rect.top);
    assert_eq!(4, bottom.width());
}

#[test]
fn it_sets_root_background_color_to_transparent() {
    let root = Root::new(Style::new().with_width(1).with_height(0), vec![]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(1, layout.len());
    let root = &layout[0];
    assert_eq!(None, root.bg);
}

#[test]
fn it_sets_text_fg_to_container_fg_when_text_fg_is_None() {
    let root = Root::new(Style::new().with_width(20).with_height(1).with_fg(RGB::new(1, 1, 1)),
                         vec![Renderable::Text(Text::new(Style::new(), "blah"))]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(2, layout.len());
    let text = &layout[1];
    assert_eq!(RGB::new(1, 1, 1), text.fg);
}

#[test]
fn it_sets_text_bg_to_container_bg_when_text_bg_is_None() {
    let root = Root::new(Style::new()
                             .with_width(20)
                             .with_height(1)
                             .with_bg(BackgroundColor::Color(RGB::new(1, 1, 1))),
                         vec![Renderable::Text(Text::new(Style::new(), "blah"))]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(2, layout.len());
    let text = &layout[1];
    assert_eq!(Some(RGB::new(1, 1, 1)), text.bg);
}

#[test]
fn it_sets_text_fg() {
    let root = Root::new(Style::new().with_width(20).with_height(1).with_fg(RGB::new(0, 0, 0)),
                         vec![Renderable::Text(Text::new(Style::new()
                                                             .with_fg(RGB::new(255, 255, 255)),
                                                         "blah"))]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(2, layout.len());
    let text = &layout[1];
    assert_eq!(RGB::new(255, 255, 255), text.fg);
}

#[test]
fn it_sets_text_bg() {
    let root = Root::new(Style::new()
                             .with_width(20)
                             .with_height(1)
                             .with_bg(BackgroundColor::Color(RGB::new(0, 0, 0))),
                         vec![
        Renderable::Text(Text::new(Style::new()
                                   .with_bg(BackgroundColor::Color(
                                           RGB::new(255, 255, 255))), "blah"))
    ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    assert_eq!(2, layout.len());
    let text = &layout[1];
    assert_eq!(Some(RGB::new(255, 255, 255)), text.bg);
}

#[test]
fn it_sets_position_fixed_single_child() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new().with_width(800).with_height(600),
                         vec![Renderable::View(View::new(Style::new()
                                                             .with_height(child_height)
                                                             .with_width(child_width)
                                                             .with_position(Position::Fixed),
                                                         vec![]))]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let child = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(0, child.rect.top);
    assert_eq!(0, child.rect.left);
    assert_eq!(child_height, child.height());
    assert_eq!(child_width, child.width());
}

#[test]
fn it_sets_position_fixed_child_static_child() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new()
                             .with_width(800)
                             .with_height(600),
                         vec![
       Renderable::View(View::new(Style::new()
                                  .with_bg(BackgroundColor::Color(RGB::new(255, 255, 255)))
                                  .with_height(child_height)
                                  .with_width(child_width)
                                  .with_position(Position::Fixed), vec![])),
       Renderable::View(View::new(Style::new()
                                  .with_bg(BackgroundColor::Color(RGB::new(255, 255, 255)))
                                  .with_height(child_height)
                                  .with_width(child_width), vec![]))
   ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let fixed = &layout[1];
    let stati = &layout[2];
    assert_eq!(3, layout.len());
    assert_eq!(child_height, fixed.height());
    assert_eq!(child_width, fixed.width());
    assert_eq!(0, fixed.rect.top);
    assert_eq!(0, fixed.rect.left);
    assert_eq!(child_height, stati.height());
    assert_eq!(child_width, stati.width());
    assert_eq!(0, stati.rect.top);
    assert_eq!(0, stati.rect.left);
}

#[test]
fn it_sets_position_fixed_child_top() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new()
                             .with_width(800)
                             .with_height(600),
                         vec![
       Renderable::View(View::new(Style::new()
                                  .with_bg(BackgroundColor::Color(RGB::new(255, 255, 255)))
                                  .with_height(child_height)
                                  .with_width(child_width)
                                  .with_top(20)
                                  .with_position(Position::Fixed), vec![])),
   ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let fixed = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(child_height, fixed.height());
    assert_eq!(child_width, fixed.width());
    assert_eq!(20, fixed.rect.top);
    assert_eq!(0, fixed.rect.left);
}

#[test]
fn it_sets_position_fixed_child_bottom_600() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new()
                             .with_width(800)
                             .with_height(600),
                         vec![
       Renderable::View(View::new(Style::new()
                                  .with_bg(BackgroundColor::Color(RGB::new(255, 255, 255)))
                                  .with_height(child_height)
                                  .with_width(child_width)
                                  .with_bottom(20)
                                  .with_position(Position::Fixed), vec![])),
   ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let fixed = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(child_height, fixed.height());
    assert_eq!(child_width, fixed.width());
    assert_eq!(480, fixed.rect.top);
    assert_eq!(0, fixed.rect.left);
}

#[test]
fn it_sets_position_fixed_child_bottom_1000() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new()
                             .with_width(800)
                             .with_height(1000),
                         vec![
      Renderable::View(View::new(Style::new()
                                 .with_bg(BackgroundColor::Color(RGB::new(255, 255, 255)))
                                 .with_height(child_height)
                                 .with_width(child_width)
                                 .with_bottom(20)
                                 .with_position(Position::Fixed), vec![])),
  ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let fixed = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(child_height, fixed.height());
    assert_eq!(child_width, fixed.width());
    assert_eq!(880, fixed.rect.top);
    assert_eq!(0, fixed.rect.left);
}

// with missing child width should be 0
// with missing child height should be 0

#[test]
fn it_sets_position_fixed_child_left() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new()
                             .with_width(800)
                             .with_height(600),
                         vec![
       Renderable::View(View::new(Style::new()
                                  .with_bg(BackgroundColor::Color(RGB::new(255, 255, 255)))
                                  .with_height(child_height)
                                  .with_width(child_width)
                                  .with_left(20)
                                  .with_position(Position::Fixed), vec![])),
   ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let fixed = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(child_height, fixed.height());
    assert_eq!(child_width, fixed.width());
    assert_eq!(0, fixed.rect.top);
    assert_eq!(20, fixed.rect.left);
}

#[test]
fn it_sets_position_fixed_child_right_600() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new()
                             .with_width(600)
                             .with_height(800),
                         vec![
       Renderable::View(View::new(Style::new()
                                  .with_bg(BackgroundColor::Color(RGB::new(255, 255, 255)))
                                  .with_height(child_height)
                                  .with_width(child_width)
                                  .with_right(20)
                                  .with_position(Position::Fixed), vec![])),
   ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let fixed = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(child_height, fixed.height());
    assert_eq!(child_width, fixed.width());
    assert_eq!(0, fixed.rect.top);
    assert_eq!(530, fixed.rect.left);
}

#[test]
fn it_sets_position_fixed_child_right_1000() {
    let child_width = 50u32;
    let child_height = 100u32;
    let root = Root::new(Style::new()
                             .with_width(1000)
                             .with_height(800),
                         vec![
     Renderable::View(View::new(Style::new()
                                .with_bg(BackgroundColor::Color(RGB::new(255, 255, 255)))
                                .with_height(child_height)
                                .with_width(child_width)
                                .with_right(20)
                                .with_position(Position::Fixed), vec![])),
 ]);

    let mm = MockMeasure;
    let l = Layout::new(&mm);
    let layout = l.layout(&root);
    let fixed = &layout[1];
    assert_eq!(2, layout.len());
    assert_eq!(child_height, fixed.height());
    assert_eq!(child_width, fixed.width());
    assert_eq!(0, fixed.rect.top);
    assert_eq!(930, fixed.rect.left);
}
