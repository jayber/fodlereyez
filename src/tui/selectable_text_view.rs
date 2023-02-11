use std::path::PathBuf;

use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key, MouseButton, MouseEvent};
use cursive::theme::{Color, ColorStyle, Style};
use cursive::view::CannotFocus;
use cursive::views::TextView;
use cursive::{Cursive, Printer, Vec2, View};

use crate::file_analysis::file_objects::DirectoryTree;

pub(crate) struct SelectableTextView {
    text_view: TextView,
    is_selectable: bool,
    color: Color,
    path: PathBuf
}

impl SelectableTextView {
    pub(crate) fn new(text_view: TextView, is_selectable: bool, color: Color, path: PathBuf) -> Self {
        Self { text_view, is_selectable, color, path }
    }
    pub(crate) fn set_style(&mut self, style: Style) { self.text_view.set_style(style) }
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
    fn draw(&self, printer: &Printer) { self.text_view.draw(printer); }
    fn layout(&mut self, size: Vec2) { self.text_view.layout(size); }
    fn required_size(&mut self, constraint: Vec2) -> Vec2 { self.text_view.required_size(constraint) }
    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::FocusLost => {
                self.set_style(Style::from(ColorStyle::new(self.color, Color::Rgb(0, 0, 0))));
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
            self.set_style(Style::from(ColorStyle::new(Color::Rgb(0, 0, 0), self.color)));
            Ok(EventResult::Consumed(None))
        } else {
            Err(CannotFocus)
        }
    }
}
