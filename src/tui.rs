use cursive::With;
use cursive::theme::{BorderStyle, Color, Palette, Theme};
use cursive::views::{LinearLayout, TextView};

use folder_size::analysis::DirectoryTree;

pub(crate) fn view(result: DirectoryTree) {
    let mut siv = cursive::default();
    siv.set_theme(build_theme());

    let view = build_views(result);
    siv.add_layer(view);

    siv.run();
}

fn build_views(result: DirectoryTree) -> LinearLayout {
    let mut layout = LinearLayout::vertical()
        .child(TextView::new(format!("{}, size: {}", result.name.to_str().unwrap(), result.len)));

    for child in result.children {
        let mut view = TextView::new(format!("\t{}\t\t {}", child.name.components().last().unwrap().as_os_str().to_str().unwrap(), child.len));
        view.set_style(Color::Rgb(0xff, 0x50, 0x50));
        // view.layout(Vec2::new())
        layout = layout.child(view);
    }
    layout
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
