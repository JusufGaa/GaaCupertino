use gpui::*;
use gaa_cupertino::base::window::GaaWindow;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            let window = GaaWindow::new();
            cx.new_view(|_cx| window)
        });
    });
}


