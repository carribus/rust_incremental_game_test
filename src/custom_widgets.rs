use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Color};
use tui::widgets::{Widget, Block, Borders};

///
/// tui-rs Label widget shamelessly borrowed (stolen) from the tui-rs examples folder as a starting point
/// 
/// # Example
/// 
/// ```
/// terminal.draw(|mut f| {
///     let size = f.size();
///     Label::default().text("Test").render(&mut f, size);
/// })?;
/// ```
pub struct Label<'a> {
    text: &'a str,
}

impl<'a> Default for Label<'a> {
    fn default() -> Self {
        Label { text: "" }
    }
}

impl<'a> Widget for Label<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.text, Style::default())
    }
}

impl<'a> Label<'a> {
    pub fn text(&mut self, text: &'a str) -> &mut Self {
        self.text = text;
        self
    }
}

///
/// A 'Button' widget for the tui-rs crate
/// 
/// Supports mouse events, focus events, click/press events etc...
pub struct Button<'a> {
    style: Style,
    text: &'a str,
}

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        Button { 
            style: Style::default(),
            text: "",
        }
    }
}

impl<'a> Widget for Button<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let text_area = { 
            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_style(self.style)
                .style(self.style);

            block.draw(area, buf);
            block.inner(area)
        };

        if text_area.height < 1 { return };

        buf.set_string(text_area.left(), text_area.top(), format!("{:^width$}", self.text, width=text_area.width as usize), self.style);
    }
}

impl<'a> Button<'a> {
    pub fn text(&mut self, text: &'a str) -> &mut Self {
        self.text = text;
        self
    }

    pub fn style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }
}