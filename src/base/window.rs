use gpui::*;

#[derive(Clone)]
pub struct GaaWindow {
    title: String,
    background: u32,
    color: u32,
    hidden: bool,
}

impl Render for GaaWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let result = div()
            .flex()
            .bg(rgb(self.background))
            .size_full();
        let body = div()
            .top(Pixels(8.0))
            .left(Pixels(8.0))
            .bg(rgb(0x00ff00))
            .text_xl()
            .text_color(rgb(self.color))
            .child(self.title.clone());
        result.child(body)
    }
}

impl GaaWindow {
    pub fn new() -> Self {
        GaaWindow {
            title: "".to_string(),
            background: 0xffffff,
            color: 0x000000,
            hidden: true,
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
        self.hidden = false;
        cx.open_window(options, |cx: &mut WindowContext| {
            if !cx.is_window_active() {
                self.hidden = true;
            };
            cx.new_view(|_cx| self)
        });
    }

    pub fn open(&mut self, cx: &mut AppContext) {
        if !self.hidden { return; }
        cx.active_window();
        self.hidden = false;
    }
}