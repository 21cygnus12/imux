use iced::{Element, widget};

#[derive(Debug)]
pub enum Message {}

#[derive(Default)]
pub struct App {}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {}
    }
    pub fn view(&self) -> Element<Message> {
        widget::image("ferris.png").into()
    }
}
