use std::path::PathBuf;

use cursive::event::Key;
use cursive::theme::{BorderStyle, Color, Effect, Palette, Style, Theme};
use cursive::view::Resizable;
use cursive::views::{LinearLayout, ResizedView, ScrollView, TextView};
use cursive::With;

use color::convert_file_size_to_color;
use selectable_text_view::SelectableTextView;

use crate::file_analysis::file_types::DirectoryEntry;
use crate::tui::patterns::PATTERNS;

mod color;
mod patterns;
mod selectable_text_view;

pub(crate) fn display_result(root_entry: DirectoryEntry, page_size: u8) {
    let mut siv = cursive::default();
    siv.set_theme(build_theme());
    let option_view = build_views(&root_entry, None, page_size, 0);
    if let Some(view) = option_view {
        siv.add_layer(view);
        siv.set_user_data(root_entry);
        siv.add_global_callback(Key::Esc, |siv| siv.quit());
        siv.run();
    }
}

pub(crate) fn build_views(
    directory_entry: &DirectoryEntry, current_directory: Option<PathBuf>, page_size: u8, page: usize
) -> Option<ResizedView<LinearLayout>> {
    if let Some(entries) = directory_entry.entries() {
        let root_layout = LinearLayout::vertical().child(TextView::new(format!(
            "{}, size: {}",
            directory_entry.get_path_clone().display(),
            directory_entry.len()
        )));
        let mut entries_layout = LinearLayout::vertical();

        if let Some(back) = create_back_entry(directory_entry, page_size) {
            entries_layout.add_child(back)
        }

        for (count, branch) in entries.iter().enumerate() {
            if count >= page_size as usize * (page + 1) {
                entries_layout.add_child(create_more_entry(&current_directory, page_size, page));
                break;
            }
            entries_layout.add_child(create_view_entry(branch, page_size));
        }

        Some(root_layout.child(ScrollView::new(entries_layout)).full_screen())
    } else {
        None
    }
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

fn create_back_entry(directory_tree: &DirectoryEntry, page_size: u8) -> Option<SelectableTextView> {
    let directory = directory_tree.get_path_clone();
    directory.parent().map(|path| path.to_path_buf()).map(|parent| {
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
        branch.get_path_clone(),
        branch.name(),
        match_comment(branch),
        Some(branch.len()),
        match branch {
            DirectoryEntry::Folder { .. } => Style::from(Effect::Simple),
            DirectoryEntry::File { .. } => Style::from(Effect::Italic),
            DirectoryEntry::Rollup { .. } => Style::from(Effect::Italic)
        },
        match branch {
            DirectoryEntry::File { .. } => false,
            DirectoryEntry::Folder { .. } => branch.has_children(),
            DirectoryEntry::Rollup { .. } => false // todo this is just "in the meantime"
        },
        page_size,
        0
    )
}

//todo whaaaaat?! make better!
fn match_comment(branch: &DirectoryEntry) -> String {
    let path = match branch {
        DirectoryEntry::File { path, .. } => path.display().to_string(),
        DirectoryEntry::Folder { path, .. } => path.display().to_string(),
        DirectoryEntry::Rollup { .. } => String::from("")
    };
    // println!("path: {}", path);
    let mut comment = String::new();
    for (a_comment, regex_set) in PATTERNS.iter() {
        if let Ok(true) = regex_set.as_ref().map(|set| set.is_match(&path)) {
            comment += a_comment;
            comment += " ";
        }
    }
    comment
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
    // use crate::file_analysis::file_types::DirectoryTree;

    // use crate::tui::build_views;

    #[test]
    #[ignore]
    fn test_build_views() {
        // let _tree = DirectoryTree::new();
        // let _views = build_views(tree);
        todo!()
    }
}
