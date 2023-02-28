use std::borrow::Borrow;
use std::path::PathBuf;
use std::process::Command;

use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key, MouseButton, MouseEvent};
use cursive::theme::Color::TerminalDefault;
use cursive::theme::{Color, ColorStyle, ColorType, Effect, Style};
use cursive::traits::{Finder, Resizable};
use cursive::view::{CannotFocus, Nameable, Selector};
use cursive::views::{DummyView, Layer, LinearLayout, TextView};
use cursive::{Cursive, Printer, Vec2, View};

use crate::file_analysis::file_types::{Byteable, DirectoryEntry};
use crate::tui::{build_views, color_for_size};

/* todo this is really at least 2 structs, one for actual fs entries and one for meta entries like more and <other files...>
eg page and page_size might only be necessary for more; comment and size for fs entries */
pub(crate) struct SelectableTextView {
    inner_view: Layer<LinearLayout>,
    selectable: bool,
    color: Color,
    path: PathBuf,
    page_size: u8,
    page: usize
}

impl SelectableTextView {
    pub(crate) fn new(
        path: PathBuf, name: String, comment: String, size: Option<&Byteable>, mut style: Style, selectable: bool,
        page_size: u8, page: usize
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
            .child(DummyView.fixed_width(1))
            .child(comment_view.with_name("comment").fixed_width(45))
            .child(DummyView.fixed_width(1))
            .child(size_view.with_name("").fixed_width(10));
        let mut inner_view = Layer::new(linear_layout);
        inner_view.set_color(ColorStyle::new(color, TerminalDefault));
        Self { inner_view, selectable, color, path, page_size, page }
    }

    pub(crate) fn select_style(&mut self, select: bool) {
        let (front, back) = if select {
            (ColorType::Color(Color::Rgb(0, 0, 0)), ColorType::Color(self.color))
        } else {
            (ColorType::Color(self.color), ColorType::Color(TerminalDefault))
        };
        let color_style = ColorStyle::new(front, back);
        self.inner_view.set_color(color_style);
        self.inner_view.call_on_all::<TextView, _>(Selector::Name("").borrow(), |view: &mut TextView| {
            view.set_style(Style::from(color_style))
        });

        self.inner_view.call_on_all::<TextView, _>(Selector::Name("comment").borrow(), |view: &mut TextView| {
            view.set_style(Style::from(if select {
                ColorStyle::new(Color::Rgb(0, 0, 0), self.color)
            } else {
                ColorStyle::new(TerminalDefault, TerminalDefault)
            }))
        })
    }

    fn get_callback(&self) -> Box<dyn Fn(&mut Cursive)> {
        let path = self.path.clone();
        let page_size = self.page_size;
        let page = self.page;
        Box::new({
            move |siv: &mut Cursive| {
                if let Some(found_entry) = siv.user_data::<DirectoryEntry>().and_then(|entry| entry.find(&path)) {
                    if let Some(view) = build_views(found_entry, Some(path.clone()), page_size, page) {
                        siv.pop_layer();
                        siv.add_layer(view);
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
            Event::Key(Key::Enter) if self.selectable => EventResult::with_cb(self.get_callback()),
            Event::Char(' ') if self.selectable => {
                Command::new("explorer").arg(self.path.clone()).output().expect("Error opening in explorer");
                EventResult::Consumed(None)
            }
            Event::Mouse { event: MouseEvent::Release(MouseButton::Left), .. } if self.selectable => {
                EventResult::with_cb(self.get_callback())
            }
            _ => EventResult::Ignored
        }
    }

    fn take_focus(&mut self, _source: Direction) -> Result<EventResult, CannotFocus> {
        if self.selectable {
            self.select_style(true);
            Ok(EventResult::Consumed(None))
        } else {
            Err(CannotFocus)
        }
    }
}
