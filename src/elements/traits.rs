use super::{Element, RenderState};
use crate::context::RenderContext;
use nexus::imgui::Ui;

/// Interface for [`Element`] tree nodes.
pub trait Node {
    /// Performs necessary loads.
    fn load(&mut self) {
        for child in self.children() {
            child.load();
        }
    }

    /// Returns the child [`Elements`].
    fn children(&mut self) -> &mut [Element];
}

/// Interface for UI elements that can be rendered.
pub trait Render {
    /// Renders the UI element.
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState);

    /// Renders options for the UI element.
    fn render_options(&mut self, ui: &Ui);
}
