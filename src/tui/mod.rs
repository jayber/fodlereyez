use std::path::{Path, PathBuf};

use cursive::event::{Event, Key};
use cursive::theme::BaseColor::Magenta;
use cursive::theme::{BorderStyle, Color, ColorStyle, Effect, Palette, Style, Theme};
use cursive::view::Resizable;
use cursive::views::{LinearLayout, OnEventView, ResizedView, ScrollView, TextView};
use cursive::{Cursive, With};

use color::convert_file_size_to_color;
use selectable_text_view::SelectableTextView;

use crate::file_analysis::file_types::DirectoryEntry;
use crate::tui::patterns::PATTERNS;

mod color;
mod patterns;
mod selectable_text_view;

pub(crate) fn display_result(root_entry: DirectoryEntry, page_size: u8, hide_comments: bool, show_hidden: bool) {
    let mut siv = cursive::default();
    siv.set_theme(build_theme());
    if let Some(view) = build_views(&root_entry, page_size, 0, true, hide_comments, show_hidden) {
        siv.add_layer(view);
        siv.set_user_data(root_entry);
        siv.add_global_callback(Key::Esc, |siv| siv.quit());
        siv.run();
    }
}

pub(crate) fn build_views(
    directory_entry: &DirectoryEntry, page_size: u8, page: usize, is_root: bool, hide_comments: bool, show_hidden: bool,
) -> Option<ResizedView<LinearLayout>> {
    directory_entry.entries().map(|entries| {
        let root_layout = create_root_layout(directory_entry);

        let entries_layout =
            create_entries_layout(directory_entry, page_size, page, is_root, hide_comments, show_hidden, entries);

        let event_view =
            register_event_listeners(directory_entry, page_size, page, hide_comments, show_hidden, entries_layout);

        root_layout.child(event_view).full_screen()
    })
}

fn register_event_listeners(
    directory_entry: &DirectoryEntry, page_size: u8, page: usize, hide_comments: bool, show_hidden: bool,
    entries_layout: LinearLayout,
) -> OnEventView<ScrollView<LinearLayout>> {
    let view = OnEventView::new(ScrollView::new(entries_layout));

    let path = directory_entry.path().to_path_buf();
    let path2 = path.clone();

    view.on_event(Event::Char('c'), move |siv| {
        show(page_size, page, !hide_comments, show_hidden, &path, siv);
    })
    .on_event(Event::Char('s'), move |siv| {
        show(page_size, page, hide_comments, !show_hidden, &path2, siv);
    })
}

fn show(page_size: u8, page: usize, hide_comments: bool, show_hidden: bool, path: &PathBuf, siv: &mut Cursive) {
    if let Some(found_entry) = siv.user_data::<DirectoryEntry>().and_then(|entry| entry.find(path)) {
        if let Some(view) = build_views(found_entry, page_size, page, found_entry.is_root(), hide_comments, show_hidden)
        {
            siv.pop_layer();
            siv.add_layer(view);
        }
    }
}

fn create_entries_layout(
    directory_entry: &DirectoryEntry, page_size: u8, page: usize, is_root: bool, hide_comments: bool,
    show_hidden: bool, entries: &Vec<DirectoryEntry>,
) -> LinearLayout {
    let mut entries_layout = LinearLayout::vertical();
    if !is_root {
        if let Some(back) = create_back_entry(directory_entry, page_size, hide_comments, show_hidden) {
            entries_layout.add_child(back)
        }
    }

    let mut count = 0;
    for branch in entries.iter() {
        if count >= page_size as usize * (page + 1) {
            entries_layout.add_child(create_more_entry(
                directory_entry.path(),
                page_size,
                page,
                hide_comments,
                show_hidden,
            ));
            break;
        }
        if !branch.is_hidden() || show_hidden {
            entries_layout.add_child(create_view_entry(branch, page_size, hide_comments, show_hidden));
            count += 1;
        }
    }
    entries_layout
}

fn create_root_layout(directory_entry: &DirectoryEntry) -> LinearLayout {
    LinearLayout::vertical()
        .child(
            TextView::new("navigate: [→←↑↓], open: [Enter], open in FileExplorer: [Space], toggle [c]omments, [s]how hidden, exit: [Esc]")
                .style(Style::from(ColorStyle::front(Magenta))),
        )
        .child(TextView::new(format!("{}, size: {}", directory_entry.path().display(), directory_entry.len())))
}

fn create_more_entry(
    path: &Path, page_size: u8, page: usize, hide_comments: bool, show_hidden: bool,
) -> SelectableTextView {
    SelectableTextView::new(
        path,
        "⮯ more…".to_string(),
        String::new(),
        None,
        Style::from(Effect::Simple),
        true,
        page_size,
        page + 1,
        hide_comments,
        show_hidden,
    )
}

fn create_back_entry(
    directory_tree: &DirectoryEntry, page_size: u8, hide_comments: bool, show_hidden: bool,
) -> Option<SelectableTextView> {
    directory_tree.get_parent().map(|path| {
        SelectableTextView::new(
            path,
            "⮬..".to_string(),
            String::new(),
            None,
            Style::from(Effect::Simple),
            true,
            page_size,
            0,
            hide_comments,
            show_hidden,
        )
    })
}

fn create_view_entry(
    branch: &DirectoryEntry, page_size: u8, hide_comments: bool, show_hidden: bool,
) -> SelectableTextView {
    SelectableTextView::new(
        branch.path(),
        branch.name(),
        get_comment_for_entry(branch),
        Some(branch.len()),
        match branch {
            DirectoryEntry::Folder { .. } => Style::from(Effect::Simple),
            DirectoryEntry::File { .. } => Style::from(Effect::Italic),
            DirectoryEntry::Rollup { .. } => Style::from(Effect::Italic),
        },
        match branch {
            DirectoryEntry::File { .. } => false,
            DirectoryEntry::Folder { .. } => branch.has_children(),
            DirectoryEntry::Rollup { .. } => false, // todo this is just "in the meantime"
        },
        page_size,
        0,
        hide_comments,
        show_hidden,
    )
}

fn get_comment_for_entry(branch: &DirectoryEntry) -> String {
    let path = match branch {
        DirectoryEntry::File { path, .. } => path.display().to_string(),
        DirectoryEntry::Folder { path, .. } => path.display().to_string(),
        DirectoryEntry::Rollup { .. } => String::from(""),
    };
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
        }),
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
