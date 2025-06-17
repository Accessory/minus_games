use iced::advanced::layout::{Limits, Node};
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Layout, Shell, mouse, renderer};
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

impl<'a, Message: 'a, Theme: 'a, Renderer> From<AlwaysHighlighter<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Theme: Catalog,
{
    fn from(item: AlwaysHighlighter<'a, Message, Theme, Renderer>) -> Self {
        Self::new(item)
    }
}

trait Catalog {
    fn get_background_color(&self) -> Color;
}

impl Catalog for iced::Theme {
    fn get_background_color(&self) -> Color {
        self.extended_palette().background.weak.color
    }
}

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Style {
//     pub background: Background,
// }

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for AlwaysHighlighter<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    fn size(&self) -> Size<Length> {
        self.base.as_widget().size()
    }
    fn size_hint(&self) -> Size<Length> {
        self.base.as_widget().size_hint()
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.base.as_widget().layout(tree, renderer, limits)
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
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
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn tag(&self) -> tree::Tag {
        self.base.as_widget().tag()
    }

    fn state(&self) -> tree::State {
        self.base.as_widget().state()
    }

    fn children(&self) -> Vec<Tree> {
        self.base.as_widget().children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.base.as_widget().diff(tree);
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.base
            .as_widget()
            .operate(state, layout, renderer, operation);
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.base.as_widget_mut().update(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.base
            .as_widget()
            .mouse_interaction(state, layout, cursor, viewport, renderer)
    }

    fn overlay<'a>(
        &'a mut self,
        state: &'a mut Tree,
        layout: Layout<'a>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'a, Message, Theme, Renderer>> {
        self.base
            .as_widget_mut()
            .overlay(state, layout, renderer, viewport, translation)
    }
}
