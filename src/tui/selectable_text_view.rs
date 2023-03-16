use std::borrow::Borrow;
use std::path::{Path, PathBuf};
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

use crate::file_analysis::file_types::Byteable;
use crate::tui::{color_for_size, show};

/* todo this is really at least 2 structs, one for actual fs entries and one for meta entries like more and <other files...>
eg page and page_size might only be necessary for more; comment and size for fs entries */
pub(crate) struct SelectableTextView {
    inner_view: Layer<LinearLayout>,
    selectable: bool,
    color: Color,
    path: PathBuf,
    page_size: u8,
    page: usize,
    hide_comments: bool,
    show_hidden: bool,
}

impl SelectableTextView {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        path: &Path, name: String, comment: String, size: Option<&Byteable>, mut style: Style, selectable: bool,
        page_size: u8, page: usize, hide_comments: bool, show_hidden: bool,
    ) -> Self {
        let mut name_view = TextView::new(name);
        let mut size_view = TextView::new(size.map_or("".to_string(), |v| v.to_string())).h_align(HAlign::Right);
        let color = if let Some(size) = size { color_for_size(size.0) } else { Color::Rgb(255, 255, 255) };

        if !selectable {
            style = style.combine(Effect::Dim);
        }

        let style = style.combine(color);
        name_view.set_style(style);
        size_view.set_style(color);
        let mut linear_layout =
            LinearLayout::horizontal().child(name_view.with_name("").full_width()).child(DummyView.fixed_width(1));

        if !hide_comments {
            linear_layout = linear_layout
                .child(TextView::new(comment).with_name("comment").fixed_width(45))
                .child(DummyView.fixed_width(1));
        }

        linear_layout = linear_layout.child(size_view.with_name("").fixed_width(10));
        let inner_view = Layer::new(linear_layout);
        Self { inner_view, selectable, color, path: path.to_path_buf(), page_size, page, hide_comments, show_hidden }
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
        let hide_comments = self.hide_comments;
        let show_hidden = self.show_hidden;
        Box::new({
            move |siv: &mut Cursive| {
                show(page_size, page, hide_comments, show_hidden, &path, siv);
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
                #[cfg(target_os = "windows")]
                {
                    let mut command = Command::new("explorer");
                    let command = command.arg(self.path.clone());
                    command.output().expect("Error opening in terminal/explorer");
                }

                #[cfg(not(target_os = "windows"))]
                {
                    let mut command = Command::new("gnome-terminal");
                    let command =
                        command.arg("--window").arg(format!("--working-directory={}", self.path.clone().display()));
                    command.output().expect("Error opening in terminal/explorer");
                }

                EventResult::Consumed(None)
            }
            Event::Mouse { event: MouseEvent::Release(MouseButton::Left), .. } if self.selectable => {
                EventResult::with_cb(self.get_callback())
            }
            _ => EventResult::Ignored,
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

// todo: unit tests?
