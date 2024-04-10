use gpui::*;
use gaa_cupertino::base::window::GaaWindow;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        // Register the `quit` function so it can be referenced by the `MenuItem::action` in the menu bar
        cx.on_action(quit);
        cx.set_menus(vec![Menu {
            name: "set_menus",
            items: vec![MenuItem::action("Quit", Quit)],
        }]);
       
        let mut window = GaaWindow::new();
        window.set_title("Hello, world!");
        cx.open_window(window.get_window_options(), |cx| {
            cx.new_view(|_cx| window)
        });
    });
}

actions!(set_menus, [Quit]);

fn quit(_: &Quit, cx: &mut AppContext) {
    println!("Gracefully quitting the application . . .");
    cx.quit();
}

