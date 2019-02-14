extern crate backend;
extern crate bot;
extern crate gomoku;

extern crate azul;

use gomoku::gomoku_app::*;

use azul::prelude::*;

const BOARD_IMG: &[u8] = include_bytes!("../assets/images/board.jpg");

fn main() {
    let mut app = App::new(GomokuApp::new(), AppConfig::default());

    app.add_image("board", &mut BOARD_IMG, ImageType::Jpeg).unwrap();

    let style_sheet = css::hot_reload_override_native(
        "./debug/hot_css.css",
        std::time::Duration::from_millis(500)
    );

    #[cfg(debug_assertions)]
    let window = Window::new_hot_reload(
        WindowCreateOptions::default(),
        style_sheet
    ).unwrap();

    #[cfg(not(debug_assertions))]
    let window = Window::new(WindowCreateOptions::default(), style_sheet).unwrap();

    app.run(window).unwrap();
}
