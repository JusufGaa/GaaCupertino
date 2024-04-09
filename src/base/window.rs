use gpui::*;
pub struct GaaWindow {
    background: u32,
    color: u32,
}

impl Render for GaaWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(self.background))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(self.color))
            .child(format!("Hello!"))
    }
}
impl GaaWindow {
    pub fn new() -> Self {
        GaaWindow {
            background: 0xffffff,
            color: 0x000000,
        }
    }
}