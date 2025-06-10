use iced::{
    Length, Rectangle, Size,
    advanced::{
        self, Layout, Widget,
        layout::{Limits, Node},
        mouse::Cursor,
        renderer::Style,
        widget::Tree,
    },
};

pub struct ImageGrid {}

impl ImageGrid {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ImageGrid
where
    Renderer: advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {}

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        todo!()
    }
}
