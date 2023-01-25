use cursive::align::HAlign;
use cursive::theme::{BorderStyle, Color, Effect, Palette, Style, Theme};
use cursive::view::Resizable;
use cursive::views::{LinearLayout, ResizedView, ScrollView, TextView};
use cursive::With;

use crate::color::*;
use crate::file_analysis::file_objects::{DirectoryEntry, DirectoryTree};

pub(crate) fn view(result: DirectoryTree) {
    let mut siv = cursive::default();
    siv.set_theme(build_theme());
    siv.add_layer(build_views(result));
    siv.run();
}

fn build_views(result: DirectoryTree) -> ResizedView<LinearLayout> {
    let mut columns = LinearLayout::horizontal();
    let root_layout = LinearLayout::vertical().child(TextView::new(format!("{}, size: {}", result.name, result.len)));

    let col_padding = LinearLayout::vertical().fixed_width(2);
    let mut col1 = LinearLayout::vertical();
    let col_padding2 = LinearLayout::vertical().fixed_width(1);
    let mut col2 = LinearLayout::vertical();
    let iter = result.entries.iter();
    for child in iter {
        let mut name = TextView::new(child.name());
        let mut size = TextView::new(child.len().to_string()).h_align(HAlign::Right);
        let style = Style::from(match child {
            DirectoryEntry::Folder(_) => Style::from(Effect::Simple),
            DirectoryEntry::File { .. } => Style::from(Effect::Italic).combine(Effect::Dim),
            DirectoryEntry::Rollup { .. } => Style::from(Effect::Italic)
        });
        let color = color_for_size(child.len().val);
        let style = style.combine(color);
        name.set_style(style);
        size.set_style(color);
        col1 = col1.child(name);
        col2 = col2.child(size);
    }
    columns = columns.child(col_padding).child(col1).child(col_padding2).child(col2);
    root_layout.child(ScrollView::new(columns)).full_screen()
}

fn color_for_size(size: u64) -> Color {
    let (r, g, b) = convert_file_size_to_color(size);
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
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::file_analysis::file_objects::DirectoryTree;
    use crate::tui::build_views;

    #[test]
    #[ignore]
    fn test_build_views() {
        let tree = DirectoryTree::new(String::from("view"));
        let _views = build_views(tree);
        todo!()
    }
}
