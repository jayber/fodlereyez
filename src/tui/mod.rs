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

pub(crate) fn display_result(directory_tree_root: DirectoryTree, page_size: u8) {
    let mut siv = cursive::default();
    siv.set_theme(build_theme());
    siv.add_layer(build_views(&directory_tree_root, None, page_size, 0));
    siv.set_user_data(directory_tree_root);
    siv.add_global_callback(Key::Esc, |siv| siv.quit());
    siv.run();
}

pub(crate) fn build_views(
    directory_tree: &DirectoryTree, current_directory: Option<PathBuf>, page_size: u8, page: usize
) -> ResizedView<LinearLayout> {
    let root_layout =
        LinearLayout::vertical().child(TextView::new(format!("{}, size: {}", directory_tree.name, directory_tree.len)));
    let mut entries_layout = LinearLayout::vertical();

    if let Some(back) = create_back_entry(directory_tree, current_directory.as_ref(), page_size) {
        entries_layout.add_child(back)
    }

    let enumerate = directory_tree.entries.iter().enumerate();
    for (count, branch) in enumerate {
        if count >= page_size as usize * (page + 1) {
            entries_layout.add_child(create_more_entry(&current_directory, page_size, page));
            break;
        }
        entries_layout.add_child(create_view_entry(branch, page_size));
    }
    root_layout.child(ScrollView::new(entries_layout)).full_screen()
}

fn create_more_entry(path: &Option<PathBuf>, page_size: u8, page: usize) -> SelectableTextView {
    SelectableTextView::new(
        path.as_ref().unwrap_or(&PathBuf::default()).clone(),
        "⮯ more…".to_string(),
        String::new(),
        None,
        Style::from(Effect::Simple),
        true,
        page_size,
        page + 1
    )
}

fn create_back_entry(
    directory_tree: &DirectoryTree, current_directory: Option<&PathBuf>, page_size: u8
) -> Option<SelectableTextView> {
    let directory = PathBuf::from(directory_tree.name.clone());
    current_directory
        .filter(|&path| path != &directory)
        .and_then(|path| path.parent())
        .map(|path| path.to_path_buf())
        .map(|parent| {
            SelectableTextView::new(
                parent,
                "⮬..".to_string(),
                String::new(),
                None,
                Style::from(Effect::Simple),
                true,
                page_size,
                0
            )
        })
}

fn create_view_entry(branch: &DirectoryEntry, page_size: u8) -> SelectableTextView {
    SelectableTextView::new(
        branch.get_path(),
        branch.name(),
        "This is a random comment.".to_string(),
        Some(branch.len()),
        match branch {
            DirectoryEntry::Folder { .. } => Style::from(Effect::Simple),
            DirectoryEntry::File { .. } => Style::from(Effect::Italic),
            DirectoryEntry::Rollup { .. } => Style::from(Effect::Italic)
        },
        match branch {
            DirectoryEntry::File { .. } => false,
            DirectoryEntry::Folder { .. } => true,
            DirectoryEntry::Rollup { .. } => false //this is just "in the meantime"
        },
        page_size,
        0
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
