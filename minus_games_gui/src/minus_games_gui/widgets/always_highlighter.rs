use iced::advanced::layout::{Limits, Node};
use iced::advanced::widget::{tree, Tree, Widget};
use iced::advanced::{renderer, Layout};
use iced::mouse::Cursor;
use iced::{Background, Color, Element, Length, Rectangle, Size};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub background: Background,
}

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
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.base
            .as_widget()
            .operate(state, layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        self.base.as_widget_mut().on_event(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        self.base
            .as_widget()
            .mouse_interaction(state, layout, cursor, viewport, renderer)
    }

    fn overlay<'a>(
        &'a mut self,
        state: &'a mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'a, Message, Theme, Renderer>> {
        self.base
            .as_widget_mut()
            .overlay(state, layout, renderer, translation)
    }
}
