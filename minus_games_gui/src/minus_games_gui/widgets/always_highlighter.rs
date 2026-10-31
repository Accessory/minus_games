use iced::advanced::layout::Limits;
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Layout, Shell, mouse, overlay, renderer};
use iced::mouse::Cursor;
use iced::{Color, Element, Event, Length, Rectangle, Size, Vector};

pub struct AlwaysHighlighter<'a, Message, Theme, Renderer> {
    base: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> AlwaysHighlighter<'a, Message, Theme, Renderer> {
    pub fn new(
        base: Element<'a, Message, Theme, Renderer>,
    ) -> AlwaysHighlighter<'a, Message, Theme, Renderer> {
        Self { base }
    }
}

// impl<'a, Message: 'a, Theme: 'a, Renderer> From<AlwaysHighlighter<'a, Message, Theme, Renderer>>
//     for Element<'a, Message, Theme, Renderer>
// where
//     Renderer: renderer::Renderer + 'a,
//     Theme: Catalog,
// {
//     fn from(item: AlwaysHighlighter<'a, Message, Theme, Renderer>) -> Self {
//         Self::new(item)
//     }
// }

trait Catalog {
    fn get_background_color(&self) -> Color;
}

impl Catalog for iced::Theme {
    fn get_background_color(&self) -> Color {
        self.palette().background.weak.color
    }
}

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Style {
//     pub background: Background,
// }

impl<'a, Message, Theme, Renderer> iced::advanced::widget::Meta
    for AlwaysHighlighter<'a, Message, Theme, Renderer>
{
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for AlwaysHighlighter<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    fn size(&self) -> Size<Length> {
        self.base.size()
    }

    fn tag(&self) -> tree::Tag {
        self.base.tag()
    }
    fn state(&self) -> tree::State {
        self.base.state()
    }

    fn diff(&mut self, tree: &mut Tree) {
        self.base.diff(tree);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) {
        self.base.layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let color = theme.get_background_color();
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            color,
        );
        self.base
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.base
            .operate(tree, layout, viewport, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.base
            .update(tree, event, layout, cursor, renderer, shell, viewport)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.base
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        size: Size,
    ) -> Vec<overlay::Element<'a, Message, Theme, Renderer>> {
        self.base
            .overlay(tree, layout, renderer, viewport, translation, size)
    }
}
