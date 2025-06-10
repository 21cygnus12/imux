use iced::{Element, widget};

mod image_grid;
use image_grid::ImageGrid;

#[derive(Debug)]
pub enum Message {}

#[derive(Default)]
pub struct App {
    image_grid: ImageGrid,
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {}
    }
    pub fn view(&self) -> Element<Message> {
        widget::image("ferris.png").into()
    }
}
