use cursive::align::HAlign;
use cursive::theme::{BorderStyle, Color, Palette, Theme};
use cursive::view::Resizable;
use cursive::views::{LinearLayout, ResizedView, TextView};
use cursive::With;

use crate::color::hsv_to_rgb;
use crate::file_analysis::DirectoryTree;

pub(crate) fn view(result: DirectoryTree) {
    let mut siv = cursive::default();
    siv.set_theme(build_theme());
    siv.add_layer(build_views(result));
    siv.run();
}

fn build_views(result: DirectoryTree) -> ResizedView<LinearLayout> {
    let mut rows = LinearLayout::horizontal();
    let top_layout = LinearLayout::vertical().child(TextView::new(format!(
        "{}, size: {}",
        result.name.to_str().unwrap(),
        result.len
    )));

    let padding = LinearLayout::vertical().fixed_width(2);
    let mut col1 = LinearLayout::vertical();
    let padding2 = LinearLayout::vertical().fixed_width(1);
    let mut col2 = LinearLayout::vertical();
    for child in result.children {
        let mut name = TextView::new(
            child
                .name
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
        );
        let mut size = TextView::new(child.len.to_string()).h_align(HAlign::Right);
        let text_color = color_for_size(child.len.val);
        name.set_style(text_color);
        size.set_style(text_color);
        col1 = col1.child(name);
        col2 = col2.child(size);
    }
    rows = rows.child(padding).child(col1).child(padding2).child(col2);
    top_layout.child(rows).full_screen()
}

fn color_for_size(size: u64) -> Color {
    let top = (1024_f64 * 1024_f64 * 1024_f64 * 10_f64) as f64;
    let bottom = 1024_f64;
    let range = top - bottom;
    let size = size as f64;
    let size_hue = size.max(bottom).min(top);
    let hue_scale = (range - (size_hue - bottom)) / range;

    let top_white = 1024_f64 * 1024_f64 * 50_f64;
    let saturation = ((size - bottom).max(0.0) / top_white).min(1.0);

    let top_black = 1024_f64 * 1024_f64 * 1024_f64 * 300_f64;
    let value = (1.0 - ((size - top).max(0.0).min(top_black) / top_black)).max(0.6);

    let (r, g, b) = hsv_to_rgb(hue_scale * 220.0, saturation, value);
    Color::Rgb(r, g, b)
}

fn build_theme() -> Theme {
    Theme {
        shadow: true,
        borders: BorderStyle::None,
        palette: Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            use cursive::theme::Color::TerminalDefault;
            use cursive::theme::PaletteColor::*;

            palette[Background] = TerminalDefault;
            palette[View] = TerminalDefault;
            palette[Primary] = White.dark();
            palette[TitlePrimary] = Blue.light();
            palette[Secondary] = Blue.light();
            palette[Highlight] = Blue.dark();
        }),
    }
}
