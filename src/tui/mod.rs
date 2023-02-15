use std::path::PathBuf;

use cursive::event::Key;
use cursive::theme::{BorderStyle, Color, Effect, Palette, Style, Theme};
use cursive::view::Resizable;
use cursive::views::{LinearLayout, ResizedView, ScrollView, TextView};
use cursive::With;

use color::convert_file_size_to_color;
use selectable_text_view::SelectableTextView;

use crate::file_analysis::file_types::{DirectoryEntry, DirectoryTree};

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
    let mut entries_layout = LinearLayout::vertical();

    if let Some(back) = create_back_entry(directory_tree, &current_directory) {
        entries_layout.add_child(back)
    }

    let mut enumerate = directory_tree.entries.iter().enumerate();
    while let Some((count, branch)) = enumerate.next() {
        if count > 10 {
            entries_layout.add_child(create_more_entry(&current_directory));
            break;
        }
        entries_layout.add_child(create_view_entry(branch));
    }

    root_layout.child(ScrollView::new(entries_layout)).full_screen()
}

fn create_more_entry(path: &Option<PathBuf>) -> SelectableTextView {
    SelectableTextView::new(
        path.as_ref().unwrap_or(&PathBuf::default()).clone(),
        "more...".to_string(),
        String::new(),
        None,
        Style::from(Effect::Simple),
        true
    )
}

fn create_back_entry(
    directory_tree: &DirectoryTree, current_directory: &Option<PathBuf>
) -> Option<SelectableTextView> {
    if let Some(dir) = &current_directory {
        if let Some(name) = dir.to_str() {
            if name != directory_tree.name {
                let parent_option = dir.parent().map(|p| p.to_path_buf());
                if let Some(parent) = parent_option {
                    return Some(SelectableTextView::new(
                        parent,
                        "⮬..".to_string(),
                        String::new(),
                        None,
                        Style::from(Effect::Simple),
                        true
                    ));
                }
            }
        }
    }
    None
}

fn create_view_entry(branch: &DirectoryEntry) -> SelectableTextView {
    SelectableTextView::new(
        branch.get_path(),
        branch.name(),
        "This is a random comment.".to_string(),
        Some(branch.len()),
        Style::from(match branch {
            DirectoryEntry::Folder { .. } => Style::from(Effect::Simple),
            DirectoryEntry::File { .. } => Style::from(Effect::Italic),
            DirectoryEntry::Rollup { .. } => Style::from(Effect::Italic)
        }),
        branch.has_children()
    )
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
    use crate::file_analysis::file_types::DirectoryTree;

    // use crate::tui::build_views;

    #[test]
    #[ignore]
    fn test_build_views() {
        let _tree = DirectoryTree::new(String::from("view"));
        // let _views = build_views(tree);
        todo!()
    }
}
