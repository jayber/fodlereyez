use std::path::PathBuf;

use cursive::align::HAlign;
use cursive::event::Key;
use cursive::theme::{BorderStyle, Color, Effect, Palette, Style, Theme};
use cursive::view::Resizable;
use cursive::views::{DummyView, LinearLayout, ResizedView, ScrollView, TextView};
use cursive::With;

use color::convert_file_size_to_color;
use selectable_text_view::SelectableTextView;

use crate::file_analysis::file_objects::{DirectoryEntry, DirectoryTree};

mod color;
mod selectable_text_view;

pub(crate) fn display_result(directory_tree_root: DirectoryTree) {
    let mut siv = cursive::default();
    siv.set_theme(build_theme());
    siv.add_layer(build_views(&directory_tree_root, None));
    siv.set_user_data(directory_tree_root);
    siv.add_global_callback(Key::Esc, |siv| siv.quit());
    siv.run();
}

pub(crate) fn build_views(
    directory_tree: &DirectoryTree, current_directory: Option<PathBuf>
) -> ResizedView<LinearLayout> {
    let root_layout =
        LinearLayout::vertical().child(TextView::new(format!("{}, size: {}", directory_tree.name, directory_tree.len)));

    let mut name_column = LinearLayout::vertical();
    let col_padding = LinearLayout::vertical().fixed_width(1);
    let mut size_column = LinearLayout::vertical();

    add_back_entry(directory_tree, &current_directory, &mut name_column, &mut size_column);

    for branch in directory_tree.entries.iter() {
        let (name_view, size_view, color) = create_view_entry(branch);
        let name_view = SelectableTextView::new(name_view, branch.has_children(), color, branch.get_path());
        name_column.add_child(name_view);
        size_column.add_child(size_view);
    }
    let mut columns = LinearLayout::horizontal();
    columns = columns.child(name_column).child(col_padding).child(size_column);
    root_layout.child(ScrollView::new(columns)).full_screen()
}

fn add_back_entry(
    directory_tree: &DirectoryTree, current_directory: &Option<PathBuf>, name_column: &mut LinearLayout,
    size_column: &mut LinearLayout
) {
    if let Some(dir) = &current_directory {
        if let Some(name) = dir.to_str() {
            if name != directory_tree.name {
                let parent_option = dir.parent().map(|p| p.to_path_buf());
                if let Some(parent) = parent_option {
                    name_column.add_child(SelectableTextView::new(
                        TextView::new("⮬.."),
                        true,
                        Color::Rgb(255, 255, 255),
                        parent
                    ));
                    size_column.add_child(DummyView);
                }
            }
        }
    }
}

fn create_view_entry(branch: &DirectoryEntry) -> (TextView, TextView, Color) {
    let mut name = TextView::new(branch.name());
    let mut size = TextView::new(branch.len().to_string()).h_align(HAlign::Right);
    let mut style = Style::from(match branch {
        DirectoryEntry::Folder { .. } => Style::from(Effect::Simple),
        DirectoryEntry::File { .. } => Style::from(Effect::Italic),
        DirectoryEntry::Rollup { .. } => Style::from(Effect::Italic)
    });

    if !branch.has_children() {
        style = style.combine(Effect::Dim);
    }
    let color = color_for_size(branch.len().val);
    let style = style.combine(color);
    name.set_style(style);
    size.set_style(color);
    (name, size, color)
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

    // use crate::tui::build_views;

    #[test]
    #[ignore]
    fn test_build_views() {
        let _tree = DirectoryTree::new(String::from("view"));
        // let _views = build_views(tree);
        todo!()
    }
}
