use std::borrow::Borrow;
use std::path::PathBuf;

use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key, MouseButton, MouseEvent};
use cursive::theme::Color::TerminalDefault;
use cursive::theme::{Color, ColorStyle, ColorType, Effect, Style};
use cursive::traits::{Finder, Resizable};
use cursive::view::{CannotFocus, Nameable, Selector};
use cursive::views::{DummyView, Layer, LinearLayout, TextView};
use cursive::{Cursive, Printer, Vec2, View};

use crate::file_analysis::file_types::{Byteable, DirectoryTree};
use crate::tui::color_for_size;

pub(crate) struct SelectableTextView {
    inner_view: Layer<LinearLayout>,
    is_selectable: bool,
    color: Color,
    path: PathBuf
}

impl SelectableTextView {
    pub(crate) fn new(
        path: PathBuf, name: String, comment: String, size: Option<&Byteable>, mut style: Style, selectable: bool
    ) -> Self {
        let mut name_view = TextView::new(name);
        let comment_view = TextView::new(comment);
        let mut size_view = TextView::new(size.map_or("".to_string(), |v| v.to_string())).h_align(HAlign::Right);
        let color = if let Some(size) = size { color_for_size(size.val) } else { Color::Rgb(255, 255, 255) };

        if !selectable {
            style = style.combine(Effect::Dim);
        }

        let style = style.combine(color);
        name_view.set_style(style);
        size_view.set_style(color);
        let linear_layout = LinearLayout::horizontal()
            .child(name_view.with_name("").full_width())
            .child(comment_view.with_name("comment").fixed_width(30))
            .child(DummyView.fixed_width(1))
            .child(size_view.with_name("").fixed_width(10));
        let mut inner_view = Layer::new(linear_layout);
        inner_view.set_color(ColorStyle::new(color, TerminalDefault));
        Self { inner_view, is_selectable: selectable, color, path }
    }

    pub(crate) fn select_style(&mut self, select: bool) {
        let mut color_style = self.inner_view.color().invert();
        if select {
            color_style.front = ColorType::Color(Color::Rgb(0, 0, 0));
        } else {
            color_style.back = ColorType::Color(TerminalDefault);
        }
        self.inner_view.set_color(color_style);
        self.inner_view.call_on_all::<TextView, _>(Selector::Name("").borrow(), |view: &mut TextView| {
            view.set_style(Style::from(color_style))
        });
        let comment_color_style = if select {
            ColorStyle::new(Color::Rgb(0, 0, 0), self.color)
        } else {
            ColorStyle::new(Color::Rgb(255, 255, 255), TerminalDefault)
        };
        self.inner_view.call_on_all::<TextView, _>(Selector::Name("comment").borrow(), |view: &mut TextView| {
            view.set_style(Style::from(comment_color_style))
        })
    }

    fn get_callback(&self) -> Box<dyn Fn(&mut Cursive)> {
        let path = self.path.clone();
        Box::new({
            move |siv: &mut Cursive| {
                siv.pop_layer();
                if let Some(tree) = siv.user_data::<DirectoryTree>() {
                    match tree.find(&path) {
                        Some(branch) => {
                            let view = crate::tui::build_views(branch, Some(path.clone()));
                            siv.add_layer(view);
                        }
                        None => ()
                    }
                }
            }
        })
    }
}

impl View for SelectableTextView {
    fn draw(&self, printer: &Printer) { self.inner_view.draw(printer); }
    fn layout(&mut self, size: Vec2) { self.inner_view.layout(size); }
    fn required_size(&mut self, constraint: Vec2) -> Vec2 { self.inner_view.required_size(constraint) }
    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::FocusLost => {
                self.select_style(false);
                EventResult::Consumed(None)
            }
            Event::Key(Key::Enter) if self.is_selectable => EventResult::with_cb(self.get_callback()),
            Event::Char(' ') if self.is_selectable => EventResult::with_cb(self.get_callback()),
            Event::Mouse { event: MouseEvent::Release(MouseButton::Left), .. } if self.is_selectable => {
                EventResult::with_cb(self.get_callback())
            }
            _ => EventResult::Ignored
        }
    }

    fn take_focus(&mut self, _source: Direction) -> Result<EventResult, CannotFocus> {
        if self.is_selectable {
            self.select_style(true);
            Ok(EventResult::Consumed(None))
        } else {
            Err(CannotFocus)
        }
    }
}
