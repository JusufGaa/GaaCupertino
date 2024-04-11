use gpui::*;

#[derive(Clone)]
pub struct GaaWindow {
    title: String,
    background: u32,
    color: u32,
}

impl Render for GaaWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.observe_window_activation(|_, cx| {
            if cx.is_window_active() {
                return;
            };
            cx.quit();
        }).detach();
        div()
            .flex()
            .bg(rgb(self.background))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(self.color))
            .child(format!("Hello, world!"))
    }
}

impl GaaWindow {
    pub fn new() -> Self {
        GaaWindow {
            title: "".to_string(),
            background: 0xffffff,
            color: 0x000000,
        }
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn get_window_options(&self) -> WindowOptions {
        WindowOptions {
            titlebar: Some(self.get_title_bar_options()),
            ..Default::default()
        }
    }

    fn get_title_bar_options(&self) -> TitlebarOptions {
        TitlebarOptions {
            title: Some((self.title).clone().into()),
            ..Default::default()
        }
    }

    pub fn load(mut self, cx: &mut AppContext) {
        let options = self.get_window_options();
        cx.open_window(options, |cx| {
            cx.new_view(|_cx| self)
        });
    }
}