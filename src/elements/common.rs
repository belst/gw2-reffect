use super::{Element, ElementType, RenderState};
use crate::{
    action::ChildElementAction,
    component_wise::ComponentWise,
    context::{Context, EditState},
    id::{Id, IdGen},
    render_util::{input_float_with_format, push_alpha_change, EnumStaticVariants},
    traits::RenderOptions,
};
use nexus::imgui::{InputTextFlags, MenuItem, Slider, SliderFlags, Ui};
use serde::{Deserialize, Serialize};

// FIXME: common default is called twice when deserializing element/pack, generating unused ids

/// Common behavior between elements and packs.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Common {
    #[serde(skip)]
    pub id: Id,

    pub name: String,

    pub pos: [f32; 2],

    pub opacity: f32,
}

impl Common {
    pub fn id_string(&self) -> String {
        self.id.to_string()
    }

    pub fn render(
        &self,
        ui: &Ui,
        ctx: &Context,
        state: &RenderState,
        contents: impl FnOnce(&RenderState),
    ) {
        let state = state.with_offset(self.pos).with_name(&self.name);
        {
            let _style = push_alpha_change(ui, self.opacity);
            contents(&state);
        }

        if ctx.edit.is_edited(self.id) {
            self.render_edit_indicator(ui, &state);
        }
    }

    fn render_edit_indicator(&self, ui: &Ui, state: &RenderState) {
        const SIZE: f32 = 5.0;
        const HALF_SIZE: f32 = 0.5 * SIZE;
        const OFFSET: [f32; 2] = [HALF_SIZE, HALF_SIZE];
        const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.8];

        let draw_list = ui.get_foreground_draw_list();

        let start = state.pos.sub(OFFSET);
        let end = state.pos.add(OFFSET);
        draw_list.add_rect(start, end, COLOR).filled(true).build();

        let pos = state.pos.add([HALF_SIZE + 1.0, 0.0]);
        draw_list.add_text(pos, COLOR, &self.name);
    }

    pub fn render_tree_label(&self, ui: &Ui, kind: &str) {
        ui.same_line();
        ui.text_disabled(kind);
        ui.same_line();
        ui.text(&self.name);
    }

    pub fn render_tree_children(
        &self,
        ui: &Ui,
        state: &mut EditState,
        children: &mut Vec<Element>,
    ) {
        let mut action = ChildElementAction::new();
        for (i, child) in children.iter_mut().enumerate() {
            action.or(i, child.render_select_tree(ui, state));
        }
        action.perform(state, children);
    }

    /// Renders common context menu options.
    pub fn render_context_menu(
        &mut self,
        ui: &Ui,
        state: &mut EditState,
        children: Option<&mut Vec<Element>>,
    ) {
        if let Some(children) = children {
            ui.menu("Create", || {
                for kind in ElementType::static_variants() {
                    let name = kind.as_ref();
                    if MenuItem::new(name).build(ui) {
                        let new = Element::of_type(kind.clone());
                        children.push(new);
                    }
                }
            });
            if MenuItem::new("Paste")
                .enabled(state.has_clipboard())
                .build(ui)
            {
                children.push(state.take_clipboard().expect("paste without clipboard"))
            }
        }
    }

    pub fn render_debug(&mut self, ui: &Ui) {
        ui.text("Id:");
        ui.same_line();
        ui.text_disabled(self.id_string());
    }
}

impl RenderOptions for Common {
    fn render_options(&mut self, ui: &Ui) {
        ui.input_text("Name", &mut self.name).build();

        let [x, y] = &mut self.pos;
        input_float_with_format("Position x", x, 1.0, 10.0, "%.2f", InputTextFlags::empty());
        input_float_with_format("Position y", y, 1.0, 10.0, "%.2f", InputTextFlags::empty());

        let mut opacity = 100.0 * self.opacity;
        if Slider::new("Opacity", 0.0, 100.0)
            .flags(SliderFlags::ALWAYS_CLAMP)
            .display_format("%.2f")
            .build(ui, &mut opacity)
        {
            self.opacity = opacity / 100.0;
        }
    }
}

impl Default for Common {
    fn default() -> Self {
        Self {
            id: IdGen::generate(),
            name: "Unnamed".into(),
            pos: [0.0, 0.0],
            opacity: 1.0,
        }
    }
}

impl Clone for Common {
    fn clone(&self) -> Self {
        Self {
            id: IdGen::generate(), // we want a fresh id for the clone
            name: self.name.clone(),
            pos: self.pos,
            opacity: self.opacity,
        }
    }
}

/// Helper to generate options render.
macro_rules! render_or_children {
    ( $self:ident, $ui:expr, $state:expr ) => {
        if $state.is_selected($self.common.id) {
            $self.render_options($ui);
            true
        } else if let Some(children) = $self.children() {
            children
                .iter_mut()
                .any(|child| child.try_render_options($ui, $state))
        } else {
            false
        }
    };
}

pub(crate) use render_or_children;
